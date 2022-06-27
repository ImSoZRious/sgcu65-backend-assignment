use warp::Filter;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // TODO: init database

    // TODO: get path
    let path = warp::path::end().map(|| "Sawadee kub");

    // serve
    warp::serve(path).run(([127, 0, 0, 1], 3030)).await;
}
