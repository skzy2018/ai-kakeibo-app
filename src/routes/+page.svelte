<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  // State variables
  let initialized = false;
  let activeTab = "dashboard";
  let accounts = [];
  let categories = [];
  let transactions = [];
  let sqlQuery = "";
  let sqlResult = null;

  // Initialize the database and load initial data
  onMount(async () => {
    try {
      const result = await invoke("init_database");
      console.log("Database initialized:", result);
      initialized = true;
      
      // Load accounts and categories
      await loadAccounts();
      await loadCategories();
      await loadTransactions();
    } catch (error) {
      console.error("Failed to initialize database:", error);
    }
  });

  async function loadAccounts() {
    try {
      const result = await invoke("get_accounts");
      accounts = JSON.parse(result);
    } catch (error) {
      console.error("Failed to load accounts:", error);
    }
  }

  async function loadCategories() {
    try {
      const result = await invoke("get_categories");
      categories = JSON.parse(result);
    } catch (error) {
      console.error("Failed to load categories:", error);
    }
  }

  async function loadTransactions(limit = 100, offset = 0) {
    try {
      const result = await invoke("get_transactions", { limit, offset });
      transactions = JSON.parse(result);
    } catch (error) {
      console.error("Failed to load transactions:", error);
    }
  }

  async function executeSql() {
    if (!sqlQuery.trim()) return;
    
    try {
      const result = await invoke("execute_sql", { sql: sqlQuery });
      sqlResult = JSON.parse(result);
    } catch (error) {
      console.error("Failed to execute SQL:", error);
      sqlResult = { success: false, error: String(error) };
    }
  }
</script>

