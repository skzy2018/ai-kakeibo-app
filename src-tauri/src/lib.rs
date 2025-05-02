use std::path::Path;
use std::process::{Command, Child, Stdio};
use std::sync::{Mutex, Arc};
use std::io::{BufRead, ErrorKind};
use std::thread;
use tauri::Manager;

// State struct to manage the Python process and API server
struct AppState {
    initialized: Mutex<bool>,
    api_server: Mutex<Option<Child>>,
    api_port: Mutex<Option<u16>>,
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
fn add_account(
    name: String,
    account_type: String,
    currency: Option<String>,
) -> Result<String, String> {
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

// SQL component management functions
// Save SQL component
#[tauri::command]
fn save_sql_component(component: serde_json::Value) -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    // 一時ファイルを作成して、コンポーネントをJSONとして保存
    let temp_file = python_env_path.join("temp_component.json");
    let mut file = std::fs::File::create(&temp_file)
        .map_err(|e| format!("Failed to create temp file: {}", e))?;
    
    // コンポーネントを一時ファイルに書き込む
    serde_json::to_writer_pretty(&mut file, &component)
        .map_err(|e| format!("Failed to write component to temp file: {}", e))?;

    // ファイルからコンポーネントを読み込むPythonスクリプト
    let script = format!(
        "import sys, json, os; sys.path.append('{}'); from db_access import save_sql_component; temp_file = '{}'; component = json.load(open(temp_file, 'r', encoding='utf-8')); result = save_sql_component(component); print(json.dumps(result)); os.remove(temp_file)",
        python_env_path.to_str().unwrap(),
        temp_file.to_str().unwrap().replace("\\", "\\\\")
    );

    let output = Command::new(python_executable)
        .arg("-c")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    // 念のため一時ファイルを削除（Pythonで削除されなかった場合）
    if temp_file.exists() {
        let _ = std::fs::remove_file(&temp_file);
    }

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// Get all SQL components
#[tauri::command]
fn get_sql_components() -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import get_sql_components; print(get_sql_components())",
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

// Get a specific SQL component by name
#[tauri::command]
fn get_sql_component(name: String) -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let script = format!(
        "import sys; sys.path.append('{}'); from db_access import get_sql_component; print(get_sql_component({:?}))",
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

// Run a SQL component with environment variables
#[tauri::command]
fn run_sql_component(name: String, env_vars: Option<serde_json::Value>) -> Result<String, String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    // Serialize the environment variables to a JSON string
    let env_vars_json = match env_vars {
        Some(vars) => serde_json::to_string(&vars)
            .map_err(|e| format!("Failed to serialize environment variables: {}", e))?,
        None => "{}".to_string(),
    };

    let script = format!(
        "import sys, json; sys.path.append('{}'); from db_access import run_sql_component; print(run_sql_component({:?}, json.loads('{}')))",
        python_env_path.to_str().unwrap(),
        name,
        env_vars_json.replace("'", "\\'")
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

// Start the API server
fn start_api_server() -> Result<(Child, u16), String> {
    let python_env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("python-env");
    let python_executable = if cfg!(target_os = "windows") {
        python_env_path.join("Scripts/python")
    } else {
        python_env_path.join("bin/python")
    };

    let api_script = python_env_path.join("api.py");
    
    // Modify the API script invocation to output the port directly to stderr/stdout
    // Start the API server as a background process
    let mut child = Command::new(&python_executable)
        .arg(&api_script)
        .arg("--direct-output") // Add a flag to indicate direct port output
        .current_dir(&python_env_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start API server: {}", e))?;
    
    // Get the stdout handle to read the port number directly
    let stdout = child.stdout.take()
        .ok_or_else(|| "Failed to get stdout from child process".to_string())?;
    
    let mut reader = std::io::BufReader::new(stdout);
    let mut line = String::new();
    
    // Read the first line which should contain the port number
    reader.read_line(&mut line)
        .map_err(|e| format!("Failed to read port from API server: {}", e))?;
    
    // Parse the port number from the output
    let port = line.trim().parse::<u16>()
        .map_err(|e| format!("Failed to parse port number from API server output: {}", e))?;

    let stderr = child.stderr.take()
        .ok_or_else(|| "Failed to get stderr from child process".to_string())?;
    
    thread::spawn( move || {
        let mut stderr_reader = std::io::BufReader::new(stderr);
        for line in stderr_reader.lines() {
            match line {
                Ok(log_line) => {
                    println!("API server stderr: {}", log_line);
                }
                Err(e) => {
                    eprintln!("Error reading API server stderr error: {}", e);
                    break;
                }
            }
        }
        println!("[API server log] stderr stream closed.");
    });
    // Return the child process and port
    Ok((child, port))
}

// Stop the API server
fn stop_api_server(child: &mut Child) -> Result<(), String> {
    println!("API server stop_api_server");
    // Try to terminate gracefully first
    if let Err(e) = child.kill() {
        println!("API server stop_api_server Error ");
        if e.kind() != ErrorKind::InvalidInput {
            // Only return error if it's not because the process has already exited
            return Err(format!("Failed to kill API server: {}", e));
        }
    }
    
    // Wait for the process to exit to avoid zombies
    match child.wait() {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == ErrorKind::InvalidInput {
                // Process already exited
                println!("API server stop_api_server Error ");
                Ok(())
            } else {
                Err(format!("Failed to wait for API server to exit: {}", e))
            }
        }
    }
}

// Get the API base URL
fn get_api_base_url(port: u16) -> String {
    format!("http://127.0.0.1:{}", port)
}

// Initialize database using API
#[tauri::command]
fn api_init_database(state: tauri::State<AppState>) -> Result<String, String> {
    let port = state.api_port.lock().unwrap();
    let port = port.expect("API server not started");
    
    let url = format!("{}/init_database", get_api_base_url(port));
    
    // Make HTTP request to initialize database
    let client = reqwest::blocking::Client::new();
    let response = client.post(&url)
        .send()
        .map_err(|e| format!("Failed to send request to API server: {}", e))?;
    
    if response.status().is_success() {
        let result = response.text()
            .map_err(|e| format!("Failed to read API response: {}", e))?;
        Ok(result)
    } else {
        Err(format!("API request failed with status: {}", response.status()))
    }
}

// Get API health status
#[tauri::command]
fn api_health(state: tauri::State<AppState>) -> Result<String, String> {
    let port = state.api_port.lock().unwrap();
    let port = port.expect("API server not started");
    
    let url = format!("{}/health", get_api_base_url(port));
    
    // Make HTTP request to check API health
    let client = reqwest::blocking::Client::new();
    let response = client.get(&url)
        .send()
        .map_err(|e| format!("Failed to send request to API server: {}", e))?;
    
    if response.status().is_success() {
        let result = response.text()
            .map_err(|e| format!("Failed to read API response: {}", e))?;
        Ok(result)
    } else {
        Err(format!("API request failed with status: {}", response.status()))
    }
}

// Get the API base URL for use in frontend
#[tauri::command]
fn get_api_url(state: tauri::State<AppState>) -> String {
    let port = state.api_port.lock().unwrap();
    match *port {
        Some(p) => get_api_base_url(p),
        None => "API server not started".to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize app state with Arc
    let app_state = Arc::new(AppState {
        initialized: Mutex::new(false),
        api_server: Mutex::new(None),
        api_port: Mutex::new(None),
    });
    
    // Create clone for setup closure
    let setup_state = Arc::clone(&app_state);
    
    // Create clone for window event closure
    //let event_state = Arc::clone(&app_state);
    
    // Create app with state
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(app_state.clone())
        .setup(move |_app| {
            // Start API server during setup
            println!("Starting API server...");
            
            match start_api_server() {
                Ok((child, port)) => {
                    println!("API server started on port {}", port);
                    
                    // Store the child process and port in app state
                    *setup_state.api_server.lock().unwrap() = Some(child);
                    *setup_state.api_port.lock().unwrap() = Some(port);
                },
                Err(e) => {
                    eprintln!("Failed to start API server: {}", e);
                }
            }
            
            Ok(())
        })
        .on_window_event(move |window, event| {
            //if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Attempt to shutdown the API server when the application is closing
                println!("Application closing, stopping API server...");
                
                //let mut api_server_guard = event_state.api_server.lock().unwrap();
                //if let Some(ref mut child) = *api_server_guard {
                // get the shared state directly from the window
                let state = window.app_handle().state::<Arc<AppState>>();
                if let Ok(mut api_server_guard) = state.api_server.lock() {
                    if let Some(ref mut child) = *api_server_guard {
                        if let Err(e) = stop_api_server(child) {
                            eprintln!("Error stopping API server: {}", e);
                        } else {
                            println!("API server stopped successfully");
                            // Clear the API server reference after successful shutdown
                            *api_server_guard = None;
                        }
                    }
                }
                
                // Get a handle to the window that can be used across thread boundaries
                //let window_handle = window.clone();
                
                // Allow the window to close without recursively triggering close events
                //api.prevent_close();
                
                // Use a separate task to close the window to avoid recursion
                //std::thread::spawn(move || {
                //    // Small delay to ensure the event loop completes
                //    std::thread::sleep(std::time::Duration::from_millis(1000));
                //    window_handle.close().unwrap();
                //});
                std::thread::sleep(std::time::Duration::from_millis(1000));
                //window.close().unwrap();
            }
        })
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
            load_csv_file,
            // SQL component management
            save_sql_component,
            get_sql_components,
            get_sql_component,
            run_sql_component,
            // API related commands
            api_health,
            api_init_database,
            get_api_url
        ]);
    
    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// App state needs to be cloneable for use in tauri::Builder
impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            initialized: Mutex::new(*self.initialized.lock().unwrap()),
            api_server: Mutex::new(None), // Can't clone child process, so initialize to None
            api_port: Mutex::new(*self.api_port.lock().unwrap()),
        }
    }
}
