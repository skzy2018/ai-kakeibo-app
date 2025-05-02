#!/usr/bin/env python
import sqlite3
import pandas as pd
from pathlib import Path
import json
import datetime
import os
import csv
import shutil

class DatabaseManager:
    def __init__(self):
        # Use absolute path to the database file
        # Get the directory where the script is located
        script_dir = Path(os.path.dirname(os.path.abspath(__file__)))
        # Navigate to the data/db directory from script location
        # Assuming structure: src-tauri/python-env/db_access.py and data/db at project root
        db_dir = script_dir.parent.parent / 'data' / 'db'
        db_dir.mkdir(parents=True, exist_ok=True)  # Ensure the directory exists
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

    def insert_record_withCur_notCommit(self, cursor, table, data):
        """Insert a record into a table. but don't commit and use cursor.
        
        Args:
            cursor (cursor): database cursor
            table (str): The name of the table.
            data (dict): A dictionary of column names and values.
        
        Returns:
            int: The ID of the inserted record.
        """
        columns = ', '.join(data.keys())
        placeholders = ', '.join(['?' for _ in data])
        values = list(data.values())
        
        query = f"INSERT INTO {table} ({columns}) VALUES ({placeholders})"
        cursor.execute(query, values)
        return cursor.lastrowid

    def delete_record(self, table, condition_column, condition_value):
        """Delete a record from a table.
        
        Args:
            table (str): The name of the table
            condition_column (str): Column name for the condition
            condition_value: Value for the condition
            
        Returns:
            int: Number of rows affected
        """
        query = f"DELETE FROM {table} WHERE {condition_column} = ?"
        
        conn = self.connect()
        cursor = conn.cursor()
        cursor.execute(query, (condition_value,))
        conn.commit()
        
        return cursor.rowcount
    
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
    
    def add_account(self, name, account_type, currency="JPY"):
        """Add a new account."""
        data = {
            "name": name,
            "account_type": account_type,
            "currency": currency
        }
        try:
            account_id = self.insert_record("accounts", data)
            return {"success": True, "account_id": account_id}
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    def add_category(self, name, type):
        """Add a new category."""
        data = {
            "name": name,
            "type": type
        }
        try:
            category_id = self.insert_record("categories", data)
            return {"success": True, "category_id": category_id}
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    def add_tag(self, name):
        """Add a new tag."""
        data = {
            "name": name
        }
        try:
            tag_id = self.insert_record("tags", data)
            return {"success": True, "tag_id": tag_id}
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    def delete_account(self, account_id):
        """Delete an account."""
        try:
            self.delete_record("accounts", "account_id", account_id)
            return {"success": True}
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    def delete_category(self, category_id):
        """Delete a category."""
        try:
            self.delete_record("categories", "category_id", category_id)
            return {"success": True}
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    def delete_tag(self, tag_id):
        """Delete a tag."""
        try:
            self.delete_record("tags", "tag_id", tag_id)
            return {"success": True}
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    def get_csv_files(self):
        """Get list of CSV files in data/csv/ directory."""
        script_dir = Path(os.path.dirname(os.path.abspath(__file__)))
        csv_dir = script_dir.parent.parent / 'data' / 'csv'
        if not csv_dir.exists():
            csv_dir.mkdir(parents=True, exist_ok=True)
        
        files = [f.name for f in csv_dir.glob('*.csv')]
        return files
    
    def load_csv_file(self, filename):
        """Load data from a CSV file into the database.
        
        Args:
            filename (str): The name of the CSV file (without path)
            
        Returns:
            dict: Result of the operation
        """
        script_dir = Path(os.path.dirname(os.path.abspath(__file__)))
        csv_path = script_dir.parent.parent / 'data' / 'csv' / filename
        dust_dir = script_dir.parent.parent / 'data' / 'dust'
        
        if not csv_path.exists():
            return {"success": False, "error": f"File not found: {filename}"}
        
        if not dust_dir.exists():
            dust_dir.mkdir(parents=True, exist_ok=True)
        
        # Parse collector and date from filename
        try:
            # Filename format: {data_collector}_{update_date}.csv
            parts = filename.split('_')
            if len(parts) < 2:
                return {"success": False, "error": "Invalid filename format"}
            
            data_collector = parts[0]
            update_date = '_'.join(parts[1:]).replace('.csv', '')
            
            conn = self.connect()
            cursor = conn.cursor()
            
            try:
                # Begin transaction
                cursor.execute("BEGIN TRANSACTION")
                
                # Insert into data_logs
                log_data = {
                    "data_collector": data_collector,
                    "update_date": update_date
                }
                #log_id = self.insert_record("data_logs", log_data)
                log_id = self.insert_record_withCur_notCommit(cursor,"data_logs", log_data)
                
                # Read CSV file
                transactions_inserted = 0
                tags_inserted = 0
                
                with open(csv_path, 'r', encoding='utf-8') as csvfile:
                    reader = csv.reader(csvfile)
                    headers = next(reader)  # Skip header row
                    
                    for row in reader:
                        if len(row) < 5:  # Ensure there's at least date, account, category_type, category_name, amount
                            continue
                        
                        transaction_date = row[0].strip()
                        account_name = row[1].strip()
                        category_type = row[2].strip()
                        category_name = row[3].strip()
                        amount = float(row[4].strip())
                        
                        # Optional fields
                        item_name = row[5].strip() if len(row) > 5 else ""
                        
                        # Parse tags from the 7th column if it exists
                        tags = []
                        if len(row) > 6 and row[6].strip():
                            tags_str = row[6].strip()
                            # Remove brackets if present
                            if tags_str.startswith('[') and tags_str.endswith(']'):
                                tags_str = tags_str[1:-1]
                            # Split by pipe
                            tags = [tag.strip() for tag in tags_str.split('|')]
                        
                        description = row[7].strip() if len(row) > 7 else ""
                        memo = row[8].strip() if len(row) > 8 else ""
                        
                        # Get account_id from accounts table, or create if not exists
                        cursor.execute("SELECT account_id FROM accounts WHERE name = ?", (account_name,))
                        result = cursor.fetchone()
                        if result:
                            account_id = result['account_id']
                        else:
                            account_data = {
                                "name": account_name,
                                "account_type": "その他"  # Default type
                            }
                            #account_id = self.insert_record("accounts", account_data)
                            account_id = self.insert_record_withCur_notCommit(cursor, "accounts", account_data)
                        
                        # Get category_id from categories table, or create if not exists
                        cursor.execute("SELECT category_id FROM categories WHERE name = ? AND type = ?", 
                                      (category_name, category_type))
                        result = cursor.fetchone()
                        if result:
                            category_id = result['category_id']
                        else:
                            category_data = {
                                "name": category_name,
                                "type": category_type
                            }
                            #category_id = self.insert_record("categories", category_data)
                            category_id = self.insert_record_withCur_notCommit(cursor, "categories", category_data)
                        
                        # Insert transaction
                        transaction_data = {
                            "account_id": account_id,
                            "category_id": category_id,
                            "log_id": log_id,
                            "amount": amount,
                            "item_name": item_name,
                            "description": description,
                            "transaction_date": transaction_date,
                            "memo": memo
                        }
                        #transaction_id = self.insert_record("transactions", transaction_data)
                        transaction_id = self.insert_record_withCur_notCommit(cursor, "transactions", transaction_data)
                        transactions_inserted += 1
                        
                        # Process tags
                        for tag_name in tags:
                            if not tag_name:
                                continue
                            
                            # Get tag_id from tags table, or create if not exists
                            cursor.execute("SELECT tag_id FROM tags WHERE name = ?", (tag_name,))
                            result = cursor.fetchone()
                            if result:
                                tag_id = result['tag_id']
                            else:
                                tag_data = {"name": tag_name}
                                #tag_id = self.insert_record("tags", tag_data)
                                tag_id = self.insert_record_withCur_notCommit(cursor, "tags", tag_data)
                            
                            # Add to transaction_tags
                            cursor.execute(
                                "INSERT INTO transaction_tags (transaction_id, tag_id) VALUES (?, ?)",
                                (transaction_id, tag_id)
                            )
                            tags_inserted += 1
                
                # Commit the transaction
                conn.commit()
                
                # Move the file to dust directory
                shutil.move(str(csv_path), str(dust_dir / filename))
                
                return {
                    "success": True, 
                    "transactions_inserted": transactions_inserted,
                    "tags_inserted": tags_inserted,
                    "log_id": log_id,
                    "data_collector": data_collector,
                    "update_date": update_date
                }
                
            except Exception as e:
                conn.rollback()
                return {"success": False, "error": str(e)}
                
        except Exception as e:
            return {"success": False, "error": str(e)}
    
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
def execute_query(sql):
    return db.execute_query(sql)

