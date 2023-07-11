use serde_derive::{Deserialize, Serialize};
use warp::Filter;
#[derive(Deserialize, Serialize)]
struct Location {
    location: String,
}

#[derive(Deserialize, Serialize)]
struct Policy {
    policy: String,
}

pub(crate) async fn register_s3_route() {
    let root = warp::path::end().map(|| "Hello, World at root!\n");

    let bucket = warp::path::param::<String>();
    // get bucket location
    let bucket_location = bucket
        .and(warp::get())
        .and(warp::query::<Location>())
        .map(|bucket, q| "location");

    //
    let bucket_policy = bucket
        .and(warp::get())
        .and(warp::query::<Policy>())
        .map(|bucket, q| "policy");
    
    


    let route = root.or(bucket_location).or(bucket_policy);
    warp::serve(route).run(([127, 0, 0, 1], 9300)).await;
    // GET /hello/warp => 200 OK with body "Hello, warp!"
}
