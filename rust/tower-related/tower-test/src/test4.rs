use tokio::sync::mpsc::channel;
use tokio::{time, task};

use http::{Request, Response, StatusCode};
use tower::Service;

use std::result::Result;
use std::task::{Poll, Context};

use futures_task::{FutureObj};

struct EchoService;

impl Service<Request<Vec<u8>>> for EchoService {
    type Response = Response<Vec<u8>>;
    type Error = http::Error;
    type Future = FutureObj<'static, Result<Response<Vec<u8>>, http::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Vec<u8>>) -> Self::Future {
        let mut resp_body = "echo body back: ".as_bytes().to_owned();
        resp_body.append(req.body_mut());
        
        let resp = Response::builder()
            .status(StatusCode::OK)
            .body(resp_body)
            .unwrap();

        FutureObj::new(Box::new(async move {
            time::delay_for(time::Duration::new(1, 0)).await;
            Ok(resp)
        }))
    }
}

#[tokio::main]
async fn main() {
    let iter_num : i32 = 5;

    let (mut req_sender, mut req_receiver) = channel::<Request<Vec<u8>>>(10);
    let (mut resp_sender, mut resp_receiver) = channel::<Response<Vec<u8>>>(10);

    let mock_client = async move {
        for i in 0..iter_num {
            let req = Request::builder()
                .uri("www.fuck.com")
                .header("fuck", "you")
                .body(format!("a polite message # {}", i).as_bytes().to_owned())
                .unwrap();
            
            req_sender.send(req).await.unwrap();
            
            let resp = resp_receiver.recv().await.unwrap();
            println!("response message is: {}", std::str::from_utf8(resp.body()).unwrap());
        }
    };

    let mock_server = async move {
        let mut echo = EchoService{};
        for _ in 0..iter_num {
            let req = req_receiver.recv().await.unwrap();
            let resp = echo.call(req).await.unwrap();
            resp_sender.send(resp).await.unwrap();
        }
    };

    let t1 = task::spawn(mock_client);
    let t2 = task::spawn(mock_server);

    t1.await.unwrap();
    t2.await.unwrap();
}