# jsonrpsee-server-template
json rpc http server based on jsonrpsee, with PostgreSQL integrated

# usage
## server
```shell
DATABASE_URL="postgresql://solana:jw8s0F4@127.0.0.1/solana"
HOST="127.0.0.1"
PORT="3000"
cargo run --bin jsonrpsee-server-template
```

## client
### 1. ping
```shell
curl --location --request POST 'http://127.0.0.1:3000' \
--header 'Content-Type: application/json' \
--data-raw '{"jsonrpc": "2.0", "method": "ping", "id": 2}'
```

response
```json
{"jsonrpc":"2.0","result":"pong","id":2}
```

### 2. get largest verified slot, max_id
```shell
curl --location --request POST 'http://127.0.0.1:3000' \
--header 'Content-Type: application/json' \
--data-raw '{"jsonrpc": "2.0", "method": "get_slot_height", "id": 2}'
```

response
```json
{"jsonrpc":"2.0","result":6,"id":2}
```

### 3. get slot verification status(slot, root_hash, verify_status)
```shell
curl --location --request POST 'http://127.0.0.1:3000' \
--header 'Content-Type: application/json' \
--data-raw '{"jsonrpc": "2.0", "method": "get_slot_status", "id": 2, "params": [6]}'
```

response
```json
{"slot":6,"root_hash":"57c6a48d5294a67580d251db7f18e7dab277792413912465868cb8238691b885","verify_status":0},"id":2}
```
