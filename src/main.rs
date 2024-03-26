use cxc_exchange_api::engine::commodity::Commodity;
use warp::{http::StatusCode, Filter, Rejection, Reply};
use warp::reject::{custom, Reject};
use tokio_rusqlite::Error;

#[derive(Debug)]
struct CustomError {
    message: String
}

impl Reject for CustomError{
    
}
#[tokio::main]
async fn main() {
    // POST /api/commodities
    let route = warp::path!("api" / "commodities")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|commodity: Commodity| async move {
            commodity.add_update().await
                .map_err(|e| warp::reject::custom(CustomError { message: e.to_string() })) // Assuming add_update returns Result<(), Error>
                .map(|_| warp::reply::with_status("Commodity added or updated", StatusCode::CREATED))
        });

    // GET /api/commodities/{id}
    let route1 = warp::path!("api" / "commodities" / i32)
        .and_then(|id| async move {
            Commodity::hydrate(id).await
                .map(|commodity| warp::reply::json(&commodity))
                .map_err(|e| warp::reject::custom(CustomError { message: e.to_string() })) // Assuming hydrate returns Result<Commodity, Error>
        });

    // GET /api/commodities
    let route2 = warp::path!("api" / "commodities")
        .and_then(|| async move {
            Commodity::get_all().await
                .map(|commodities| warp::reply::json(&commodities))
                .map_err(|e| warp::reject::custom(CustomError { message: e.to_string() })) // Assuming get_all returns Result<Vec<Commodity>, Error>
        });

    let routes = route.or(route1).or(route2);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
