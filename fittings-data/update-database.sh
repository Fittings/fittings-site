#!/usr/bin/env bash

type sqlite3 >/dev/null 2>&1 || { echo >&2 "Failed to begin script: sqlite3 is not installed."; exit 1; }

if [ ! -f /dist/fittings.db ]; then
    echo "Creating fittings.db for the first time."
    mkdir -p dist
    cd dist
    sqlite3 fittings.db < ./init-scripts/schema-history.sql
    cd ..
fi

echo "Running update scripts."

cd update-scripts
for i in $(ls *.sql | sort -V)
do
  major=$(echo $i | awk -F"." '{ print $1 }')
  minor=$(echo $i | awk -F"." '{ print $2 }')
  desc=$(echo $i | awk -F"." '{ print $3 }')
  # cmd [option] "$file" >> results.out
done
cd ..
