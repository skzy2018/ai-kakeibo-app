#!/usr/bin/env python
# -*- coding: utf-8 -*-

import os
import json
import uvicorn
import socket
import signal
import sys
from typing import Dict, List, Optional, Union
from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import db_access


import logging
#logging.basicConfig(level=logging.INFO,handlers=[logging.FileHandler("./api_server.log") ] )
logging.basicConfig(level=logging.INFO,stream=sys.stderr,format='%(asctime)s - %(levelname)s - %(message)s')

LOGGING_CONFIG = {
    "version": 1,
    "disable_existing_loggers": False,
    "formatters": {
        "default": {
            "()": "uvicorn.logging.DefaultFormatter",
            "fmt": "%(levelprefix)s %(asctime)s - %(message)s",
            "datefmt": "%Y-%m-%d %H:%M:%S",
        },
        "access": {
            "()": "uvicorn.logging.AccessFormatter",
            "fmt": '%(levelprefix)s %(asctime)s - %(client_addr)s - "%(request_line)s" %(status_code)s',
            "datefmt": "%Y-%m-%d %H:%M:%S",
        },
    },
    "handlers": {
        "default": {
            "formatter": "default",
            "class": "logging.StreamHandler",
            "stream": "ext://sys.stderr", # Output to stderr
        },
        "access": {
            "formatter": "access",
            "class": "logging.StreamHandler",
            "stream": "ext://sys.stderr", # Output access logs to stderr
        },
    },
    "loggers": {
        "uvicorn": {"handlers": ["default"], "level": "INFO", "propagate": False},
        "uvicorn.error": {"level": "INFO"}, # Use default handler
        "uvicorn.access": {"handlers": ["access"], "level": "INFO", "propagate": False}, # Use access handler
    },
}


# Create FastAPI application
app = FastAPI(title="Kakeibo API Server")

# Add CORS middleware to allow requests from the Tauri frontend
app.add_middleware(
    CORSMiddleware,
    allow_origins=["tauri://localhost", "http://localhost:1420", "http://localhost"],
    allow_credentials=False,  # Changed to False to avoid CORS issues with credentials
    allow_methods=["*"],
    allow_headers=["*"],
)

# Get temp directory for API server files
#def get_temp_dir():
#    temp_dir = os.path.join(os.path.expanduser("~"), ".kakeibo-api-server")
#    os.makedirs(temp_dir, exist_ok=True)
#    return temp_dir

# Create PID file for the API server
#def write_pid_file():
#    pid = os.getpid()
#    temp_dir = get_temp_dir()
#    pid_file = os.path.join(temp_dir, "api_server.pid")
#    with open(pid_file, "w") as f:
#        f.write(str(pid))

# Find an available port
def find_available_port(start_port=8000, max_attempts=100):
    for port in range(start_port, start_port + max_attempts):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            if s.connect_ex(('127.0.0.1', port)) != 0:
                return port
    raise RuntimeError(f"Could not find an available port after {max_attempts} attempts")

# Write server info including port
#def write_server_info(port):
#    temp_dir = get_temp_dir()
#    info_file = os.path.join(temp_dir, "api_server_info.json")
#    with open(info_file, "w") as f:
#        json.dump({"port": port}, f)

# Graceful shutdown handler
def handle_exit(signum, frame):
    print(f"Received signal {signum}, shutting down API server...")
    sys.exit(0)

# Models for API requests and responses
class Account(BaseModel):
    name: str
    account_type: str
    currency: str = "JPY"

class Category(BaseModel):
    name: str
    category_type: str

class Tag(BaseModel):
    name: str

class Transaction(BaseModel):
    account_id: int
    category_id: int
    amount: float
    description: str
    transaction_date: str
    memo: Optional[str] = None
    tags: Optional[List[int]] = None

class SQLComponent(BaseModel):
    name: str
    sql: str
    description: Optional[str] = None
    environment_variables: Optional[Dict[str, str]] = None

# Health check endpoint
@app.get("/health")
async def health_check():
    return {"status": "ok", "version": "1.0.0"}

