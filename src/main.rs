mod fs;
mod route;
mod storage;
use crate::route::*;

#[tokio::main]
async fn main() {
    route::register_s3_route().await;
}
