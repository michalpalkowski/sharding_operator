#!/bin/bash

cargo run -- --contract-address 0x07c8cf43139e1033f8c1031ab5842c14e91e343d81d9c36ae93fc715a68c903c --proxy-address 0x02b4345dc1ec9f1bd29feac3b446b706de84b4d8b9af95d83d3bd12fd6c4278f --start-block 1 --rpc-url http://localhost:5050/ --event-name GameFinished
