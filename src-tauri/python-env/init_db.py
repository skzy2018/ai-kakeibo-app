#!/usr/bin/env python
import os
import sqlite3
from pathlib import Path

def init_database():
    # Create the database directory if it doesn't exist
    db_dir = Path('../../data/db')
    db_dir.mkdir(parents=True, exist_ok=True)
    
    # Connect to the SQLite database (creates it if it doesn't exist)
    db_path = db_dir / 'database.sqlite'
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    # Read and execute all DDL files
    ddl_dir = Path('../../data/ddl')
    ddl_files = [
        'accounts.sql',
        'categories.sql',
        'data_logs.sql',
        'tags.sql',
        'transactions.sql',
        'transaction_tags.sql'
    ]
    
    for ddl_file in ddl_files:
        print(f"Executing DDL file: {ddl_file}")
        with open(ddl_dir / ddl_file, 'r') as f:
            sql = f.read()
            cursor.execute(sql)
    
    # Commit the changes and close the connection
    conn.commit()
    conn.close()
    
    print(f"Database initialized at {db_path.absolute()}")

if __name__ == '__main__':
    init_database()
