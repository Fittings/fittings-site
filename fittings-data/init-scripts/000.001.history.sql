CREATE TABLE update_history(
                  major_version INTEGER NOT NULL,
                  minor_version INTEGER NOT NULL,
                  description STRING,
                  script_type TEXT NOT NULL CHECK (script_type IN ('INIT', 'UPDATE')),
                  script_name TEXT NOT NULL,
                  run_time INTEGER NOT NULL,

                  PRIMARY KEY (major_version, minor_version, script_type)
);
