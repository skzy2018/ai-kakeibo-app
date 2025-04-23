CREATE TABLE transaction_tags (
    transaction_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY(transaction_id, tag_id),
    FOREIGN KEY(transaction_id) REFERENCES transactions(transaction_id),
    FOREIGN KEY(tag_id) REFERENCES tags(tag_id)
);

