#!/bin/bash

# Variables
RPC_URL="http://localhost:50456"   # Replace with your RPC server URL
ADDRESS="jkl12g4qwenvpzqeakavx5adqkw203s629tf6k8vdg"              # Replace with the actual Cosmos address

# JSON-RPC Request Payload
read -r -d '' JSON_PAYLOAD << EOM
{
  "jsonrpc": "2.0",
  "method": "account",
  "params": {
    "address": "$ADDRESS"
  },
  "id": 1
}
EOM

# Send the JSON-RPC request using curl
curl -s -X POST $RPC_URL \
    -H "Content-Type: application/json" \
    -d "$JSON_PAYLOAD"
