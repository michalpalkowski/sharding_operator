#!/bin/bash

cargo run --release -- \
  --contract-address 0x02249d24dfbe04e84e796f2d0de7f280196720bb21fa040a6b34b9dac92b919b \
  --proxy-address 0x07d1a17d7881adb769f7870bd670da293532a34e88f8c512bd9e59c60a5e2345 \
  --start-block 1 \
  --rpc-url "http://localhost:5050" \
  --event-name "GameFinished" \
  --atlantic-key "" \
  --account-address 0x1f401c745d3dba9b9da11921d1fb006c96f571e9039a0ece3f3b0dc14f04c3d \
  --account-private-key 0x7230b49615d175307d580c33d6fda61fc7b9aec91df0f5c1a5ebe3b8cbfee02
