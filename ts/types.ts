export enum CommandType {
    ConnectWithOptions = "ConnectWithOptions",
    Cmd = "Cmd"
}

export interface ClientOptions {
    host?:string;
    port?:number;
    db?:number;
}


export interface Command {
    command_type: CommandType;
    client_id?: number;
    command_id?: number;
}
