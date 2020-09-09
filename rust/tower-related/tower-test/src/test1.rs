use std::pin::Pin;
use std::task::{Poll, Context};
use std::future::Future;
use tower_service::Service;

 use http::{Request, Response, StatusCode};

struct HelloWorld;

impl Service<Request<Vec<u8>>> for HelloWorld {
    type Response = Response<Vec<u8>>;
    type Error = http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Vec<u8>>) -> Self::Future {
        // create the body
        let body: Vec<u8> = "hello, world!\n"
            .as_bytes()
            .to_owned();
        // Create the HTTP response
        let resp = Response::builder()
            .status(StatusCode::OK)
            .body(body)
            .expect("Unable to create `http::Response`");
        
        // create a response in a future.
        let fut = async {
            Ok(resp)
        };

        // Return the response as an immediate future
        Box::pin(fut)
    }
}

#[tokio::main]
async fn main() {
    // create an HTTP request
    let req = Request::builder()
            .uri("www.fuck.com")
            .header("User-Agent", "my-fucking-agent-1.0").body(Vec::new()).unwrap();
    
    let mut service = HelloWorld{};
    let res = service.call(req).await.unwrap();
    println!("response is {}", std::str::from_utf8(res.body()).unwrap());
}