def execute_sql(sql):
    return db.execute_sql_component(sql)

def get_accounts():
    accounts = db.get_accounts()
    return json.dumps(accounts, default=db.json_serializer)

def get_categories():
    categories = db.get_categories()
    return json.dumps(categories, default=db.json_serializer)

def get_tags():
    tags = db.get_tags()
    return json.dumps(tags, default=db.json_serializer)

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

# Master table management functions
def add_account(name, account_type, currency="JPY"):
    result = db.add_account(name, account_type, currency)
    return json.dumps(result, default=db.json_serializer)

def add_category(name, type):
    result = db.add_category(name, type)
    return json.dumps(result, default=db.json_serializer)

def add_tag(name):
    result = db.add_tag(name)
    return json.dumps(result, default=db.json_serializer)

def delete_account(account_id):
    result = db.delete_account(account_id)
    return json.dumps(result, default=db.json_serializer)

def delete_category(category_id):
    result = db.delete_category(category_id)
    return json.dumps(result, default=db.json_serializer)

def delete_tag(tag_id):
    result = db.delete_tag(tag_id)
    return json.dumps(result, default=db.json_serializer)

# CSV file management functions
def get_csv_files():
    files = db.get_csv_files()
    return json.dumps(files, default=db.json_serializer)

def load_csv_file(filename):
    result = db.load_csv_file(filename)
    return json.dumps(result, default=db.json_serializer)

