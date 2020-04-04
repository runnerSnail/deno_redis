import { RedisClient } from "./client.ts";
import { CommandType } from "./types.ts";
import { dispatchAsync, encode } from "./operator.ts";

export class Collection {

  constructor(
    private readonly _id: number,
  ) { }
  public async set(key: string, value: string): Promise<any> {
    const doc = await dispatchAsync(
      {
        command_type: CommandType.Cmd,
        client_id: this._id,
      },
      encode(
        JSON.stringify({
          key,
          value,
        })
      )
    );
    return doc;
  }
}
