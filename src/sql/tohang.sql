CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY not null,
    task TEXT not null,
    done BOOLEAN not null DEFAULT 0
)