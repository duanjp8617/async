use std::marker::{PhantomData, Unpin};
use std::future::Future;
use std::pin::Pin;
use std::task::{Poll, Context};

// This LocalFutureObj is not compatible with async closure. 
// Because the type of async closre can not be specified.
// Here, to use LocalFutureObj as a struct field, we must specify the 
// type of F.
// Therefore, we can never construct such a struct field with async closure,
// which is not acceptble.
struct LocalFutureObj<'a, T, F: Future<Output = T> + 'a> {
    future_ptr: *mut F,
    drop_fn: unsafe fn(*mut F),
    marker: PhantomData<&'a ()>,
}

impl<'a, T, F: Future<Output = T> + 'a> Unpin for LocalFutureObj<'a, T, F> {}

#[allow(dead_code)]
impl<'a, T, F: Future<Output = T> + 'a> LocalFutureObj<'a, T, F> {
    fn new<UF: UnsafeFutureObj<'a, T, F>>(f: UF) -> Self {
        LocalFutureObj {
            future_ptr: f.into_raw(),
            drop_fn: UF::drop_fn,
            marker: PhantomData,
        }
    }
}

impl<'a, T, F: Future<Output = T> + 'a> Future for LocalFutureObj<'a, T, F> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            F::poll(Pin::new_unchecked(&mut *self.future_ptr), cx)
        }
    }
}

impl<'a, T, F: Future<Output = T> + 'a> Drop for LocalFutureObj<'a, T, F> {
    fn drop(&mut self) {
        unsafe {
            (self.drop_fn)(self.future_ptr);
        }
    }
}

unsafe trait UnsafeFutureObj<'a, T, F: Future<Output = T> + 'a > {
    fn into_raw(self) -> *mut F;

    unsafe fn drop_fn(future_ptr: *mut F);
}

unsafe impl<'a, T, F> UnsafeFutureObj<'a, T, F> for Box<F>
    where F: Future<Output = T> + 'a
{
    fn into_raw(self) -> *mut F {
        Box::into_raw(self)
    }

    unsafe fn drop_fn(future_ptr: *mut F) {
        drop(Box::from_raw(future_ptr))
    }
}

// struct EchoService;

// impl Service<Request<Vec<u8>>> for EchoService {
//     type Response = Response<Vec<u8>>;
//     type Error = http::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

//     fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, mut req: Request<Vec<u8>>) -> Self::Future {
//         let mut resp_body = "echo body back: ".as_bytes().to_owned();
//         resp_body.append(req.body_mut());
        
//         let resp = Response::builder()
//             .status(StatusCode::OK)
//             .body(resp_body)
//             .unwrap();

//         Box::pin(async move {
//             time::delay_for(time::Duration::new(1,0)).await;
//             Ok(resp)
//         })
//     }
// }

fn main() {
    println!("Do nothing");
}