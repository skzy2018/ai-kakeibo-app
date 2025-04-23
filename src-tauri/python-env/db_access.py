#!/usr/bin/env python
import sqlite3
import pandas as pd
from pathlib import Path
import json
import datetime

class DatabaseManager:
    def __init__(self):
        db_dir = Path('../../data/db')
        self.db_path = db_dir / 'database.sqlite'
        self._conn = None
    
    def connect(self):
        """Connect to the SQLite database."""
        if not self._conn:
            self._conn = sqlite3.connect(self.db_path)
            # Enable foreign keys
            self._conn.execute("PRAGMA foreign_keys = ON")
            # Configure SQLite to return rows as dictionaries
            self._conn.row_factory = sqlite3.Row
        return self._conn
    
    def disconnect(self):
        """Close the database connection."""
        if self._conn:
            self._conn.close()
            self._conn = None
    
    def execute_query(self, query, params=None):
        """Execute a query and return the results as a list of dictionaries."""
        conn = self.connect()
        cursor = conn.cursor()
        
        if params:
            cursor.execute(query, params)
        else:
            cursor.execute(query)
            
        results = [dict(row) for row in cursor.fetchall()]
        return results
    
    def execute_query_as_df(self, query, params=None):
        """Execute a query and return the results as a pandas DataFrame."""
        conn = self.connect()
        
        if params:
            df = pd.read_sql_query(query, conn, params=params)
        else:
            df = pd.read_sql_query(query, conn)
            
        return df
    
    def execute_update(self, query, params=None):
        """Execute an update query (INSERT, UPDATE, DELETE)."""
        conn = self.connect()
        cursor = conn.cursor()
        
        if params:
            cursor.execute(query, params)
        else:
            cursor.execute(query)
            
        conn.commit()
        return cursor.rowcount
    
    def insert_record(self, table, data):
        """Insert a record into a table.
        
        Args:
            table (str): The name of the table.
            data (dict): A dictionary of column names and values.
        
        Returns:
            int: The ID of the inserted record.
        """
        columns = ', '.join(data.keys())
        placeholders = ', '.join(['?' for _ in data])
        values = list(data.values())
        
        query = f"INSERT INTO {table} ({columns}) VALUES ({placeholders})"
        
        conn = self.connect()
        cursor = conn.cursor()
        cursor.execute(query, values)
        conn.commit()
        
        return cursor.lastrowid
    
    def get_accounts(self):
        """Get all accounts."""
        return self.execute_query("SELECT * FROM accounts ORDER BY name")
    
    def get_categories(self):
        """Get all categories."""
        return self.execute_query("SELECT * FROM categories ORDER BY name")
    
    def get_tags(self):
        """Get all tags."""
        return self.execute_query("SELECT * FROM tags ORDER BY name")
    
    def get_transactions(self, limit=100, offset=0):
        """Get transactions with pagination."""
        query = """
        SELECT t.*, a.name as account_name, c.name as category_name 
        FROM transactions t
        JOIN accounts a ON t.account_id = a.account_id
        JOIN categories c ON t.category_id = c.category_id
        ORDER BY t.transaction_date DESC
        LIMIT ? OFFSET ?
        """
        return self.execute_query(query, (limit, offset))
    
    def get_transaction_tags(self, transaction_id):
        """Get tags for a transaction."""
        query = """
        SELECT t.* FROM tags t
        JOIN transaction_tags tt ON t.tag_id = tt.tag_id
        WHERE tt.transaction_id = ?
        ORDER BY t.name
        """
        return self.execute_query(query, (transaction_id,))
    
    def json_serializer(self, obj):
        """JSON serializer for objects not serializable by default json code."""
        if isinstance(obj, (datetime.datetime, datetime.date)):
            return obj.isoformat()
        raise TypeError(f"Type {type(obj)} not serializable")
    
    def execute_sql_component(self, sql):
        """Execute a SQL component and return the results as JSON."""
        try:
            df = self.execute_query_as_df(sql)
            return json.dumps({
                "success": True,
                "data": df.to_dict(orient="records"),
                "columns": df.columns.tolist()
            }, default=self.json_serializer)
        except Exception as e:
            return json.dumps({
                "success": False,
                "error": str(e)
            })

# Create a global instance for easy access
db = DatabaseManager()

# Example functions that can be called from Rust/Tauri
def execute_sql(sql):
    return db.execute_sql_component(sql)

def get_accounts():
    accounts = db.get_accounts()
    return json.dumps(accounts, default=db.json_serializer)

def get_categories():
    categories = db.get_categories()
    return json.dumps(categories, default=db.json_serializer)

def get_transactions(limit=100, offset=0):
    transactions = db.get_transactions(limit, offset)
    return json.dumps(transactions, default=db.json_serializer)

def add_transaction(account_id, category_id, amount, description, transaction_date, memo="", tags=None):
    conn = db.connect()
    cursor = conn.cursor()
    
    try:
        cursor.execute("BEGIN TRANSACTION")
        
        # Insert transaction
        transaction_data = {
            "account_id": account_id,
            "category_id": category_id,
            "amount": amount,
            "description": description,
            "transaction_date": transaction_date,
            "memo": memo
        }
        transaction_id = db.insert_record("transactions", transaction_data)
        
        # Add tags if provided
        if tags and isinstance(tags, list):
            for tag_id in tags:
                cursor.execute(
                    "INSERT INTO transaction_tags (transaction_id, tag_id) VALUES (?, ?)",
                    (transaction_id, tag_id)
                )
        
        conn.commit()
        return json.dumps({"success": True, "transaction_id": transaction_id})
    
    except Exception as e:
        conn.rollback()
        return json.dumps({"success": False, "error": str(e)})
    
def import_from_csv(csv_file, mapping):
    """
    Import transactions from a CSV file
    
    Args:
        csv_file (str): Path to the CSV file
        mapping (dict): A dictionary mapping CSV columns to database fields
    
    Returns:
        str: JSON string with success/error information
    """
    try:
        # Load CSV into DataFrame
        df = pd.read_csv(csv_file)
        
        # Process according to mapping
        # ...
        
        return json.dumps({"success": True, "records_imported": len(df)})
    except Exception as e:
        return json.dumps({"success": False, "error": str(e)})