# Initialize database
@app.post("/init_database")
async def init_database():
    try:
        result = db_access.init_database()
        return {"success": True, "result": result}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Execute custom SQL
@app.post("/execute_sql")
async def execute_sql(sql: str):
    try:
        result = db_access.execute_sql(sql)
        return {"success": True, "result": result}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Get all accounts
@app.get("/accounts")
async def get_accounts():
    try:
        accounts = db_access.get_accounts()
        return json.loads(accounts)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Add new account
@app.post("/accounts")
async def add_account(account: Account):
    try:
        result = db_access.add_account(account.name, account.account_type, account.currency)
        return json.loads(result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Delete account
@app.delete("/accounts/{account_id}")
async def delete_account(account_id: int):
    try:
        result = db_access.delete_account(account_id)
        return json.loads(result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Get all categories
@app.get("/categories")
async def get_categories():
    try:
        categories = db_access.get_categories()
        return json.loads(categories)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Add new category
@app.post("/categories")
async def add_category(category: Category):
    try:
        result = db_access.add_category(category.name, category.category_type)
        return json.loads(result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Delete category
@app.delete("/categories/{category_id}")
async def delete_category(category_id: int):
    try:
        result = db_access.delete_category(category_id)
        return json.loads(result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Get all tags
@app.get("/tags")
async def get_tags():
    try:
        tags = db_access.get_tags()
        return json.loads(tags)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Add new tag
@app.post("/tags")
async def add_tag(tag: Tag):
    try:
        result = db_access.add_tag(tag.name)
        return json.loads(result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Delete tag
@app.delete("/tags/{tag_id}")
async def delete_tag(tag_id: int):
    try:
        result = db_access.delete_tag(tag_id)
        return json.loads(result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Get transactions with pagination
@app.get("/transactions")
async def get_transactions(limit: int = 100, offset: int = 0):
    try:
        transactions = db_access.get_transactions(limit, offset)
        return json.loads(transactions)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Add new transaction
@app.post("/transactions")
async def add_transaction(transaction: Transaction):
    try:
        result = db_access.add_transaction(
            transaction.account_id,
            transaction.category_id,
            transaction.amount,
            transaction.description,
            transaction.transaction_date,
            transaction.memo,
            transaction.tags
        )
        return json.loads(result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Get CSV files
@app.get("/csv_files")
async def get_csv_files():
    try:
        csv_files = db_access.get_csv_files()
        #return json.loads(csv_files)
        return csv_files
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Load CSV file
@app.post("/csv_files/{filename}")
async def load_csv_file(filename: str):
    try:
        result = db_access.load_csv_file(filename)
        return json.loads(result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Get all SQL components
@app.get("/sql_components")
async def get_sql_components():
    try:
        components = db_access.get_sql_components()
        #return json.loads(components)
        return components
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Get a specific SQL component
@app.get("/sql_components/{name}")
async def get_sql_component(name: str):
    try:
        component = db_access.get_sql_component(name)
        #return json.loads(component)
        return component
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Save SQL component
@app.post("/sql_components")
async def save_sql_component(component: dict):
    try:
        #logging.info(f"Saving SQL component: {component}")
        result = db_access.save_sql_component(component)
        #logging.info(f"Result of saving SQL component: {result}")
        #return json.loads(result)
        return result
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Run SQL component
@app.post("/sql_components/{name}/run")
async def run_sql_component(name: str, env_vars: Dict[str, str] = {}):
    try:
        result = db_access.run_sql_component(name, env_vars)
        return json.loads(result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    # Register signal handlers for graceful shutdown
    signal.signal(signal.SIGINT, handle_exit)
    signal.signal(signal.SIGTERM, handle_exit)
    
    # Find an available port
    port = find_available_port()
    
    # Check if direct output flag is provided (used by Rust to get the port)
    if "--direct-output" in sys.argv:
        # Output port directly to stdout for Rust to read
        print(port, flush=True)
    
    # Write the PID file
    #write_pid_file()
    
    # Write server info with port
    #write_server_info(port)
    
    # Start the API server
    uvicorn.run(
        app, 
        host="127.0.0.1", 
        port=port,
        log_config=LOGGING_CONFIG
    )
