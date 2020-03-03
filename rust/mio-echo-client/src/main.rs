mod try_read_write;
use try_read_write::{TryRead, TryWrite};

use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::net::{TcpStream};
use bytes::{Buf, ByteBuf, MutByteBuf, SliceBuf};
use std::io;
use std::net::SocketAddr;
use std::str::FromStr;

#[macro_use] extern crate log;

const CLIENT: Token = Token(10_000_001);

struct EchoClient {
    sock: TcpStream,
    msgs: Vec<&'static str>,
    tx: SliceBuf<'static>,
    rx: SliceBuf<'static>,
    mut_buf: Option<MutByteBuf>,
    token: Token,
    interest: Ready,
    shutdown: bool,
}

// Sends a message and expects to receive the same exact message, one at a time
impl EchoClient {
    fn new(sock: TcpStream, token: Token,  mut msgs: Vec<&'static str>) -> EchoClient {
        let curr = msgs.remove(0);

        EchoClient {
            sock,
            msgs,
            tx: SliceBuf::wrap(curr.as_bytes()),
            rx: SliceBuf::wrap(curr.as_bytes()),
            mut_buf: Some(ByteBuf::mut_with_capacity(2048)),
            token,
            interest: Ready::empty(),
            shutdown: false,
        }
    }

    fn readable(&mut self, poll: &mut Poll) -> io::Result<()> {
        debug!("client socket readable");

        let mut buf = self.mut_buf.take().unwrap();

        match self.sock.try_read_buf(&mut buf) {
            Ok(None) => {
                debug!("CLIENT : spurious read wakeup");
                self.mut_buf = Some(buf);
            }
            Ok(Some(r)) => {
                println!("CLIENT : We read {} bytes!", r);

                // prepare for reading
                let mut buf = buf.flip();

                while buf.has_remaining() {
                    let actual = buf.read_byte().unwrap();
                    let expect = self.rx.read_byte().unwrap();

                    assert!(actual == expect, "actual={}; expect={}", actual, expect);
                }

                self.mut_buf = Some(buf.flip());

                self.interest.remove(Ready::readable());

                if !self.rx.has_remaining() {
                    self.next_msg(poll).unwrap();
                }
            }
            Err(e) => {
                panic!("not implemented; client err={:?}", e);
            }
        };

        if !self.interest.is_empty() {
            assert!(self.interest.is_readable() || self.interest.is_writable(), "actual={:?}", self.interest);
            poll.reregister(&self.sock, self.token, self.interest,
                            PollOpt::edge() | PollOpt::oneshot())?;
        }

        Ok(())
    }

    fn writable(&mut self, poll: &mut Poll) -> io::Result<()> {
        debug!("client socket writable");

        match self.sock.try_write_buf(&mut self.tx) {
            Ok(None) => {
                debug!("client flushing buf; WOULDBLOCK");
                self.interest.insert(Ready::writable());
            }
            Ok(Some(r)) => {
                debug!("CLIENT : we wrote {} bytes!", r);
                self.interest.insert(Ready::readable());
                self.interest.remove(Ready::writable());
            }
            Err(e) => debug!("not implemented; client err={:?}", e)
        }

        if self.interest.is_readable() || self.interest.is_writable() {
            poll.reregister(&self.sock, self.token, self.interest,
                            PollOpt::edge() | PollOpt::oneshot())?;
        }

        Ok(())
    }

    fn next_msg(&mut self, poll: &mut Poll) -> io::Result<()> {
        if self.msgs.is_empty() {
            self.shutdown = true;
            return Ok(());
        }

        let curr = self.msgs.remove(0);

        debug!("client prepping next message");
        self.tx = SliceBuf::wrap(curr.as_bytes());
        self.rx = SliceBuf::wrap(curr.as_bytes());

        self.interest.insert(Ready::writable());
        poll.reregister(&self.sock, self.token, self.interest,
                              PollOpt::edge() | PollOpt::oneshot())
    }
}

fn main() {
    let mut poll = Poll::new().unwrap();

    // set up the server address
    let port_num = 10240;
    let s = format!("127.0.0.1:{}", port_num);        
    let srv_addr : SocketAddr = FromStr::from_str(&s).unwrap();

    let sock = TcpStream::connect(&srv_addr).unwrap();

    // Connect to the server
    poll.register(&sock, CLIENT, Ready::writable(),
                        PollOpt::edge() | PollOpt::oneshot()).unwrap();
    // == Create storage for events
    let mut events = Events::with_capacity(1024);

    let mut echo_client = EchoClient::new(sock, CLIENT, vec!["foo", "bar"]);

    // Start the event loop
    while !echo_client.shutdown {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            debug!("ready {:?} {:?}", event.token(), event.readiness());
            if event.readiness().is_readable() {
                match event.token() {
                    CLIENT => echo_client.readable(&mut poll).unwrap(),
                    _ => panic!("impossible")
                }
            }

            if event.readiness().is_writable() {
                match event.token() {
                    CLIENT => echo_client.writable(&mut poll).unwrap(),
                    _ => panic!("impossible")
                };
            }
        }
    }

    println!("echo client quits")
}
