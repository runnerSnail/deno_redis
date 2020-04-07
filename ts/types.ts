export enum CommandType {
    ConnectWithOptions = "ConnectWithOptions",
    Set = "Set",
    Get = "Get",
    Hset = "Hset",
    Hget = "Hget",
    Cmd = "cmd",
}

export interface ClientOptions {
    host?: string;
    port?: number;
    db?: number;
}


export interface Command {
    command_type: CommandType;
    client_id?: number;
    command_id?: number;
}
