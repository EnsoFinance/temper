use evm::Evm;
use serde::de::DeserializeOwned;
use simulation::{SimulationRequest, StatefulSimulationRequest};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use warp::{Filter, Rejection, Reply};

pub mod config;
use config::Config;

pub mod errors;
pub mod evm;

pub mod simulation;

pub struct SharedSimulationState {
    pub stateful_simulation_id: Arc<Mutex<u32>>,
    pub evms: Arc<Mutex<HashMap<u32, Evm>>>,
}

pub fn simulate_routes(
    config: Config,
    state: Arc<SharedSimulationState>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    simulate(config.clone())
        .or(simulate_bundle(config.clone()))
        .or(simulate_stateful_new(config.clone(), state.clone()))
        .or(simulate_stateful(config, state))
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

/// POST /simulate-stateful-new
pub fn simulate_stateful_new(
    config: Config,
    state: Arc<SharedSimulationState>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("simulate-stateful-new")
        .and(warp::post())
        .and(json_body::<StatefulSimulationRequest>())
        .and(with_config(config))
        .and(with_state(state))
        .and_then(simulation::simulate_stateful_new)
}

/// POST /simulate-stateful/{simulation_id}
pub fn simulate_stateful(
    config: Config,
    state: Arc<SharedSimulationState>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("simulate-stateful")
        .and(warp::path::param::<u32>())
        .and(warp::post())
        .and(json_body())
        .and(with_config(config))
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

fn json_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
