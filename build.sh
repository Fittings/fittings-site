#!/bin/bash

mkdir -p ./build


#Build Server
cd fittings-server
cargo install --force
cp .env ../build/.env
cd ..


#Build Client
cd fittings-client
yarn build

#Move Client to dist.
cd ..
cp -R fittings-client/build/* ./build


