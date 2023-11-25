use dashmap::DashMap;
use evm::Evm;
use serde::de::DeserializeOwned;
use simulation::{SimulationRequest, StatefulSimulationRequest};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

pub mod config;
use config::Config;

pub mod errors;
pub mod evm;

pub mod simulation;

pub struct SharedSimulationState {
    pub evms: Arc<DashMap<Uuid, Arc<Mutex<Evm>>>>,
}

pub fn simulate_routes(
    config: Config,
    state: Arc<SharedSimulationState>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    simulate(config.clone())
        .or(simulate_bundle(config.clone()))
        .or(simulate_stateful_new(config.clone(), state.clone()))
        .or(simulate_stateful_end(state.clone()))
        .or(simulate_stateful(config, state))
}

/// POST /simulate
pub fn simulate(config: Config) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("simulate")
        .and(warp::post())
        .and(json_body::<SimulationRequest>(&config))
        .and(with_config(config))
        .and_then(simulation::simulate)
}

/// POST /simulate-bundle
pub fn simulate_bundle(
    config: Config,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("simulate-bundle")
        .and(warp::post())
        .and(json_body(&config))
        .and(with_config(config))
        .and_then(simulation::simulate_bundle)
}

/// POST /simulate-stateful
pub fn simulate_stateful_new(
    config: Config,
    state: Arc<SharedSimulationState>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("simulate-stateful")
        .and(warp::post())
        .and(json_body::<StatefulSimulationRequest>(&config))
        .and(with_config(config))
        .and(with_state(state))
        .and_then(simulation::simulate_stateful_new)
}

/// DELETE /simulate-stateful/{statefulSimulationId}
pub fn simulate_stateful_end(
    state: Arc<SharedSimulationState>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("simulate-stateful" / Uuid)
        .and(warp::delete())
        .and(with_state(state))
        .and_then(simulation::simulate_stateful_end)
}

/// POST /simulate-stateful/{statefulSimulationId}
pub fn simulate_stateful(
    config: Config,
    state: Arc<SharedSimulationState>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("simulate-stateful" / Uuid)
        .and(warp::post())
        .and(json_body(&config))
        .and(with_state(state))
        .and_then(simulation::simulate_stateful)
}

fn with_config(
    config: Config,
) -> impl Filter<Extract = (Config,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || config.clone())
}

fn with_state(
    state: Arc<SharedSimulationState>,
) -> impl Filter<Extract = (Arc<SharedSimulationState>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || state.clone())
}

fn json_body<T: DeserializeOwned + Send>(
    config: &Config,
) -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
    warp::body::content_length_limit(config.max_request_size).and(warp::body::json())
}
