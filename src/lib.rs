#[macro_use]
extern crate deno_core;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;
// #[macro_use] extern crate log;
extern crate bson;
extern crate redis;
extern crate serde;

use std::sync::RwLock;
use std::sync::Arc;
use deno_core::CoreOp;
use deno_core::PluginInitContext;
use deno_core::{Buf, ZeroCopyBuf};
use futures::FutureExt;
use redis::Client;
use redis::RedisResult;
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

use std::{collections::HashMap, sync::Mutex, sync::MutexGuard};

pub mod command;

pub mod util;

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<usize, Client>> = Mutex::new(HashMap::new());
    static ref CLIENTS_CONNECT: Arc<RwLock<HashMap<usize, Arc<MultiplexedConnection>>>> = Arc::new(RwLock::new(HashMap::new()));
    static ref CLIENT_ID: AtomicUsize = AtomicUsize::new(0);
}

#[derive(Serialize)]
pub struct AsyncResult<T>
where
    T: Serialize,
{
    command_id: usize,
    data: T,
}

#[derive(Serialize, Deserialize)]
pub enum CommandType {
    ConnectWithOptions,
    Cmd,
}

#[derive(Deserialize)]
pub struct CommandArgs {
    command_type: CommandType,
    command_id: Option<usize>,
    client_id: Option<usize>,
}

impl CommandArgs {
    fn new(data: &[u8]) -> CommandArgs {
        serde_json::from_slice(data).unwrap()
    }
}

pub struct Command {
    identity: CommandArgs,
    data: Option<ZeroCopyBuf>,
}

impl Command {
    fn new(identity: CommandArgs, data: Option<ZeroCopyBuf>) -> Command {
        Command { identity, data }
    }
    fn get_client(&self) -> Client {
        get_client(self.identity.client_id.unwrap())
    }
}

fn get_client(client_id: usize) -> Client {
    let map: MutexGuard<HashMap<usize, Client>> = CLIENTS.lock().unwrap();
    map.get(&client_id).unwrap().clone()
}

init_fn!(init);


fn init(context: &mut dyn PluginInitContext) {

    context.register_op("redis_command", Box::new(op_command));
}

fn op_command(data: &[u8], zero_copy: Option<ZeroCopyBuf>) -> CoreOp {
    let args = CommandArgs::new(data);
    let executor = match args.command_type {
        CommandType::ConnectWithOptions => command::get_connection,
        CommandType::Cmd => command::operator::set,
    };
    executor(Command::new(args, zero_copy))
}

#[test]
fn test_future() {
    pub fn create_basic_runtime() -> tokio::runtime::Runtime {
        let mut builder = tokio::runtime::Builder::new();
        builder
            .basic_scheduler()
            .enable_io()
            .enable_time()
            .build()
            .unwrap()
    }

    pub fn run_basic<F, R>(future: F) -> R
    where
        F: std::future::Future<Output = R> + 'static,
    {
        let mut rt = create_basic_runtime();
        rt.block_on(future)
    }
    
    let fut1 = async move {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut connect: MultiplexedConnection =
            client.get_multiplexed_tokio_connection().await.unwrap();

        let data: String = redis::cmd("SET")
            .arg("key1")
            .arg(b"foo")
            .query_async(&mut connect)
            .await
            .unwrap();
        let result = b"test";

        let result_box: Buf = Box::new(*result);

        println!("result:{}",data);
        println!("=====>");
        // Ok(result_box)
        // .await
    };
    let result = run_basic(fut1);
}
