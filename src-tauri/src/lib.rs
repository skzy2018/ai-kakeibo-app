use std::path::Path;
use std::process::Command;
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
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
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
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import execute_sql; print(execute_sql({:?}))",
        python_env_path.to_str().unwrap(),
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
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import get_accounts; print(get_accounts())",
        python_env_path.to_str().unwrap()
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

// Get categories
#[tauri::command]
fn get_categories() -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import get_categories; print(get_categories())",
        python_env_path.to_str().unwrap()
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

// Get tags
#[tauri::command]
fn get_tags() -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import get_tags; print(get_tags())",
        python_env_path.to_str().unwrap()
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

// Get transactions
#[tauri::command]
fn get_transactions(limit: u32, offset: u32) -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import get_transactions; print(get_transactions({}, {}))",
        python_env_path.to_str().unwrap(),
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
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
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
        "import sys; sys.path.append('{}'); from db_access import add_transaction; print(add_transaction({}, {}, {}, {:?}, {:?}, {:?}, {}))",
        python_env_path.to_str().unwrap(),
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

// Master table management functions
// Add account
#[tauri::command]
fn add_account(name: String, account_type: String, currency: Option<String>) -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let currency_str = currency.unwrap_or_else(|| "JPY".to_string());

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import add_account; print(add_account({:?}, {:?}, {:?}))",
        python_env_path.to_str().unwrap(),
        name, account_type, currency_str
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

// Add category
#[tauri::command]
fn add_category(name: String, category_type: String) -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import add_category; print(add_category({:?}, {:?}))",
        python_env_path.to_str().unwrap(),
        name, category_type
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

// Add tag
#[tauri::command]
fn add_tag(name: String) -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import add_tag; print(add_tag({:?}))",
        python_env_path.to_str().unwrap(),
        name
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

// Delete account
#[tauri::command]
fn delete_account(account_id: u32) -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import delete_account; print(delete_account({}))",
        python_env_path.to_str().unwrap(),
        account_id
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

// Delete category
#[tauri::command]
fn delete_category(category_id: u32) -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import delete_category; print(delete_category({}))",
        python_env_path.to_str().unwrap(),
        category_id
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

// Delete tag
#[tauri::command]
fn delete_tag(tag_id: u32) -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import delete_tag; print(delete_tag({}))",
        python_env_path.to_str().unwrap(),
        tag_id
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

// CSV file management functions
// Get CSV files
#[tauri::command]
fn get_csv_files() -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import get_csv_files; print(get_csv_files())",
        python_env_path.to_str().unwrap()
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

// Load CSV file
#[tauri::command]
fn load_csv_file(filename: String) -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import load_csv_file; print(load_csv_file({:?}))",
        python_env_path.to_str().unwrap(),
        filename
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
            get_tags,
            get_transactions,
            add_transaction,
            // Master table management
            add_account,
            add_category,
            add_tag,
            delete_account,
            delete_category,
            delete_tag,
            // CSV file management
            get_csv_files,
            load_csv_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
