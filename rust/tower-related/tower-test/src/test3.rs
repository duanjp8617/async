use tokio::sync::mpsc::channel;
use tokio::{time, task};
use http::{Request, Response, StatusCode};
use tower::Service;

use std::pin::Pin;
use std::future::Future; 
use std::result::Result;
use std::task::{Poll, Context};

use std::sync::Arc;
use futures_task::{ArcWake};

/// Delay the response for a certain amount of time 
struct DelayedResponse {
    delay: time::Delay,
    resp: Response<Vec<u8>>
}

impl Unpin for DelayedResponse{}

impl Future for DelayedResponse {
    type Output = Result<Response<Vec<u8>>, http::Error>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        match Future::poll(Pin::new(&mut self.delay), ctx) {
            Poll::Ready(()) => {
                let inner = Pin::into_inner(self);
                let resp = std::mem::replace(&mut inner.resp, Response::builder().body(Vec::new()).unwrap());
                Poll::Ready(Ok(resp))
            },
            Poll::Pending => {
                Poll::Pending
            }
        }
    }
}


/// The following piece of code tests whether polling a Future trait object is equivalent to 
/// polling the underlying Future object
// struct WakerImpl {
//     task_id : usize,
// }

// impl ArcWake for WakerImpl {
//     fn wake_by_ref(arc_self : &Arc<Self>) {
//        // Do Nothing
//     }
// }

// fn poll_delayed_response(f: *mut (dyn Future<Output = Result<Response<Vec<u8>>, http::Error>> + 'static)) 
//     -> Poll<Result<Response<Vec<u8>>, http::Error>>{

//     let waker = futures_task::waker(Arc::new(WakerImpl{task_id : 5}));
//     let mut ctx = Context::from_waker(& waker);
//     let res = unsafe{ Future::poll(Pin::new_unchecked(&mut *f), &mut ctx) };
     
//     res
// }

struct EchoService;

impl Service<Request<Vec<u8>>> for EchoService {
    type Response = Response<Vec<u8>>;
    type Error = http::Error;
    type Future = DelayedResponse;

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

        DelayedResponse {
            delay: time::delay_for(time::Duration::new(1,0)),
            resp: resp,
        }
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