# SQL component management functions
def save_sql_component(component):
    """Save a SQL component to a JSON file.
    
    Args:
        component (dict): The SQL component object
        
    Returns:
        dict: Result of the operation
    """
    try:
        # Get the SQL component name
        name = component.get('name')
        if not name:
            return {"success": False, "error": "SQL component name is required"}
        
        # Validate component name (only alphanumeric and no spaces)
        import re
        name_regex = re.compile(r'^[A-Za-z0-9_-]+$')
        if not name_regex.match(name):
            return {"success": False, "error": "Component name should only contain alphanumeric characters, underscores, and hyphens"}
        
        # Get the directory where the script is located
        script_dir = Path(os.path.dirname(os.path.abspath(__file__)))
        # Navigate to the data/component directory from script location
        component_dir = script_dir.parent.parent / 'data' / 'component'
        component_dir.mkdir(parents=True, exist_ok=True)  # Ensure the directory exists
        
        # File path for the component
        component_path = component_dir / f"{name}.json"
        
        # Sanitize the input data to ensure there are no control characters
        # For d3code and sql which may contain JS/SQL code with indentation
        if 'd3code' in component and component['d3code'] is not None:
            component['d3code'] = component['d3code'].replace('\t', '  ')  # Replace tabs with spaces
        
        if 'sql' in component and component['sql'] is not None:
            component['sql'] = component['sql'].replace('\t', '  ')  # Replace tabs with spaces
        
        # Write the component to file
        with open(component_path, 'w', encoding='utf-8') as f:
            json.dump(component, f, ensure_ascii=False, indent=2, default=db.json_serializer)
        
        return {"success": True, "message": f"SQL component '{name}' saved successfully"}
    except Exception as e:
        return {"success": False, "error": str(e)}

