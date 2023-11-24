CREATE TABLE users(
    user_id uuid NOT NULL,
    PRIMARY KEY(user_id),
    user_name TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    account_created_on TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);

CREATE TABLE lists(
    list_id uuid NOT NULL,
    PRIMARY KEY(list_id),
    name TEXT NOT NULL,
    created_by_user_id uuid NOT NULL REFERENCES users (user_id),
    created_date TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    is_daily_task_list BOOLEAN NOT NULL,
    description TEXT
);

CREATE TABLE tasks(
    task_id uuid NOT NULL,
    PRIMARY KEY(task_id),
    name TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_date TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    has_child_tasks BOOLEAN NOT NULL,
    created_by_user_id uuid NOT NULL REFERENCES users (user_id),
    parent_list_id uuid NOT NULL REFERENCES lists (list_id),
    completed_by_user_id uuid,
    notes TEXT,
    due_date TIMESTAMPTZ
);