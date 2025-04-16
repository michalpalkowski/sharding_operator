#!/bin/bash

cargo run --release -- \
  --contract-address 0x038785c9c4f93101a6f1ca968ed253240718b5f77f969cf505d293e96ff1d249 \
  --proxy-address 0x06f1491009c80bc09ff7e14efbd0ad94d1842fe4887b4d89287b9d5781d78e1b \
  --start-block 1 \
  --rpc-url "http://localhost:5050" \
  --event-name "GameFinished" \
  --atlantic-key "" \
  --account-address  \
  --account-private-key 
