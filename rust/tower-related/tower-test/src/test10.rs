use tokio::sync::mpsc::channel;
use tokio::{time, task};

use http::{Request, Response, StatusCode};

use std::result::Result;
use std::task::{Poll, Context};
use std::future::Future;

use futures_task::{FutureObj};

pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;

    fn call(&mut self, req: Request) -> Self::Future;
}

// This is the return monad a -> M a
pub trait Layer<S> {
    type Service;
    fn layer(&self, inner: S) -> Self::Service;
}

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
            Ok(resp)
        }))
    }
}

/// Layer 1:

struct TimeoutLayer {
    duration: time::Duration,
}

impl<S> Layer<S> for TimeoutLayer 
{
    type Service = TimeoutService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TimeoutService {
            inner: inner,
            duration: self.duration,
        }
    }
}

struct TimeoutService<S> {
    inner: S,
    duration: time::Duration,
}

impl<S, Request> Service<Request> for TimeoutService<S> 
    where 
        S: Service<Request>,
        S::Future: Send + 'static
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = FutureObj<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let resp = self.inner.call(req);
        let duration_copy = self.duration;
        
        FutureObj::new(Box::new(async move {
            time::delay_for(duration_copy).await;
            resp.await
        }))
    }
}

/// Layer 2

struct LogLayer<'a> {
    log_str: &'a str,
}

impl<'a, S> Layer<S> for LogLayer<'a> 
{
    type Service = LogService<'a, S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogService {
            inner: inner,
            log_str: self.log_str
        }
    }
}

struct LogService<'a, S> {
    inner: S,
    log_str: &'a str,
}

impl<'a, S, Request> Service<Request> for LogService<'a, S> 
    where S: Service<Request>
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        println!("Service {} is processing an request", self.log_str);
        self.inner.call(req)
    }
}

// impl Service<time::Duration> for TimeoutService {
//     type Response = ();
//     type Error = time
// }

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
        let echo = EchoService{};
        let delayed_echo = TimeoutLayer{
            duration: time::Duration::new(1, 0)
        }.layer(echo);
        let mut log_delayed_echo = LogLayer{
            log_str: "Echo",
        }.layer(delayed_echo);
        for _ in 0..iter_num {
            let req = req_receiver.recv().await.unwrap();
            let resp = log_delayed_echo.call(req).await.unwrap();
            resp_sender.send(resp).await.unwrap();
        }
    };

    let t1 = task::spawn(mock_client);
    let t2 = task::spawn(mock_server);

    t1.await.unwrap();
    t2.await.unwrap();
}