import { ClientOptions, CommandType } from "./types.ts";
import { dispatch, encode, decode } from "./operator.ts";

export class RedisClient {
    private _id: number = 0;

    get client_id() {
        return this._id;
    }

    get_connection(options: ClientOptions) {
        const data = dispatch(
            { command_type: CommandType.ConnectWithOptions },
            encode(JSON.stringify(options))
        );
        console.log('data',decode(data));
        this._id = parseInt(decode(data));
    }
}