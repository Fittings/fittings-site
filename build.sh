#!/bin/bash

#Build Server
cd fittings-server
cargo build --release

#Move Server to dist.
cd .. 
mkdir -p dist/server/
cp -r fittings-server/target/release/* dist/server/ 

#Build Client
cd fittings-client
yarn build

#Move Client to dist.
cd ..
mkdir -p dist/client/
cp -R fittings-client/dist/* dist/client/