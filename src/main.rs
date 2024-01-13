mod router;
mod handler;
mod api;
mod remote;
mod domain;

#[tokio::main]
async fn main()  {
    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, router::get_routes()).await.unwrap();

}
