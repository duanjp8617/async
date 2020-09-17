use tokio::sync::mpsc::channel;
use tokio::{time, task};

use http::{Request, Response, StatusCode};
use tower::Service;

use std::result::Result;

// use std::task::{Context, Poll};
// use std::future::Future;
// use futures_task::FutureObj;

use core::{
    mem,
    fmt,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

/// A custom trait object for polling futures, roughly akin to
/// `Box<dyn Future<Output = T> + 'a>`.
///
/// This custom trait object was introduced as currently it is not possible to
/// take `dyn Trait` by value and `Box<dyn Trait>` is not available in no_std
/// contexts.
pub struct LocalFutureObj<'a, T> {
    future: *mut (dyn Future<Output = T> + 'static),
    drop_fn: unsafe fn(*mut (dyn Future<Output = T> + 'static)),
    _marker: PhantomData<&'a ()>,
}

// As LocalFutureObj only holds pointers, even if we move it, the pointed to values won't move,
// so this is safe as long as we don't provide any way for a user to directly access the pointers
// and move their values.
impl<T> Unpin for LocalFutureObj<'_, T> {}

#[allow(single_use_lifetimes)]
#[allow(clippy::transmute_ptr_to_ptr)]
unsafe fn remove_future_lifetime<'a, T>(ptr: *mut (dyn Future<Output = T> + 'a))
    -> *mut (dyn Future<Output = T> + 'static)
{
    mem::transmute(ptr)
}

#[allow(single_use_lifetimes)]
unsafe fn remove_drop_lifetime<'a, T>(ptr: unsafe fn (*mut (dyn Future<Output = T> + 'a)))
    -> unsafe fn(*mut (dyn Future<Output = T> + 'static))
{
    mem::transmute(ptr)
}

impl<'a, T> LocalFutureObj<'a, T> {
    /// Create a `LocalFutureObj` from a custom trait object representation.
    #[inline]
    pub fn new<F: UnsafeFutureObj<'a, T> + 'a>(f: F) -> LocalFutureObj<'a, T> {
        LocalFutureObj {
            future: unsafe { remove_future_lifetime(f.into_raw()) },
            drop_fn: unsafe { remove_drop_lifetime(F::drop) },
            _marker: PhantomData,
        }
    }

    /// Converts the `LocalFutureObj` into a `FutureObj`.
    ///
    /// # Safety
    ///
    /// To make this operation safe one has to ensure that the `UnsafeFutureObj`
    /// instance from which this `LocalFutureObj` was created actually
    /// implements `Send`.
    #[inline]
    pub unsafe fn into_future_obj(self) -> FutureObj<'a, T> {
        FutureObj(self)
    }
}

impl<T> fmt::Debug for LocalFutureObj<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocalFutureObj")
            .finish()
    }
}

impl<'a, T> From<FutureObj<'a, T>> for LocalFutureObj<'a, T> {
    #[inline]
    fn from(f: FutureObj<'a, T>) -> LocalFutureObj<'a, T> {
        f.0
    }
}

impl<T> Future for LocalFutureObj<'_, T> {
    type Output = T;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        unsafe {
            Pin::new_unchecked(&mut *self.future).poll(cx)
        }
    }
}

impl<T> Drop for LocalFutureObj<'_, T> {
    fn drop(&mut self) {
        unsafe {
            (self.drop_fn)(self.future)
        }
    }
}

/// A custom trait object for polling futures, roughly akin to
/// `Box<dyn Future<Output = T> + Send + 'a>`.
///
/// This custom trait object was introduced as currently it is not possible to
/// take `dyn Trait` by value and `Box<dyn Trait>` is not available in no_std
/// contexts.
///
/// You should generally not need to use this type outside of `no_std` or when
/// implementing `Spawn`, consider using `BoxFuture` instead.
pub struct FutureObj<'a, T>(LocalFutureObj<'a, T>);

impl<T> Unpin for FutureObj<'_, T> {}
unsafe impl<T> Send for FutureObj<'_, T> {}

impl<'a, T> FutureObj<'a, T> {
    /// Create a `FutureObj` from a custom trait object representation.
    #[inline]
    pub fn new<F: UnsafeFutureObj<'a, T> + Send>(f: F) -> FutureObj<'a, T> {
        FutureObj(LocalFutureObj::new(f))
    }
}

impl<T> fmt::Debug for FutureObj<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FutureObj")
            .finish()
    }
}

impl<T> Future for FutureObj<'_, T> {
    type Output = T;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        Pin::new( &mut self.0 ).poll(cx)
    }
}

/// A custom implementation of a future trait object for `FutureObj`, providing
/// a vtable with drop support.
///
/// This custom representation is typically used only in `no_std` contexts,
/// where the default `Box`-based implementation is not available.
///
/// # Safety
///
/// See the safety notes on individual methods for what guarantees an
/// implementor must provide.
pub unsafe trait UnsafeFutureObj<'a, T>: 'a {
    /// Convert an owned instance into a (conceptually owned) fat pointer.
    ///
    /// # Safety
    ///
    /// ## Implementor
    ///
    /// The trait implementor must guarantee that it is safe to convert the
    /// provided `*mut (dyn Future<Output = T> + 'a)` into a `Pin<&mut (dyn
    /// Future<Output = T> + 'a)>` and call methods on it, non-reentrantly,
    /// until `UnsafeFutureObj::drop` is called with it.
    fn into_raw(self) -> *mut (dyn Future<Output = T> + 'a);

    /// Drops the future represented by the given fat pointer.
    ///
    /// # Safety
    ///
    /// ## Implementor
    ///
    /// The trait implementor must guarantee that it is safe to call this
    /// function once per `into_raw` invocation.
    ///
    /// ## Caller
    ///
    /// The caller must ensure:
    ///
    ///  * the pointer passed was obtained from an `into_raw` invocation from
    ///    this same trait object
    ///  * the pointer is not currently in use as a `Pin<&mut (dyn Future<Output
    ///    = T> + 'a)>`
    ///  * the pointer must not be used again after this function is called
    unsafe fn drop(ptr: *mut (dyn Future<Output = T> + 'a));
}

unsafe impl<'a, T, F> UnsafeFutureObj<'a, T> for Box<F>
    where F: Future<Output = T> + 'a
{
    fn into_raw(self) -> *mut (dyn Future<Output = T> + 'a) {
        Box::into_raw(self)
    }

    unsafe fn drop(ptr: *mut (dyn Future<Output = T> + 'a)) {
        drop(Box::from_raw(ptr as *mut F))
    }
}



unsafe impl<'a, T: 'a> UnsafeFutureObj<'a, T> for Box<dyn Future<Output = T> + 'a> {
    fn into_raw(self) -> *mut (dyn Future<Output = T> + 'a) {
        Box::into_raw(self)
    }

    unsafe fn drop(ptr: *mut (dyn Future<Output = T> + 'a)) {
        drop(Box::from_raw(ptr))
    }
}

unsafe impl<'a, T: 'a> UnsafeFutureObj<'a, T> for Box<dyn Future<Output = T> + Send + 'a> {
    fn into_raw(self) -> *mut (dyn Future<Output = T> + 'a) {
        Box::into_raw(self)
    }

    unsafe fn drop(ptr: *mut (dyn Future<Output = T> + 'a)) {
        drop(Box::from_raw(ptr))
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
            // drop(req);
            resp_sender.send(resp).await.unwrap();
        }
    };

    let t1 = task::spawn(mock_client);
    let t2 = task::spawn(mock_server);

    t1.await.unwrap();
    t2.await.unwrap();
}