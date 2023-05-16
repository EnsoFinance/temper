use ethers::abi::{Address, Uint};
use ethers::types::{Bytes, Log};
use foundry_evm::executor::{fork::CreateFork, Executor};
use foundry_evm::executor::{opts::EvmOpts, Backend, ExecutorBuilder};
use foundry_evm::trace::identifier::{EtherscanIdentifier, SignaturesIdentifier};
use foundry_evm::trace::node::CallTraceNode;
use foundry_evm::trace::{CallTraceArena, CallTraceDecoder, CallTraceDecoderBuilder};
use revm::Env;
use revm::Return;

use crate::errors::EvmError;
use crate::simulation::CallTrace;

#[derive(Debug, Clone)]
pub struct CallRawResult {
    pub gas_used: u64,
    pub block_number: u64,
    pub success: bool,
    pub trace: Option<CallTraceArena>,
    pub logs: Vec<Log>,
    pub exit_reason: Return,
    pub return_data: Bytes,
    pub formatted_trace: Option<String>,
}

impl From<CallTraceNode> for CallTrace {
    fn from(item: CallTraceNode) -> Self {
        CallTrace {
            call_type: item.trace.kind,
            from: item.trace.caller,
            to: item.trace.address,
            value: item.trace.value,
        }
    }
}

pub struct Evm {
    executor: Executor,
    decoder: CallTraceDecoder,
    etherscan_identifier: Option<EtherscanIdentifier>,
}

impl Evm {
    pub fn new(
        env: Option<Env>,
        fork_url: String,
        fork_block_number: Option<u64>,
        gas_limit: u64,
        tracing: bool,
        etherscan_key: Option<String>,
    ) -> Self {
        let evm_opts = EvmOpts {
            fork_url: Some(fork_url.clone()),
            fork_block_number,
            env: foundry_evm::executor::opts::Env {
                chain_id: None,
                code_size_limit: None,
                gas_price: Some(0),
                gas_limit: u64::MAX,
                ..Default::default()
            },
            memory_limit: foundry_config::Config::default().memory_limit,
            ..Default::default()
        };

        let fork_opts = CreateFork {
            url: fork_url,
            enable_caching: true,
            env: evm_opts.evm_env_blocking().unwrap(),
            evm_opts,
        };

        let db = Backend::spawn(Some(fork_opts.clone()));

        let mut builder = ExecutorBuilder::default()
            .with_gas_limit(gas_limit.into())
            .set_tracing(tracing);

        if let Some(env) = env {
            builder = builder.with_config(env);
        } else {
            builder = builder.with_config(fork_opts.env.clone());
        }

        let executor = builder.build(db);

        let foundry_config = foundry_config::Config {
            etherscan_api_key: etherscan_key,
            ..Default::default()
        };

        let etherscan_identifier =
            EtherscanIdentifier::new(&foundry_config, Some(fork_opts.env.cfg.chain_id)).ok();
        let mut decoder = CallTraceDecoderBuilder::new().with_verbosity(5).build();

        if let Ok(identifier) =
            SignaturesIdentifier::new(foundry_config::Config::foundry_cache_dir(), false)
        {
            decoder.add_signature_identifier(identifier);
        }

        Evm {
            executor,
            decoder,
            etherscan_identifier,
        }
    }

    pub async fn call_raw(
        &mut self,
        from: Address,
        to: Address,
        value: Option<Uint>,
        data: Option<Bytes>,
        format_trace: bool,
    ) -> Result<CallRawResult, EvmError> {
        let res = self
            .executor
            .call_raw(
                from,
                to,
                data.unwrap_or_default().0,
                value.unwrap_or_default(),
            )
            .map_err(|err| {
                dbg!(&err);
                EvmError(err)
            })?;

        let formatted_trace = if format_trace {
            let mut output = String::new();
            for trace in &mut res.traces.clone() {
                if let Some(identifier) = &mut self.etherscan_identifier {
                    self.decoder.identify(trace, identifier);
                }
                self.decoder.decode(trace).await;
                output.push_str(format!("{trace}").as_str());
            }
            Some(output)
        } else {
            None
        };

        Ok(CallRawResult {
            gas_used: res.gas_used,
            block_number: res.env.block.number.as_u64(),
            success: !res.reverted,
            trace: res.traces,
            logs: res.logs,
            exit_reason: res.exit_reason,
            return_data: Bytes(res.result),
            formatted_trace,
        })
    }

    pub async fn call_raw_committing(
        &mut self,
        from: Address,
        to: Address,
        value: Option<Uint>,
        data: Option<Bytes>,
        gas_limit: u64,
        format_trace: bool,
    ) -> Result<CallRawResult, EvmError> {
        self.executor.set_gas_limit(gas_limit.into());
        let res = self
            .executor
            .call_raw_committing(
                from,
                to,
                data.unwrap_or_default().0,
                value.unwrap_or_default(),
            )
            .map_err(|err| {
                dbg!(&err);
                EvmError(err)
            })?;

        let formatted_trace = if format_trace {
            let mut output = String::new();
            for trace in &mut res.traces.clone() {
                if let Some(identifier) = &mut self.etherscan_identifier {
                    self.decoder.identify(trace, identifier);
                }
                self.decoder.decode(trace).await;
                output.push_str(format!("{trace}").as_str());
            }
            Some(output)
        } else {
            None
        };

        Ok(CallRawResult {
            gas_used: res.gas_used,
            block_number: res.env.block.number.as_u64(),
            success: !res.reverted,
            trace: res.traces,
            logs: res.logs,
            exit_reason: res.exit_reason,
            return_data: Bytes(res.result),
            formatted_trace,
        })
    }

    pub fn get_chain_id(&self) -> Uint {
        self.executor.env().cfg.chain_id
    }
}
