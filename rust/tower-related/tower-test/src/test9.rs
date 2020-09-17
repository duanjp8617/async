use tokio::sync::mpsc::channel;
use tokio::{time, task};

use http::{Request, Response, StatusCode};
use tower::Service;

use std::result::Result;

use std::marker::{PhantomData, Unpin};
use std::future::Future;
use std::pin::Pin;
use std::task::{Poll, Context};

struct LocalFutureObj<'a, T> {
    future_ptr: *mut (dyn Future<Output = T> + 'a),
    drop_fn: unsafe fn(*mut (dyn Future<Output = T> + 'a)),
    marker: PhantomData<&'a ()>,
}

// #[allow(single_use_lifetimes)]
// unsafe fn remove_drop_lifetime<'a, T>(ptr: unsafe fn (*mut (dyn Future<Output = T> + 'a)))
//     -> unsafe fn(*mut (dyn Future<Output = T> + 'static))
// {
//     mem::transmute(ptr)
// }

impl<T> Unpin for LocalFutureObj<'_, T> {}

impl<'a, T> LocalFutureObj<'a, T> {
    pub fn new<F: UnsafeFutureObj<'a, T>>(f: F) -> Self {
        LocalFutureObj {
            future_ptr: f.into_raw(),
            drop_fn: F::drop_fn,
            marker: PhantomData,
        }
    }
}

impl<T> Future for LocalFutureObj<'_, T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll::<Self::Output> {
        unsafe {
            Future::poll(Pin::new_unchecked(&mut *self.future_ptr), ctx)
        }
    }
}

impl<T> Drop for LocalFutureObj<'_, T> {
    fn drop(&mut self) {
        unsafe {
            (self.drop_fn)(self.future_ptr);
        }
    }
}

struct FutureObj<'a, T>(LocalFutureObj<'a, T>);

impl<T> Unpin for FutureObj<'_, T> {}
unsafe impl<T> Send for FutureObj<'_, T> {}

impl<'a, T> FutureObj<'a, T> {
    fn new<F: UnsafeFutureObj<'a, T> + Send>(f: F) -> Self{
        Self(LocalFutureObj::new(f))
    }
}

impl<T> Future for FutureObj<'_, T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll::<Self::Output> {
        Future::poll(Pin::new(&mut self.0), ctx)
    }
}

unsafe trait UnsafeFutureObj<'a, T> {
    fn into_raw(self) -> *mut (dyn Future<Output = T> + 'a);
    
    unsafe fn drop_fn(future_ptr: *mut (dyn Future<Output = T> + 'a));
}

unsafe impl<'a, T> UnsafeFutureObj<'a, T> for Box<dyn Future<Output = T> + Send + 'a>
{
    fn into_raw(self) -> *mut (dyn Future<Output = T> + 'a) {
        Box::into_raw(self)
    }

    unsafe fn drop_fn(future_ptr: *mut (dyn Future<Output = T> + 'a)) {
        drop(Box::from_raw(future_ptr));
    }
}

struct EchoService;

fn convert<'a>(req: &'a Request<Vec<u8>>) -> Box<dyn Future<Output = Result<Response<Vec<u8>>, http::Error>> + Send + 'a> {
    let async_closure = async move {
        time::delay_for(time::Duration::new(1, 0)).await;
        let resp = Response::builder()
            .status(StatusCode::OK)
            .body(req.body().clone())
            .unwrap();
        
        Ok(resp)
    };
    Box::new(async_closure)
}

impl<'a> Service<&'a Request<Vec<u8>>> for EchoService {
    type Response = Response<Vec<u8>>;
    type Error = http::Error;
    type Future = FutureObj<'a, Result<Response<Vec<u8>>, http::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: &'a Request<Vec<u8>>) -> Self::Future {
        // let async_closure = async move {
        //     time::delay_for(time::Duration::new(1, 0)).await;
        //     let resp = Response::builder()
        //         .status(StatusCode::OK)
        //         .body(req.body().clone())
        //         .unwrap();
            
        //     Ok(resp)
        // };
        FutureObj::new(convert(req))
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
            let resp_fut = echo.call(&req);
            // drop(req);
            let resp = resp_fut.await.unwrap();
            drop(req);
            resp_sender.send(resp).await.unwrap();
        }
    };

    let t1 = task::spawn(mock_client);
    let t2 = task::spawn(mock_server);

    t1.await.unwrap();
    t2.await.unwrap();
}