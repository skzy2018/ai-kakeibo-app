<script lang="ts">
  import { onMount } from "svelte";
  
  // Form data
  let componentName = "";
  let sqlQuery = "";
  let description = "";
  let visualizationType = "bar"; // Default visualization type
  let isCreating = false;
  let createSuccess = false;
  let errorMessage = "";
  
  // Sample metadata about database structure to help users write SQL
  let tableInfo = [
    { 
      name: "transactions", 
      description: "取引記録テーブル",
      columns: [
        { name: "id", type: "INTEGER", description: "主キー" },
        { name: "account_id", type: "INTEGER", description: "口座ID" },
        { name: "category_id", type: "INTEGER", description: "カテゴリID" },
        { name: "amount", type: "DECIMAL", description: "金額" },
        { name: "transaction_date", type: "DATE", description: "取引日" },
        { name: "description", type: "TEXT", description: "説明" }
      ]
    },
    { 
      name: "accounts", 
      description: "口座テーブル",
      columns: [
        { name: "id", type: "INTEGER", description: "主キー" },
        { name: "name", type: "TEXT", description: "口座名" },
        { name: "type", type: "TEXT", description: "口座種類" },
        { name: "balance", type: "DECIMAL", description: "残高" }
      ]
    },
    { 
      name: "categories", 
      description: "カテゴリテーブル",
      columns: [
        { name: "id", type: "INTEGER", description: "主キー" },
        { name: "name", type: "TEXT", description: "カテゴリ名" },
        { name: "type", type: "TEXT", description: "カテゴリ種類 (収入/支出)" },
        { name: "parent_id", type: "INTEGER", description: "親カテゴリID" }
      ]
    },
    { 
      name: "tags", 
      description: "タグテーブル",
      columns: [
        { name: "id", type: "INTEGER", description: "主キー" },
        { name: "name", type: "TEXT", description: "タグ名" }
      ]
    },
    { 
      name: "transaction_tags", 
      description: "取引とタグの関連テーブル",
      columns: [
        { name: "transaction_id", type: "INTEGER", description: "取引ID" },
        { name: "tag_id", type: "INTEGER", description: "タグID" }
      ]
    }
  ];
  
  // Sample SQL queries for different visualization types
  let sampleQueries: Record<string, string> = {
    bar: `-- 月別支出合計 (棒グラフ)
SELECT 
  strftime('%Y-%m', transaction_date) as month,
  SUM(CASE WHEN amount < 0 THEN ABS(amount) ELSE 0 END) as total_expense
FROM transactions
GROUP BY month
ORDER BY month
LIMIT 12;`,
    
    line: `-- 月別収支推移 (折れ線グラフ)
SELECT 
  strftime('%Y-%m', transaction_date) as month,
  SUM(CASE WHEN amount > 0 THEN amount ELSE 0 END) as income,
  SUM(CASE WHEN amount < 0 THEN ABS(amount) ELSE 0 END) as expense
FROM transactions
GROUP BY month
ORDER BY month
LIMIT 12;`,
    
    pie: `-- カテゴリ別支出割合 (円グラフ)
SELECT 
  c.name as category_name,
  SUM(ABS(t.amount)) as total_amount
FROM transactions t
JOIN categories c ON t.category_id = c.id
WHERE t.amount < 0
GROUP BY c.name
ORDER BY total_amount DESC;`,
    
    table: `-- 最近の取引 (テーブル)
SELECT 
  t.transaction_date,
  t.description,
  c.name as category,
  a.name as account,
  t.amount
FROM transactions t
JOIN categories c ON t.category_id = c.id
JOIN accounts a ON t.account_id = a.id
ORDER BY t.transaction_date DESC
LIMIT 10;`
  };
  
  // Update the sample query when visualization type changes
  $: if (visualizationType) {
    // Only update if the user hasn't written anything yet or if they're using a sample
    if (!sqlQuery || Object.values(sampleQueries).includes(sqlQuery)) {
      sqlQuery = sampleQueries[visualizationType];
    }
  }
  
  async function createSQLComponent() {
    if (!componentName.trim() || !sqlQuery.trim()) {
      errorMessage = "コンポーネント名とSQLクエリを入力してください";
      return;
    }
    
    isCreating = true;
    errorMessage = "";
    
    try {
      // In a real app, this would call a backend function
      // For now, we'll just simulate success
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      createSuccess = true;
      
      // Reset form after short delay
      setTimeout(() => {
        componentName = "";
        sqlQuery = sampleQueries[visualizationType];
        description = "";
        createSuccess = false;
      }, 3000);
      
    } catch (error) {
      console.error("Failed to create SQL component:", error);
      errorMessage = `コンポーネントの作成に失敗しました: ${error}`;
    } finally {
      isCreating = false;
    }
  }
  
  function testSQLQuery() {
    // In a real app, this would test the SQL query
    console.log("Testing SQL query:", sqlQuery);
    // For now, we'll just show a success message
    alert("SQLのテストは実装予定です。現在はモックモードで動作しています。");
  }
