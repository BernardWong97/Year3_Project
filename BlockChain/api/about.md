## This is and api crate for the blokchain app.

#### End points:
**GET**
- `#[get("/messages")]`: get all available messages.
- `#[get("/chin_info")]`: get blockchain info

**Post**
- `#[post("/message")]`: send message

**GET messages**:
```JSON
[{
    "message": {
        "chain":"global chain id",
        "block":"block_id",
        "from":"senders_id",
        "to":"receivers_id",
        "timestamp":"timestamp",
        "size":"message size in Bytes",
        "content":"Hello, here is message!",
    }
}]
```

**GET chain_info**:
```JSON
{
    "timestamp":"timestamp",
    "user":"user id",
    "user_balance":"users balance",
    "chain_uuid":"blockchain universally unique ID",
    "current_block":"chains latest block id",
    "nodes":"currently connected nodes",
    "transactions":"pending transactions",
    "miner":{
        "block":"block currently mining",
        "nonce":"nonce: iteration count",
        "time_spend":"time spend on current block",
        "speed":"miner speed hash/sec",
    }  

}
```

**POST message**:
```JSON
{
    "to":"receiver",
    "timestamp":"timestamp",
    "size":"message size in Bytes",
    "content":"message content",
}
```
