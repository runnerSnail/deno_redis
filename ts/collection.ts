import { RedisClient } from "./client.ts";
import { CommandType } from "./types.ts";
import { dispatchAsync, encode } from "./operator.ts";

type fileds = string[];

export class Collection {

  constructor(
    private readonly _id: number,
  ) { }
  public async set(key: string, value: string): Promise<any> {
    const data = await dispatchAsync(
      {
        command_type: CommandType.Set,
        client_id: this._id,
      },
      encode(
        JSON.stringify({
          key,
          value,
        })
      )
    );
    return data;
  }

  public async get(key: string): Promise<any> {
    const data = await dispatchAsync(
      {
        command_type: CommandType.Get,
        client_id: this._id,
      },
      encode(
        JSON.stringify({
          key,
        })
      )
    );
    return data;
  }

  public async hmset(key: string, fileds: fileds): Promise<any> {
    const data = await dispatchAsync(
      {
        command_type: CommandType.Hset,
        client_id: this._id,
      },
      encode(
        JSON.stringify({
          key,
          fileds
        })
      )
    );
    return data;
  }

  public async hmget(key: string, filed: string): Promise<any> {
    const data = await dispatchAsync(
      {
        command_type: CommandType.Hget,
        client_id: this._id,
      },
      encode(
        JSON.stringify({
          key,
          filed
        })
      )
    );
    return data;
  }

}
