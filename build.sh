#!/bin/bash

#Build Server
cd fittings-server
cargo install --force
cd ..

mkdir -p ./build

#Build Client
cd fittings-client
yarn build

#Move Client to dist.
cd ..
cp -R fittings-client/build/* ./build


#Database
cd fittings-data
source ./update-database.sh
