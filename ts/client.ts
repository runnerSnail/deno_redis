import { ClientOptions, CommandType } from "./types.ts";
import { dispatch, encode, decode } from "./operator.ts";
import { Collection } from "./collection.ts";
import { assert, assertEquals } from "https://deno.land/std/testing/asserts.ts";


export class RedisClient {

    private _id: number = 0;
    private options: ClientOptions;
    public connection:Collection; 
    get client_id() {
        return this._id;
    }
    constructor(options: ClientOptions) {
        this.options = options;
        this.connection = this.setClient();
    }

    setClient(): Collection {
        if (this.options) {
            const data = dispatch(
                { command_type: CommandType.ConnectWithOptions },
                encode(JSON.stringify(this.options))
            );
            this._id = parseInt(decode(data));
        } else {
            assert(this._id != 0, "no create Client");
        }
        return new Collection(this._id);
    }

}