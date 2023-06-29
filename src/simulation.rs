use std::str::FromStr;
use std::sync::Arc;

use dashmap::mapref::one::RefMut;
use ethers::abi::{Address, Uint};
use ethers::core::types::Log;
use ethers::types::Bytes;
use foundry_evm::CallKind;
use revm::interpreter::InstructionResult;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::reject::custom;
use warp::reply::Json;
use warp::Rejection;

use crate::errors::{
    FromDecStrError, FromHexError, IncorrectChainIdError, InvalidBlockNumbersError,
    MultipleChainIdsError, NoURLForChainIdError,
};
use crate::SharedSimulationState;

use super::config::Config;
use super::evm::Evm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationRequest {
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    pub from: Address,
    pub to: Address,
    pub data: Option<Bytes>,
    #[serde(rename = "gasLimit")]
    pub gas_limit: u64,
    pub value: Option<String>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<u64>,
    #[serde(rename = "formatTrace")]
    pub format_trace: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimulationResponse {
    #[serde(rename = "simulationId")]
    pub simulation_id: u64,
    #[serde(rename = "gasUsed")]
    pub gas_used: u64,
    #[serde(rename = "blockNumber")]
    pub block_number: u64,
    pub success: bool,
    pub trace: Vec<CallTrace>,
    #[serde(rename = "formattedTrace")]
    pub formatted_trace: Option<String>,
    pub logs: Vec<Log>,
    #[serde(rename = "exitReason")]
    pub exit_reason: InstructionResult,
    #[serde(rename = "returnData")]
    pub return_data: Bytes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatefulSimulationRequest {
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "gasLimit")]
    pub gas_limit: u64,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatefulSimulationResponse {
    #[serde(rename = "statefulSimulationId")]
    pub stateful_simulation_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatefulSimulationEndResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallTrace {
    #[serde(rename = "callType")]
    pub call_type: CallKind,
    pub from: Address,
    pub to: Address,
    pub value: Uint,
}

fn chain_id_to_fork_url(chain_id: u64) -> Result<String, Rejection> {
    match chain_id {
        // ethereum
        1 => Ok("https://eth.llamarpc.com".to_string()),
        5 => Ok("https://eth-goerli.g.alchemy.com/v2/demo".to_string()),
        11155111 => Ok("https://eth-sepolia.g.alchemy.com/v2/demo".to_string()),
        // polygon
        137 => Ok("https://polygon-mainnet.g.alchemy.com/v2/demo".to_string()),
        80001 => Ok("https://polygon-mumbai.g.alchemy.com/v2/demo".to_string()),
        // avalanche
        43114 => Ok("https://api.avax.network/ext/bc/C/rpc".to_string()),
        43113 => Ok("https://api.avax-test.network/ext/bc/C/rpc".to_string()),
        // fantom
        250 => Ok("https://rpcapi.fantom.network/".to_string()),
        4002 => Ok("https://rpc.testnet.fantom.network/".to_string()),
        // xdai
        100 => Ok("https://rpc.xdaichain.com/".to_string()),
        // bsc
        56 => Ok("https://bsc-dataseed.binance.org/".to_string()),
        97 => Ok("https://data-seed-prebsc-1-s1.binance.org:8545/".to_string()),
        // arbitrum
        42161 => Ok("https://arb1.arbitrum.io/rpc".to_string()),
        421613 => Ok("https://goerli-rollup.arbitrum.io/rpc".to_string()),
        // optimism
        10 => Ok("https://mainnet.optimism.io/".to_string()),
        420 => Ok("https://goerli.optimism.io/".to_string()),
        _ => Err(NoURLForChainIdError.into()),
    }
}

async fn run(
    evm: &mut Evm,
    transaction: SimulationRequest,
    commit: bool,
) -> Result<SimulationResponse, Rejection> {
    // Accept value in hex or decimal formats
    let value = if let Some(value) = transaction.value {
        if value.starts_with("0x") {
            Some(Uint::from_str(value.as_str()).map_err(|_err| custom(FromHexError))?)
        } else {
            Some(Uint::from_dec_str(value.as_str()).map_err(|_err| custom(FromDecStrError))?)
        }
    } else {
        None
    };

    let result = if commit {
        evm.call_raw_committing(
            transaction.from,
            transaction.to,
            value,
            transaction.data,
            transaction.gas_limit,
            transaction.format_trace.unwrap_or_default(),
        )
        .await?
    } else {
        evm.call_raw(
            transaction.from,
            transaction.to,
            value,
            transaction.data,
            transaction.format_trace.unwrap_or_default(),
        )
        .await?
    };

    Ok(SimulationResponse {
        simulation_id: 1,
        gas_used: result.gas_used,
        block_number: result.block_number,
        success: result.success,
        trace: result
            .trace
            .unwrap_or_default()
            .arena
            .into_iter()
            .map(CallTrace::from)
            .collect(),
        logs: result.logs,
        exit_reason: result.exit_reason,
        formatted_trace: result.formatted_trace,
        return_data: result.return_data,
    })
}

pub async fn simulate(transaction: SimulationRequest, config: Config) -> Result<Json, Rejection> {
    let fork_url = config
        .fork_url
        .unwrap_or(chain_id_to_fork_url(transaction.chain_id)?);
    let mut evm = Evm::new(
        None,
        fork_url,
        transaction.block_number,
        transaction.gas_limit,
        true,
        config.etherscan_key,
    );

    if evm.get_chain_id() != Uint::from(transaction.chain_id) {
        return Err(warp::reject::custom(IncorrectChainIdError()));
    }

    let response = run(&mut evm, transaction, false).await?;

    Ok(warp::reply::json(&response))
}

pub async fn simulate_bundle(
    transactions: Vec<SimulationRequest>,
    config: Config,
) -> Result<Json, Rejection> {
    let first_chain_id = transactions[0].chain_id;
    let first_block_number = transactions[0].block_number;

    let fork_url = config
        .fork_url
        .unwrap_or(chain_id_to_fork_url(first_chain_id)?);
    let mut evm = Evm::new(
        None,
        fork_url,
        first_block_number,
        transactions[0].gas_limit,
        true,
        config.etherscan_key,
    );

    if evm.get_chain_id() != Uint::from(first_chain_id) {
        return Err(warp::reject::custom(IncorrectChainIdError()));
    }

    let mut response = Vec::with_capacity(transactions.len());
    for transaction in transactions {
        if transaction.chain_id != first_chain_id {
            return Err(warp::reject::custom(MultipleChainIdsError()));
        }
        if transaction.block_number != first_block_number {
            let tx_block = transaction
                .block_number
                .expect("Transaction has no block number");
            if transaction.block_number < first_block_number || tx_block < evm.get_block().as_u64()
            {
                return Err(warp::reject::custom(InvalidBlockNumbersError()));
            }
            evm.set_block(tx_block)
                .await
                .expect("Failed to set block number");
            evm.set_block_timestamp(evm.get_block_timestamp().as_u64() + 12)
                .await
                .expect("Failed to set block timestamp");
        }
        response.push(run(&mut evm, transaction, true).await?);
    }

    Ok(warp::reply::json(&response))
}

pub async fn simulate_stateful_new(
    stateful_simulation_request: StatefulSimulationRequest,
    config: Config,
    state: Arc<SharedSimulationState>,
) -> Result<Json, Rejection> {
    let fork_url = config
        .fork_url
        .unwrap_or(chain_id_to_fork_url(stateful_simulation_request.chain_id)?);
    let evm = Evm::new(
        None,
        fork_url,
        stateful_simulation_request.block_number,
        stateful_simulation_request.gas_limit,
        true,
        config.etherscan_key,
    );
    let new_id = Uuid::new_v4();
    state.evms.insert(new_id, Arc::new(Mutex::new(evm)));

    let response = StatefulSimulationResponse {
        stateful_simulation_id: new_id,
    };

    Ok(warp::reply::json(&response))
}

pub async fn simulate_stateful_end(
    param: Uuid,
    state: Arc<SharedSimulationState>,
) -> Result<Json, Rejection> {
    if state.evms.contains_key(&param) {
        state.evms.remove(&param);
        let response = StatefulSimulationEndResponse { success: true };
        Ok(warp::reply::json(&response))
    } else {
        let response = StatefulSimulationEndResponse { success: false };
        Ok(warp::reply::json(&response))
    }
}

pub async fn simulate_stateful(
    param: Uuid,
    transactions: Vec<SimulationRequest>,
    state: Arc<SharedSimulationState>,
) -> Result<Json, Rejection> {
    let first_chain_id = transactions[0].chain_id;
    let first_block_number = transactions[0].block_number;

    let mut response = Vec::with_capacity(transactions.len());

    // Get a mutable reference to the EVM here.
    let evm_ref_mut: RefMut<'_, Uuid, Arc<Mutex<Evm>>> = state
        .evms
        .get_mut(&param)
        .ok_or_else(warp::reject::not_found)?;

    // Dereference to obtain the EVM.
    let evm = evm_ref_mut.value();
    let mut evm = evm.lock().await;

    if evm.get_chain_id() != Uint::from(first_chain_id) {
        return Err(warp::reject::custom(IncorrectChainIdError()));
    }

    for transaction in transactions {
        if transaction.chain_id != first_chain_id {
            return Err(warp::reject::custom(MultipleChainIdsError()));
        }
        if transaction.block_number != first_block_number
            || transaction.block_number.unwrap() != evm.get_block().as_u64()
        {
            let tx_block = transaction
                .block_number
                .expect("Transaction has no block number");
            if transaction.block_number < first_block_number || tx_block < evm.get_block().as_u64()
            {
                return Err(warp::reject::custom(InvalidBlockNumbersError()));
            }
            evm.set_block(tx_block)
                .await
                .expect("Failed to set block number");
            let block_timestamp = evm.get_block_timestamp().as_u64();
            evm.set_block_timestamp(block_timestamp + 12)
                .await
                .expect("Failed to set block timestamp");
        }
        response.push(run(&mut evm, transaction, true).await?);
    }

    Ok(warp::reply::json(&response))
}
