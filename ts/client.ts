import { ClientOptions, CommandType } from "./types.ts";
import { dispatch, encode, decode } from "./operator.ts";
import { Collection } from "./collection.ts";

export class RedisClient {
    private _id: number = 0;

    get client_id() {
        return this._id;
    }

    public establishConnection(options: ClientOptions) {
        const data = dispatch(
            { command_type: CommandType.ConnectWithOptions },
            encode(JSON.stringify(options))
        );
        // console.log('data', decode(data));
        this._id = parseInt(decode(data));
    }

    public getConenction() {
        return new Collection(this._id);
    }

}