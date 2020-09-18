use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, TcpListener};

#[tokio::main]
async fn main() -> io::Result<()> {//implement a simple tcp server which can ehco my messages
    let t1 = tokio::spawn(async move {//processing code in my tcp server
        let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();
        let mut buf = vec![0;1024];

        let (mut socket, address) = listener.accept().await.unwrap();
        println!("The accepted address is {}", &address);
        loop {
            let read_res = socket.read(&mut buf);
            println!("read once");

            match read_res.await {
                Ok(0) => {
                    println!("server read nothing");
                    return;
                },
                Ok(n) => {
                    println!("server read {} bytes", n);
                    socket.write_all(&buf[..n]).await.unwrap();
                    return;
                }
                Err(_) => {
                    println!("server got an error");
                    return;
                }
            }
        }
    });

    let t2 = tokio::spawn(async move {//receive the returned message and print
        let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap();
        let (mut rd, mut wr) = socket.into_split();
        wr.write_all(b"message from PengYang!").await.unwrap();//send my message
        println!("am i here");
        let mut buffer = String::new();
        loop {
            match rd.read_to_string(&mut buffer).await {
                Ok(0) => {
                    println!("client read nothing");
                    return;
                },
                Ok(_n) => {
                    println!("client read {} bytes", _n);
                    println!("The received message's context is:{}", buffer);
                },
                Err(_) => {
                    println!("client got an error");
                    return;
                },
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    io::Result::Ok(())
}

// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
// use tokio::net::{TcpStream, TcpListener};

// #[tokio::main]
// async fn main() -> io::Result<()> {//implement a simple tcp server which can ehco my messages
//     let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();
//     let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap();
//     let (mut rd, mut wr) = socket.into_split();
//     wr.write_all(b"message from PengYang!").await;//send my message
//     let (mut socket, address) = listener.accept().await?;

//     let t1 = tokio::spawn(async move {//processing code in my tcp server
//         let mut buf = vec![0;1024];
//         println!("the address connected is: {}", address);
            
//         loop {
//             match socket.read(&mut buf).await {
//                 Ok(0) => return,
//                 Ok(n) => {
//                     if socket.write_all(&buf[..n]).await.is_err() {
//                         return;
//                     }
//                 }
//                 Err(_) => {
//                     return;
//                 }
//             }
//         }
//     });

//     let t2 = tokio::spawn(async move {//receive the returned message and print
//         let mut buffer = String::new();

//         loop {
//             match rd.read_to_string(&mut buffer).await {
//                 Ok(0) => return,
//                 Ok(_n) => println!("The received message's context is:{}", buffer),
//                 Err(_) => {
//                     println!("Error happened!");
//                     return;
//                 },
//             }
//         }
//     });
//     t1.await.unwrap();
//     t2.await.unwrap();
//     Ok::<_, io::Error>(())
// }