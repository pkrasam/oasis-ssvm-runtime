// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Web3 gateway.

extern crate clap;
extern crate futures;
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate parking_lot;
extern crate rayon;
#[macro_use]
extern crate serde_derive;
extern crate jsonrpc_core;
extern crate serde;
#[macro_use]
extern crate jsonrpc_macros;
extern crate common_types;
extern crate ethcore;
extern crate ethcore_bytes as bytes;
extern crate ethcore_transaction as transaction;
extern crate ethereum_types;
extern crate failure;
extern crate grpcio;
#[cfg(test)]
extern crate hex;
extern crate io_context;
extern crate jsonrpc_http_server;
extern crate jsonrpc_pubsub;
extern crate jsonrpc_ws_server;
extern crate keccak_hash as hash;
extern crate kvdb;
extern crate parity_reactor;
extern crate parity_rpc;
extern crate prometheus;
extern crate rlp_compress;
extern crate serde_cbor;
extern crate slog;
extern crate tokio;
extern crate tokio_threadpool;

extern crate ekiden_client;
extern crate ekiden_keymanager_client;
extern crate ekiden_runtime;

extern crate runtime_ethereum_api;
extern crate runtime_ethereum_common;

mod impls;
mod informant;
mod middleware;
mod pubsub;
mod rpc;
mod rpc_apis;
mod run;
mod servers;
mod traits;
mod translator;
pub mod util;

use std::sync::Arc;

use clap::{value_t_or_exit, ArgMatches};
use ekiden_client::{create_txn_api_client, Node, TxnClient};
use ekiden_runtime::common::runtime::RuntimeId;
use ethereum_types::U256;
use failure::Fallible;
use grpcio::EnvBuilder;
use runtime_ethereum_api::*;

pub use self::run::RunningGateway;

with_api! {
    create_txn_api_client!(EthereumRuntimeClient, api);
}

pub fn start(
    args: ArgMatches,
    pubsub_interval_secs: u64,
    http_port: u16,
    num_threads: usize,
    ws_port: u16,
    ws_max_connections: usize,
    ws_rate_limit: usize,
    gas_price: U256,
    jsonrpc_max_batch_size: usize,
) -> Fallible<RunningGateway> {
    let node_address = args.value_of("node-address").unwrap();
    let runtime_id = value_t_or_exit!(args, "runtime-id", RuntimeId);

    let env = Arc::new(EnvBuilder::new().build());
    let node = Node::new(env.clone(), node_address);
    let txn_client = TxnClient::new(node.channel(), runtime_id, None);
    let client = EthereumRuntimeClient::new(txn_client);
    // TODO: Key manager MRENCLAVE.
    let km_client = Arc::new(ekiden_keymanager_client::RemoteClient::new_grpc(
        runtime_id,
        None,
        node.channel(),
    ));

    run::execute(
        client,
        km_client,
        pubsub_interval_secs,
        http_port,
        num_threads,
        ws_port,
        ws_max_connections,
        ws_rate_limit,
        gas_price,
        jsonrpc_max_batch_size,
    )
}
