CREATE TABLE transactions (
    transaction_id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    log_id INTEGER ,
    amount REAL NOT NULL,             -- 支出は負数、収入は正数
    item_name TEXT,
    description TEXT,
    transaction_date DATETIME NOT NULL,
    memo TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(account_id) REFERENCES accounts(account_id),
    FOREIGN KEY(category_id) REFERENCES categories(category_id),
    FOREIGN KEY(log_id) REFERENCES data_logs(log_id)
);