<main>
  <header>
    <h1>AI-Kakeibo-App</h1>
  </header>

  <nav>
    <ul>
      <li class:active={activeTab === "dashboard"}>
        <button on:click={() => activeTab = "dashboard"}>ダッシュボード</button>
      </li>
      <li class:active={activeTab === "transactions"}>
        <button on:click={() => activeTab = "transactions"}>取引</button>
      </li>
      <li class:active={activeTab === "data-collector"}>
        <button on:click={() => activeTab = "data-collector"}>データコレクタ</button>
      </li>
      <li class:active={activeTab === "sql-component"}>
        <button on:click={() => activeTab = "sql-component"}>SQLコンポーネント</button>
      </li>
      <li class:active={activeTab === "ai-chat"}>
        <button on:click={() => activeTab = "ai-chat"}>AIチャット</button>
      </li>
    </ul>
  </nav>

  <div class="content">
    {#if !initialized}
      <div class="loading">
        <p>データベースを初期化中...</p>
      </div>
    {:else if activeTab === "dashboard"}
      <section class="dashboard">
        <h2>ダッシュボード</h2>
        <p>複数のSQLコンポーネントを自由に配置できます。</p>
        <div class="dashboard-grid">
          <!-- Dashboard components will be placed here -->
          <div class="dashboard-widget">
            <h3>最近の取引</h3>
            <div class="widget-content">
              {#if transactions.length > 0}
                <ul class="transaction-list">
                  {#each transactions.slice(0, 5) as transaction}
                    <li>
                      <span class="date">{transaction.transaction_date}</span>
                      <span class="description">{transaction.description}</span>
                      <span class="amount">{transaction.amount.toLocaleString()} 円</span>
                    </li>
                  {/each}
                </ul>
              {:else}
                <p>取引データがありません</p>
              {/if}
            </div>
          </div>
        </div>
      </section>
    {:else if activeTab === "transactions"}
      <section class="transactions">
        <h2>取引一覧</h2>
        <div class="transaction-controls">
          <button>新規取引を追加</button>
        </div>
        <div class="transaction-table">
          {#if transactions.length > 0}
            <table>
              <thead>
                <tr>
                  <th>日付</th>
                  <th>説明</th>
                  <th>カテゴリ</th>
                  <th>口座</th>
                  <th>金額</th>
                </tr>
              </thead>
              <tbody>
                {#each transactions as transaction}
                  <tr>
                    <td>{transaction.transaction_date}</td>
                    <td>{transaction.description}</td>
                    <td>{transaction.category_name}</td>
                    <td>{transaction.account_name}</td>
                    <td class:negative={transaction.amount < 0}>
                      {transaction.amount.toLocaleString()} 円
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {:else}
            <p>取引データがありません</p>
          {/if}
        </div>
      </section>
    {:else if activeTab === "data-collector"}
      <section class="data-collector">
        <h2>データコレクタ</h2>
        <p>CSVファイルや外部データを取り込み、家計簿データベースに登録します。</p>
        <div class="collector-form">
          <div class="import-section">
            <h3>CSVインポート</h3>
            <input type="file" accept=".csv" />
            <button>選択したファイルをインポート</button>
          </div>
        </div>
      </section>
    {:else if activeTab === "sql-component"}
      <section class="sql-component">
        <h2>SQLコンポーネント</h2>
        <p>SQLクエリを実行して家計簿データを分析・可視化できます。</p>
        <div class="sql-editor">
          <textarea 
            bind:value={sqlQuery} 
            placeholder="SELECT * FROM transactions LIMIT 10"></textarea>
          <button on:click={executeSql}>実行</button>
        </div>
        <div class="sql-result">
          {#if sqlResult}
            {#if sqlResult.success}
              <div class="success">
                <h3>実行結果</h3>
                {#if sqlResult.data && sqlResult.data.length > 0}
                  <table>
                    <thead>
                      <tr>
                        {#each sqlResult.columns as column}
                          <th>{column}</th>
                        {/each}
                      </tr>
                    </thead>
                    <tbody>
                      {#each sqlResult.data as row}
                        <tr>
                          {#each sqlResult.columns as column}
                            <td>{row[column]}</td>
                          {/each}
                        </tr>
                      {/each}
                    </tbody>
                  </table>
                {:else}
                  <p>結果はありません</p>
                {/if}
              </div>
            {:else}
              <div class="error">
                <h3>エラー</h3>
                <p>{sqlResult.error}</p>
              </div>
            {/if}
          {/if}
        </div>
      </section>
    {:else if activeTab === "ai-chat"}
      <section class="ai-chat">
        <h2>AIチャット</h2>
        <p>家計簿の現状や分析について、AIにチャットで相談できます。</p>
        <div class="chat-interface">
          <div class="chat-messages">
            <div class="message ai">
              <p>こんにちは！あなたの家計簿について、何かお手伝いできることはありますか？</p>
            </div>
          </div>
          <div class="chat-input">
            <textarea placeholder="AIに質問してください..."></textarea>
            <button>送信</button>
          </div>
        </div>
      </section>
    {/if}
  </div>
</main>

<style>
  :root {
    font-family: 'Helvetica Neue', Arial, sans-serif;
    font-size: 16px;
    line-height: 1.5;
    --primary-color: #3e64ff;
    --secondary-color: #5edfff;
    --text-color: #333;
    --bg-color: #f5f7fa;
    --card-bg: #ffffff;
    --border-color: #e0e0e0;
    --success-color: #28a745;
    --error-color: #dc3545;
  }

  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
    background-color: var(--bg-color);
    min-height: 100vh;
  }

  header {
    margin-bottom: 2rem;
  }

  h1 {
    color: var(--primary-color);
    font-size: 2.5rem;
    margin: 0;
  }

  nav ul {
    display: flex;
    list-style: none;
    padding: 0;
    margin: 0 0 2rem 0;
    background-color: var(--card-bg);
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  nav li {
    flex: 1;
  }

  nav button {
    width: 100%;
    padding: 1rem;
    border: none;
    background: transparent;
    cursor: pointer;
    font-weight: 500;
    color: var(--text-color);
    transition: all 0.3s;
  }

  nav li.active button {
    color: var(--primary-color);
    border-bottom: 3px solid var(--primary-color);
  }

  nav button:hover {
    background-color: rgba(62, 100, 255, 0.1);
  }

  .content {
    background-color: var(--card-bg);
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  section h2 {
    color: var(--primary-color);
    margin-top: 0;
  }

  /* Dashboard specific styles */
  .dashboard-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-top: 1.5rem;
  }

  .dashboard-widget {
    background-color: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }

  .dashboard-widget h3 {
    font-size: 1.1rem;
    margin-top: 0;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  /* Transaction specific styles */
  .transaction-controls {
    margin-bottom: 1rem;
  }

  .transaction-table {
    overflow-x: auto;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th, td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid var(--border-color);
  }

  th {
    background-color: rgba(62, 100, 255, 0.1);
    font-weight: 500;
  }

  .negative {
    color: var(--error-color);
  }

  /* SQL Component specific styles */
  .sql-editor {
    display: flex;
    margin-bottom: 1rem;
  }

  .sql-editor textarea {
    flex: 1;
    min-height: 150px;
    padding: 0.5rem;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-family: monospace;
    margin-right: 0.5rem;
  }

  button {
    padding: 0.5rem 1rem;
    background-color: var(--primary-color);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.3s;
  }

  button:hover {
    background-color: #2a4cda;
  }

  .error {
    color: var(--error-color);
    padding: 1rem;
    background-color: rgba(220, 53, 69, 0.1);
    border-radius: 4px;
    margin-top: 1rem;
  }

  /* AI Chat specific styles */
  .chat-interface {
    display: flex;
    flex-direction: column;
    height: 500px;
  }

  .chat-messages {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    background-color: #f9f9f9;
    border-radius: 8px;
    margin-bottom: 1rem;
  }

  .message {
    margin-bottom: 1rem;
    padding: 0.75rem;
    border-radius: 8px;
    max-width: 70%;
  }

  .message.ai {
    background-color: #e9f5ff;
    align-self: flex-start;
  }

  .message.user {
    background-color: #e3f2fd;
    align-self: flex-end;
    margin-left: auto;
  }

  .chat-input {
    display: flex;
  }

  .chat-input textarea {
    flex: 1;
    padding: 0.75rem;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    resize: none;
    min-height: 80px;
    margin-right: 0.5rem;
  }

  /* Responsive adjustments */
  @media (max-width: 768px) {
    nav ul {
      flex-direction: column;
    }

    .dashboard-grid {
      grid-template-columns: 1fr;
    }
  }

  .transaction-list {
    list-style: none;
    padding: 0;
  }

  .transaction-list li {
    padding: 0.5rem 0;
    border-bottom: 1px solid var(--border-color);
    display: flex;
    justify-content: space-between;
  }

  .transaction-list .date {
    flex: 0 0 100px;
  }

  .transaction-list .description {
    flex: 1;
    margin: 0 1rem;
  }

  .transaction-list .amount {
    flex: 0 0 100px;
    text-align: right;
    font-weight: bold;
  }
</style>
