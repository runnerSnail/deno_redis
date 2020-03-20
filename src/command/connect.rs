use crate::*;

#[derive(Deserialize, Debug)]
struct ConnectArgs {
    host: Option<String>,
    port: Option<String>,
    db: Option<u32>,
}


pub fn get_connection(command: Command) -> CoreOp {
    let args:ConnectArgs = serde_json::from_slice(command.data.unwrap().as_ref()).unwrap();
    let url = format!("redis://{}:{}/{}", args.host.unwrap(), args.port.unwrap(), args.db.unwrap());
    let client = redis::Client::open(url).unwrap();
    let client_id:usize = CLIENT_ID.fetch_add(1,Ordering::SeqCst);
    CLIENTS.lock().unwrap().insert(client_id,client);
    CoreOp::Sync(Buf::from(client_id.to_string().as_bytes()))
}
