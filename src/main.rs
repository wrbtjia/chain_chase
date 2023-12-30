mod router;
mod handler;
mod api;
mod remote;
mod domain;

#[tokio::main]
async fn main()  {
    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8090")
        .await
        .unwrap();

    axum::serve(listener, router::get_routes()).await.unwrap();

}
