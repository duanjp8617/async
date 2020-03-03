mod try_read_write;
use try_read_write::{TryRead, TryWrite};

use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::net::{TcpListener, TcpStream};
use bytes::{ByteBuf, MutByteBuf};
use slab::Slab;
use std::io;
use std::net::SocketAddr;
use std::str::FromStr;

#[macro_use] extern crate log;

const SERVER: Token = Token(10_000_000);

struct EchoConn {
    sock: TcpStream,
    buf: Option<ByteBuf>,
    mut_buf: Option<MutByteBuf>,
    token: Option<Token>,
    interest: Ready
}

impl EchoConn {
    fn new(sock: TcpStream) -> EchoConn {
        EchoConn {
            sock,
            buf: None,
            mut_buf: Some(ByteBuf::mut_with_capacity(2048)),
            token: None,
            interest: Ready::empty(),
        }
    }

    fn writable(&mut self, poll: &mut Poll) -> io::Result<()> {
        let mut buf = self.buf.take().unwrap();

        match self.sock.try_write_buf(&mut buf) {
            Ok(None) => {
                debug!("client flushing buf; WOULDBLOCK");

                self.buf = Some(buf);
                self.interest.insert(Ready::writable());
            }
            Ok(Some(r)) => {
                debug!("CONN : we wrote {} bytes!", r);

                self.mut_buf = Some(buf.flip());

                self.interest.insert(Ready::readable());
                self.interest.remove(Ready::writable());
            }
            Err(e) => debug!("not implemented; client err={:?}", e),
        }

        assert!(self.interest.is_readable() || self.interest.is_writable(), "actual={:?}", self.interest);
        poll.reregister(&self.sock, self.token.unwrap(), self.interest,
                              PollOpt::edge() | PollOpt::oneshot())
    }

    fn readable(&mut self, poll: &mut Poll) -> io::Result<()> {
        let mut buf = self.mut_buf.take().unwrap();

        match self.sock.try_read_buf(&mut buf) {
            Ok(None) => {
                debug!("CONN : spurious read wakeup");
                self.mut_buf = Some(buf);
            }
            Ok(Some(r)) => {
                debug!("CONN : we read {} bytes!", r);

                // prepare to provide this to writable
                self.buf = Some(buf.flip());

                self.interest.remove(Ready::readable());
                self.interest.insert(Ready::writable());
            }
            Err(e) => {
                debug!("not implemented; client err={:?}", e);
                self.interest.remove(Ready::readable());
            }

        };

        assert!(self.interest.is_readable() || self.interest.is_writable(), "actual={:?}", self.interest);
        poll.reregister(&self.sock, self.token.unwrap(), self.interest,
                              PollOpt::edge())
    }
}

struct EchoServer {
    sock: TcpListener,
    conns: Slab<EchoConn>
}

impl EchoServer {
    fn accept(&mut self, poll: &mut Poll) -> io::Result<()> {
        debug!("server accepting socket");

        let sock = self.sock.accept().unwrap().0;
        let conn = EchoConn::new(sock,);
        let tok = self.conns.insert(conn);

        // Register the connection
        self.conns[tok].token = Some(Token(tok));
        poll.register(&self.conns[tok].sock, Token(tok), Ready::readable(),
                                PollOpt::edge() | PollOpt::oneshot())
            .expect("could not register socket with event loop");

        Ok(())
    }

    fn conn_readable(&mut self, poll: &mut Poll,
                     tok: Token) -> io::Result<()> {
        debug!("server conn readable; tok={:?}", tok);
        self.conn(tok).readable(poll)
    }

    fn conn_writable(&mut self, poll: &mut Poll,
                     tok: Token) -> io::Result<()> {
        debug!("server conn writable; tok={:?}", tok);
        self.conn(tok).writable(poll)
    }

    fn conn(&mut self, tok: Token) -> &mut EchoConn {
        &mut self.conns[tok.into()]
    }
}

fn main() {
    let mut poll = Poll::new().unwrap();

    // set up the server address
    let port_num = 10240;
    let s = format!("127.0.0.1:{}", port_num);        
    let srv_addr : SocketAddr = FromStr::from_str(&s).unwrap();

    // create a server sock
    let srv = TcpListener::bind(&srv_addr).unwrap();

    println!("listen for connections");
    poll.register(&srv, SERVER, Ready::readable(),
                            PollOpt::edge() | PollOpt::oneshot()).unwrap();

    // == Create storage for events
    let mut events = Events::with_capacity(1024);

    let mut echo_srv = EchoServer {
        sock : srv,
        conns: Slab::with_capacity(128),
    };

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            debug!("ready {:?} {:?}", event.token(), event.readiness());
            if event.readiness().is_readable() {
                match event.token() {
                    SERVER => echo_srv.accept(&mut poll).unwrap(),                    
                    i => echo_srv.conn_readable(&mut poll, i).unwrap()
                }
            }

            if event.readiness().is_writable() {
                match event.token() {
                    SERVER => panic!("received writable for token 0"),
                    i => echo_srv.conn_writable(&mut poll, i).unwrap()
                };
            }
        }
    }
}
