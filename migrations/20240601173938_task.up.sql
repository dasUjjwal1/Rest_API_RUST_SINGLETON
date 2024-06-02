-- Add up migration script here
CREATE TABLE IF NOT EXISTS tbl_task (
    id BIGINT GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    "description" TEXT,
    is_completed BOOLEAN DEFAULT FALSE,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    modified_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    PRIMARY KEY (id, created_at),
    FOREIGN KEY(user_id) REFERENCES tbl_user(id)
 ) PARTITION BY RANGE(created_at);

CREATE INDEX IF NOT EXISTS task_idx ON tbl_task (user_id);

CREATE TABLE tbl_task_06_2024 PARTITION OF tbl_task
    FOR VALUES FROM ('2024-06-01') TO ('2024-07-01'); --PG CRON/TOKIO SCHEDULER WILL DO THE JOB 