</script>

<div class="page-container">
  <h1>SQLコンポーネントの作成</h1>
  <p>家計簿データベースに対するSQLクエリと可視化方法を設定して、ダッシュボードに追加できるコンポーネントを作成します。</p>
  
  <div class="form-container">
    <div class="form-section">
      <h2>基本情報</h2>
      
      <div class="form-group">
        <label for="componentName">コンポーネント名</label>
        <input 
          type="text" 
          id="componentName" 
          bind:value={componentName} 
          placeholder="例: 月別支出推移"
          required
        />
      </div>
      
      <div class="form-group">
        <label for="description">説明 (オプション)</label>
        <textarea 
          id="description" 
          bind:value={description} 
          placeholder="このコンポーネントの目的や内容を説明してください"
          rows="3"
        ></textarea>
      </div>
      
      <div class="form-group">
        <label for="visualizationType">可視化タイプ</label>
        <select id="visualizationType" bind:value={visualizationType}>
          <option value="bar">棒グラフ</option>
          <option value="line">折れ線グラフ</option>
          <option value="pie">円グラフ</option>
          <option value="table">テーブル表示</option>
        </select>
      </div>
    </div>
    
    <div class="form-section">
      <h2>SQLクエリ</h2>
      <div class="form-group">
        <div class="sql-editor-header">
          <label for="sqlQuery">SQLクエリ</label>
          <button class="secondary small" on:click={testSQLQuery}>テスト実行</button>
        </div>
        <textarea 
          id="sqlQuery" 
          bind:value={sqlQuery} 
          class="sql-editor"
          placeholder="SELECT * FROM transactions LIMIT 10"
          rows="10"
          required
        ></textarea>
      </div>
    </div>
    
    <div class="actions">
      {#if errorMessage}
        <div class="error-message">{errorMessage}</div>
      {/if}
      
      {#if createSuccess}
        <div class="success-message">
          コンポーネントが正常に作成されました！
        </div>
      {/if}
      
      <button 
        on:click={createSQLComponent} 
        disabled={isCreating}
        class="primary"
      >
        {#if isCreating}
          作成中...
        {:else}
          SQLコンポーネントを作成
        {/if}
      </button>
    </div>
  </div>
  
  <div class="help-section">
    <h2>データベース構造</h2>
    <p>以下の情報を参考にSQLクエリを作成してください。</p>
    
    <div class="table-info">
      {#each tableInfo as table}
        <div class="db-table">
          <h3>{table.name}</h3>
          <p>{table.description}</p>
          <table>
            <thead>
              <tr>
                <th>カラム名</th>
                <th>型</th>
                <th>説明</th>
              </tr>
            </thead>
            <tbody>
              {#each table.columns as column}
                <tr>
                  <td><code>{column.name}</code></td>
                  <td>{column.type}</td>
                  <td>{column.description}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .page-container {
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .form-container {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    margin: 1.5rem 0;
    background-color: white;
    border-radius: var(--radius-lg);
    padding: 2rem;
    box-shadow: var(--shadow-md);
  }
  
  .form-section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .form-section h2 {
    font-size: 1.25rem;
    margin-bottom: 0.5rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--light-border);
  }
  
  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .sql-editor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .sql-editor {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    line-height: 1.5;
    tab-size: 2;
  }
  
  button.small {
    padding: 0.25rem 0.5rem;
    font-size: 0.875rem;
  }
  
  .actions {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .error-message {
    background-color: rgba(239, 68, 68, 0.1);
    border-left: 4px solid var(--danger);
    padding: 0.75rem 1rem;
    color: var(--danger);
    border-radius: var(--radius-md);
  }
  
  .success-message {
    background-color: rgba(16, 185, 129, 0.1);
    border-left: 4px solid var(--success);
    padding: 0.75rem 1rem;
    color: var(--success);
    border-radius: var(--radius-md);
  }
  
  .help-section {
    margin-top: 3rem;
    background-color: white;
    border-radius: var(--radius-lg);
    padding: 2rem;
    box-shadow: var(--shadow-md);
  }
  
  .table-info {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 1.5rem;
    margin-top: 1rem;
  }
  
  .db-table {
    border: 1px solid var(--light-border);
    border-radius: var(--radius-md);
    padding: 1rem;
  }
  
  .db-table h3 {
    font-family: var(--font-mono);
    font-size: 1.1rem;
    margin-bottom: 0.25rem;
  }
  
  .db-table p {
    font-size: 0.9rem;
    color: var(--light-text-muted);
    margin-bottom: 0.75rem;
  }
  
  .db-table table {
    width: 100%;
    font-size: 0.9rem;
  }
  
  .db-table th {
    font-size: 0.8rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  
  .db-table code {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    background-color: var(--light-bg-darker);
    padding: 0.1rem 0.3rem;
    border-radius: var(--radius-sm);
  }
  
  /* Responsive adjustments */
  @media (max-width: 768px) {
    .table-info {
      grid-template-columns: 1fr;
    }
  }
</style>
