#!/usr/bin/env bash
echo "Starting database initialisation & update."
type sqlite3 >/dev/null 2>&1 || { echo >&2 "Failed to begin script: sqlite3 is not installed."; exit 1; }

if [ ! -f ../build/fittings.db ];
  then
    echo "Creating fittings.db for the first time."
    mkdir -p ../build

    echo "Running update script: 000.001.history.sql" #ZZZ TODO Loop through like the update function.
    sqlite3 ../build/fittings.db < ./init-scripts/000.001.history.sql
    sqlite3 ../build/fittings.db "INSERT INTO update_history(major_version, minor_version, description, script_type, script_name, run_time)
                                VALUES(000, 001, \"history\",  'INIT', \"000.001.history.sql\",  $(date +%s))"
  else
    echo "Skipping initialisation scripts."
fi

echo
echo "Running update scripts."
max_major=$(sqlite3 ../build/fittings.db "select max(major_version) from update_history where script_type='UPDATE'")
max_minor=$(sqlite3 ../build/fittings.db "select max(minor_version) from update_history where script_type='UPDATE'")
cd update-scripts

for script_name in $(ls *.sql | sort -V)
do
  major=$(echo $script_name | awk -F"." '{ print $1 }')
  minor=$(echo $script_name | awk -F"." '{ print $2 }')
  desc=$(echo $script_name | awk -F"." '{ print $3 }')

  if !((major < max_major)) && ((minor > max_minor ));
  then
    if sqlite3 ../../build/fittings.db "INSERT INTO update_history(major_version, minor_version, description, script_type, script_name, run_time)
                                VALUES(${major}, ${minor}, \"${desc}\", \"UPDATE\", \"${i}\", $(date +%s))"
    then
      echo "Success: ${script_name}"
    else
      echo "Failure: ${script_name}"
      exit $?
    fi
  else
    echo "Skipping: ${script_name}"
  fi
  # cmd [option] "$file" >> results.out
done
