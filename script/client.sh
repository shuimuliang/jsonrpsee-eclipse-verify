curl --location --request POST 'http://127.0.0.1:3000' \
--header 'Content-Type: application/json' \
--data-raw '{"jsonrpc": "2.0", "method": "ping", "id": 2}'

curl --location --request POST 'http://127.0.0.1:3000' \
--header 'Content-Type: application/json' \
--data-raw '{"jsonrpc": "2.0", "method": "get_slot_height", "id": 2}'

curl --location --request POST 'http://127.0.0.1:3000' \
--header 'Content-Type: application/json' \
--data-raw '{"jsonrpc": "2.0", "method": "get_slot_status", "id": 2, "params": [6]}'
