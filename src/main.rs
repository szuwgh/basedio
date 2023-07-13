mod api_function;
mod erasure;
mod fs;
mod object;
mod route;
mod storage;
use crate::api_function::ObjectAPIFunction;
use fs::Fs;
use object::ObjectManager;
use route::*;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let storage_impl = ObjectManager::new(Fs {});
    let root = route::register_s3_route(storage_impl);
    warp::serve(root).run(([127, 0, 0, 1], 9300)).await;
}
