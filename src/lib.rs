use serde::de::DeserializeOwned;
use simulation::SimulationRequest;
use warp::{Filter, Rejection, Reply};

pub mod config;
use config::Config;

pub mod errors;
pub mod evm;

pub mod simulation;

pub fn simulate_routes(
    config: Config,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    simulate(config.clone()).or(simulate_bundle(config))
}

/// POST /simulate
pub fn simulate(config: Config) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("simulate")
        .and(warp::post())
        .and(json_body::<SimulationRequest>())
        .and(with_config(config))
        .and_then(simulation::simulate)
}

/// POST /simulate-bundle
pub fn simulate_bundle(
    config: Config,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("simulate-bundle")
        .and(warp::post())
        .and(json_body())
        .and(with_config(config))
        .and_then(simulation::simulate_bundle)
}

fn with_config(
    config: Config,
) -> impl Filter<Extract = (Config,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || config.clone())
}

fn json_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
