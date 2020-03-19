#[macro_use]
extern crate deno_core;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;
extern crate bson;
extern crate redis;
extern crate serde;

use deno_core::CoreOp;
use deno_core::PluginInitContext;
use deno_core::{Buf, ZeroCopyBuf};
use futures::FutureExt;
use redis::Client;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

use std::{collections::HashMap, sync::Mutex, sync::MutexGuard};

// mod commond
// mod util

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<usize, Client>> = Mutex::new(HashMap::new());
    static ref CLIENT_ID: AtomicUsize = AtomicUsize::new(0);
}

#[derive(Serialize, Deserialize)]
pub enum CommandType {
    ConnectWithOptions,
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

// init_fn!(init);

// fn init(context: &mut dyn PluginInitContext) {
//     context.register_op("mongo_command", Box::new(op_command));
// }

// fn op_command(data: &[u8], zero_copy: Option<ZeroCopyBuf>) -> CoreOp {
//     let args = CommandArgs::new(data);
//     let executor = match args.command_type {

//     };

//     executor(Command::new(args, zero_copy))
// }
