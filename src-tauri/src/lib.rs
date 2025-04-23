use std::path::Path;
use std::process::Command;
use tauri::api::process::{Command as TauriCommand};
use tauri::{Runtime, State};
use std::sync::Mutex;

// State struct to manage the Python process
struct PythonState {
    initialized: Mutex<bool>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Initialize the database
#[tauri::command]
fn init_database() -> Result<String, String> {
    let python_env_path = Path::new("src-tauri/python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let init_script = python_env_path.join("init_db.py");

    let output = Command::new(python_executable)
        .arg(init_script)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// Execute SQL command
#[tauri::command]
fn execute_sql(sql: &str) -> Result<String, String> {
    let python_env_path = Path::new("src-tauri/python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "from db_access import execute_sql; print(execute_sql({:?}))",
        sql
    );

    let output = Command::new(python_executable)
        .arg("-c")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// Get accounts
#[tauri::command]
fn get_accounts() -> Result<String, String> {
    let python_env_path = Path::new("src-tauri/python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = "from db_access import get_accounts; print(get_accounts())";

    let output = Command::new(python_executable)
        .arg("-c")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// Get categories
#[tauri::command]
fn get_categories() -> Result<String, String> {
    let python_env_path = Path::new("src-tauri/python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = "from db_access import get_categories; print(get_categories())";

    let output = Command::new(python_executable)
        .arg("-c")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// Get transactions
#[tauri::command]
fn get_transactions(limit: u32, offset: u32) -> Result<String, String> {
    let python_env_path = Path::new("src-tauri/python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "from db_access import get_transactions; print(get_transactions({}, {}))",
        limit, offset
    );

    let output = Command::new(python_executable)
        .arg("-c")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// Add transaction
#[tauri::command]
fn add_transaction(
    account_id: u32,
    category_id: u32,
    amount: f64,
    description: String,
    transaction_date: String,
    memo: Option<String>,
    tags: Option<Vec<u32>>,
) -> Result<String, String> {
    let python_env_path = Path::new("src-tauri/python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let memo_str = memo.unwrap_or_else(|| "".to_string());
    let tags_str = match tags {
        Some(t) => format!("{:?}", t),
        None => "None".to_string(),
    };

    let script = format!(
        "from db_access import add_transaction; print(add_transaction({}, {}, {}, {:?}, {:?}, {:?}, {}))",
        account_id, category_id, amount, description, transaction_date, memo_str, tags_str
    );

    let output = Command::new(python_executable)
        .arg("-c")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// Import from CSV
#[tauri::command]
fn import_from_csv(csv_file: String, mapping: String) -> Result<String, String> {
    let python_env_path = Path::new("src-tauri/python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "from db_access import import_from_csv; print(import_from_csv({:?}, {}))",
        csv_file, mapping
    );

    let output = Command::new(python_executable)
        .arg("-c")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let python_state = PythonState {
        initialized: Mutex::new(false),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(python_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            init_database,
            execute_sql,
            get_accounts,
            get_categories,
            get_transactions,
            add_transaction,
            import_from_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
