use axum::{
    routing::get,
    Router, body::StreamBody,
};
use futures_util::Stream;
use std::{net::{Ipv4Addr, SocketAddr}, pin::Pin, task::{Poll, Context}, thread, time::Duration, io};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route(
            "/stream",
            get(mystream),
        );

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


struct MyStream;

impl Stream for MyStream {
    type Item = io::Result<String>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        println!("poll next");
        thread::sleep(Duration::from_secs(1));
        Poll::Ready(Some(Ok("1\n".to_string())))
    }
}



async fn mystream() -> StreamBody<impl Stream <Item = io::Result<String>>> {
    StreamBody::new(MyStream{})
}