def get_sql_components():
    """Get all SQL components.
    
    Returns:
        list: List of SQL component names
    """
    try:
        # Get the directory where the script is located
        script_dir = Path(os.path.dirname(os.path.abspath(__file__)))
        # Navigate to the data/component directory from script location
        component_dir = script_dir.parent.parent / 'data' / 'component'
        
        if not component_dir.exists():
            component_dir.mkdir(parents=True, exist_ok=True)
            return []
        
        # Get all JSON files in the component directory
        component_files = list(component_dir.glob('*.json'))
        
        # Extract component names from file names
        components = []
        for file in component_files:
            try:
                with open(file, 'r', encoding='utf-8') as f:
                    component = json.load(f)
                    components.append({
                        "name": component.get('name', file.stem),
                        "description": component.get('description', '')
                    })
            except Exception as e:
                print(f"Error loading component {file}: {str(e)}")
        
        return components
    except Exception as e:
        print(f"Error getting SQL components: {str(e)}")
        return []

def get_sql_component(name):
    """Get a specific SQL component by name.
    
    Args:
        name (str): The name of the SQL component
        
    Returns:
        dict: The SQL component
    """
    try:
        # Get the directory where the script is located
        script_dir = Path(os.path.dirname(os.path.abspath(__file__)))
        # Navigate to the data/component directory from script location
        component_dir = script_dir.parent.parent / 'data' / 'component'
        
        # File path for the component
        component_path = component_dir / f"{name}.json"
        
        if not component_path.exists():
            return {"success": False, "error": f"SQL component '{name}' not found"}
        
        # Read the component from file
        with open(component_path, 'r', encoding='utf-8') as f:
            component = json.load(f)
        
        return {"success": True, "component": component}
    except Exception as e:
        return {"success": False, "error": str(e)}


def delete_sql_component(name):
    """Delete a specific SQL component by name.
    
    Args:
        name (str): The name of the SQL component
        
    Returns:
        dict: {"success": True, ( "error": str ) if error}
    """
    try:
        # Get the directory where the script is located
        script_dir = Path(os.path.dirname(os.path.abspath(__file__)))
        # Navigate to the data/component directory from script location
        component_dir = script_dir.parent.parent / 'data' / 'component'

        # File path for the component
        component_path = component_dir / f"{name}.json"

        if not component_path.exists():
            return {"success": False, "error": f"SQL component '{name}' not found"}

        os.remove(component_path)
        return {"success": True}

    except Exception as e:
        return {"success": False, "error": str(e)}


def run_sql_component(name, env_vars=None):
    """Run a SQL component with environment variables.
    
    Args:
        name (str): The name of the SQL component
        env_vars (dict): Environment variables for the SQL
        
    Returns:
        dict: Result of the operation
    """
    try:
        # Get the SQL component
        component_result = get_sql_component(name)
        if not component_result.get("success", False):
            return json.dumps(component_result)
        
        component = component_result.get("component", {})
        
        # Get the SQL from the component
        sql = component.get("sql", "")
        if not sql:
            return json.dumps({"success": False, "error": "SQL is required"})
        
        # Replace environment variables in the SQL
        if env_vars:
            for var_name, var_value in env_vars.items():
                sql = sql.replace(f"${var_name}", str(var_value))
        
        # Run the SQL
        try:
            df = db.execute_query_as_df(sql)
            
            # Convert DataFrame to JSON
            return json.dumps({
                "success": True,
                "data": df.to_dict(orient="records"),
                "columns": df.columns.tolist(),
                "component": component
            }, default=db.json_serializer)
        except Exception as e:
            return json.dumps({
                "success": False,
                "error": f"SQL execution error: {str(e)}",
                "component": component
            })
    except Exception as e:
        return json.dumps({"success": False, "error": str(e)})
