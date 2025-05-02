// API Client for communicating with the FastAPI server

// API client for the Kakeibo application
export class ApiClient {
  private initializing_flag = false;
  private baseUrl: string | null = null;
  private isBrowser: boolean;

  // Constructor initializes the API client
  constructor() {
    this.isBrowser = typeof window !== 'undefined';
  }

  // Initialize the API client by getting the API URL from Rust
  async initialize(): Promise<void> {
    console.log('Initializing API client...');
    if (this.initializing_flag) {
      console.log('already Initializing API client!!');
      return;
    }
    this.initializing_flag = true;

    // Skip initialization if not in browser
    if (!this.isBrowser) {
      console.warn('API client not initialized: Not in browser environment');
      return;
    }
    
    try {
      // In Tauri 2, we need a different approach for state-dependent commands
      // Use fetch directly to get the port from a well-known location
      let apiUrl = "http://127.0.0.1:8000"; // Default fallback
      
      // Try a sequence of ports, starting from 8000
      for (let port = 8000; port < 8100; port++) {
        console.log('creating api client with port:', port)
        try {
          const response = await fetch(`http://127.0.0.1:${port}/health`, {
            method: 'GET',
            headers: {
              'Content-Type': 'application/json'
            },
            signal: AbortSignal.timeout(100) // Quick timeout to move to next port
          });
          
          if (response.ok) {
            apiUrl = `http://127.0.0.1:${port}`;
            break;
          }
        } catch (e) {
          // Ignore connection errors and try next port
        }
      }
      
      console.log("exiting initializeing process ... initializing_flag will be flalse")
      this.initializing_flag = false;
      this.baseUrl = apiUrl;
      console.log(`API client initialized with base URL: ${this.baseUrl}`);
    } catch (error) {
      console.log("exiting initializeing process with error ... initializing_flag will be flalse")
      this.initializing_flag = false;
      console.error('Failed to initialize API client:', error);
      throw error;
    }
  }

  // Check if the API client is initialized
  isInitialized(): boolean {
    // In non-browser environments, pretend we're initialized to avoid errors
    console.log('check initializied :', this.baseUrl, this.initializing_flag, this.isBrowser);
    if (!this.isBrowser) {
      return true;
    }
    return ( this.baseUrl !== null || this.initializing_flag );
  }

  // Check the health of the API
  async checkHealth(): Promise<any> {
    if (!this.baseUrl) {
      throw new Error('API client not initialized');
    }
    return this.get<any>('/health');
  }

  // Initialize the database via API
  async initDatabase(): Promise<any> {
    if (!this.baseUrl) {
      throw new Error('API client not initialized');
    }
    return this.post<any>('/init_database');
  }

  // Generic HTTP request function
  private async request<T>(
    method: string,
    endpoint: string,
    data?: any
  ): Promise<T> {
    if (!this.baseUrl) {
      throw new Error('API client not initialized');
    }

    const url = `${this.baseUrl}${endpoint}`;
    
    const headers = {
      'Content-Type': 'application/json',
    };

    const options: RequestInit = {
      method,
      headers,
      // Do not include credentials to avoid CORS issues
    };

    if (data && (method === 'POST' || method === 'PUT' || method === 'PATCH')) {
      options.body = JSON.stringify(data);
    }

    const response = await fetch(url, options);

    if (!response.ok) {
      throw new Error(`API request failed: ${response.status} ${response.statusText}`);
    }

    if (response.headers.get('Content-Type')?.includes('application/json')) {
      return response.json() as Promise<T>;
    }

    return response.text() as unknown as T;
  }

  // Helper methods for common HTTP methods
  private async get<T>(endpoint: string): Promise<T> {
    return this.request<T>('GET', endpoint);
  }

  private async post<T>(endpoint: string, data?: any): Promise<T> {
    return this.request<T>('POST', endpoint, data);
  }

  private async put<T>(endpoint: string, data: any): Promise<T> {
    return this.request<T>('PUT', endpoint, data);
  }

  private async delete<T>(endpoint: string): Promise<T> {
    return this.request<T>('DELETE', endpoint);
  }

  // API methods for accounts
  async getAccounts(): Promise<any[]> {
    return this.get<any[]>('/accounts');
  }

  async addAccount(name: string, accountType: string, currency: string = 'JPY'): Promise<any> {
    return this.post<any>('/accounts', { name, account_type: accountType, currency });
  }

  async deleteAccount(accountId: number): Promise<any> {
    return this.delete<any>(`/accounts/${accountId}`);
  }

  // API methods for categories
  async getCategories(): Promise<any[]> {
    return this.get<any[]>('/categories');
  }

  async addCategory(name: string, categoryType: string): Promise<any> {
    return this.post<any>('/categories', { name, category_type: categoryType });
  }

  async deleteCategory(categoryId: number): Promise<any> {
    return this.delete<any>(`/categories/${categoryId}`);
  }

  // API methods for tags
  async getTags(): Promise<any[]> {
    return this.get<any[]>('/tags');
  }

  async addTag(name: string): Promise<any> {
    return this.post<any>('/tags', { name });
  }

  async deleteTag(tagId: number): Promise<any> {
    return this.delete<any>(`/tags/${tagId}`);
  }

  // API methods for transactions
  async getTransactions(limit: number = 100, offset: number = 0): Promise<any[]> {
    return this.get<any[]>(`/transactions?limit=${limit}&offset=${offset}`);
  }

  async addTransaction(
    accountId: number,
    categoryId: number,
    amount: number,
    description: string,
    transactionDate: string,
    memo?: string,
    tags?: number[]
  ): Promise<any> {
    return this.post<any>('/transactions', {
      account_id: accountId,
      category_id: categoryId,
      amount,
      description,
      transaction_date: transactionDate,
      memo,
      tags
    });
  }

  // API methods for CSV files
  async getCsvFiles(): Promise<string[]> {
    return this.get<any>('/csv_files');
  }

  async loadCsvFile(filename: string): Promise<any> {
    return this.post<any>(`/csv_files/${filename}`);
  }

  // API methods for SQL components
  async getSqlComponents(): Promise<any[]> {
    return this.get<any[]>('/sql_components');
  }

  async getSqlComponent(name: string): Promise<any> {
    return this.get<any>(`/sql_components/${name}`);
  }

  async saveSqlComponent(component: any): Promise<any> {
    return this.post<any>('/sql_components', component);
  }

  async runSqlComponent(name: string, envVars?: any): Promise<any> {
    return this.post<any>(`/sql_components/${name}/run`, envVars || {});
  }

  // Execute custom SQL query
  async executeSql(sql: string): Promise<any> {
    return this.post<any>('/execute_sql', {"sql":sql});
  }
}

// Create a singleton instance of the API client
export const apiClient = new ApiClient();
