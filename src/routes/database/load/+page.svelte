<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { confirm } from "@tauri-apps/plugin-dialog";
  
  // State variables
  let loading = false;
  let error: any = null;
  let success = false;
  let successMessage = "";
  let csvFiles: string[] = [];
  let processingFile = "";
  
  onMount(async () => {
    await loadCsvFiles();
  });
  
  async function loadCsvFiles() {
    loading = true;
    error = null;
    
    try {
      // Get list of CSV files
      const result = await invoke<string>("get_csv_files");
      console.log("get_csv_file execute")
      csvFiles = JSON.parse(result);
      loading = false;
    } catch (err) {
      console.error("Failed to get CSV files:", err);
      error = err;
      loading = false;
    }
  }
  
  async function loadCsvFile(filename: string) {
    if (!await confirm(`CSVファイル ${filename} をデータベースにロードしますか？`)) {
      return;
    }
    
    loading = true;
    error = null;
    success = false;
    processingFile = filename;
    
    try {
      // Load CSV file
      const result = await invoke<string>("load_csv_file", { filename });
      const resultData = JSON.parse(result);
      
      if (resultData.success) {
        success = true;
        successMessage = `ファイル ${filename} からデータを正常にロードしました。
          ${resultData.transactions_inserted}件のトランザクションと
          ${resultData.tags_inserted}件のタグが登録されました。`;
          
        // Reload CSV file list
        await loadCsvFiles();
      } else {
        error = resultData.error || "データのロードに失敗しました";
      }
    } catch (err) {
      console.error("Failed to load CSV file:", err);
      error = `ファイルの処理に失敗しました: ${err}`;
    } finally {
      loading = false;
      processingFile = "";
    }
  }
</script>

<div class="page-container">
  <h1>データロード</h1>
  <p>CSVファイルから家計簿データをインポートします。ファイルは data/csv ディレクトリに配置してください。</p>
  
  <div class="content-card">
    <div class="form-section">
      <h2>ロード可能なCSVファイル</h2>
      
      {#if loading}
        <div class="loading">
          <p>データを読み込み中...</p>
        </div>
      {:else if error}
        <div class="error-message">
          <p>エラーが発生しました: {error}</p>
          <button on:click={() => loadCsvFiles()}>再試行</button>
        </div>
      {:else if csvFiles.length === 0}
        <div class="empty-state">
          <p>ロード可能なCSVファイルが見つかりません。</p>
          <p>CSVファイルを data/csv ディレクトリに配置してください。</p>
        </div>
      {:else}
        <div class="csv-file-list">
          <table>
            <thead>
              <tr>
                <th>ファイル名</th>
                <th class="actions-column">操作</th>
              </tr>
            </thead>
            <tbody>
              {#each csvFiles as filename}
                <tr>
                  <td>{filename}</td>
                  <td class="actions">
                    <button 
                      class="primary"
                      on:click={() => loadCsvFile(filename)}
                      disabled={loading && processingFile === filename}
                    >
                      {loading && processingFile === filename ? 'ロード中...' : 'ロード実行'}
                    </button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
    
    {#if error}
      <div class="error-message">
        {error}
      </div>
    {/if}
    
    {#if success}
      <div class="success-message">
        {successMessage}
      </div>
    {/if}
  </div>
  
  <div class="tips-section">
    <h2>CSVファイルロードのヒント</h2>
    
    <div class="tips-content">
      <h3>CSVファイルのフォーマット</h3>
      <ul>
        <li>CSVファイルは最初の行がヘッダー行である必要があります。</li>
        <li>ヘッダー行の形式: transaction_date,accounts.name,categories.type,categories.name,transactions.amount,transactions.item_name,tags.name|tags.name|...,transactions.description,transactions.memo</li>
        <li>日付は YYYY-MM-DD 形式を推奨します（例: 2025-04-25）。</li>
        <li>金額はマイナス符号（-）を使用して支出を表します（例: -5000）。</li>
        <li>カテゴリと口座は、存在する場合は名前で指定します。存在しない場合は自動的に作成されます。</li>
        <li>タグは配列形式で指定します（例:タグ1|タグ2|タグ3）。</li>
      </ul>
      
      <h3>サンプルCSVフォーマット:</h3>
      <pre>transaction_date,accounts.name,categories.type,categories.name,transactions.amount,transactions.item_name,tags.name|tags.name|...,transactions.description,transactions.memo
2025-04-25,現金,支出,食費,-3500,スーパーでの買い物,食料品|日用品,スーパーで食料品を購入,
2025-04-24,銀行口座,収入,給料,280000,4月分給与,固定収入,会社からの給与,
2025-04-23,銀行口座,支出,住居費,-15000,電気代,固定費|公共料金,4月分の電気代,</pre>
    </div>
  </div>
</div>

<style>
  .page-container {
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .content-card {
    background-color: white;
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    padding: 2rem;
    margin-bottom: 2rem;
  }
  
  .form-section {
    margin-bottom: 2rem;
  }
  
  .form-section h2 {
    margin-top: 0;
    margin-bottom: 1.5rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--light-border);
  }
  
  .loading, .empty-state {
    text-align: center;
    padding: 2rem;
    background-color: var(--light-bg);
    border-radius: var(--radius-md);
    margin-top: 1rem;
  }
  
  .error-message {
    background-color: rgba(239, 68, 68, 0.1);
    border-left: 4px solid var(--danger);
    padding: 0.75rem 1rem;
    color: var(--danger);
    border-radius: var(--radius-md);
    margin-bottom: 1.5rem;
  }
  
  .success-message {
    background-color: rgba(16, 185, 129, 0.1);
    border-left: 4px solid var(--success);
    padding: 0.75rem 1rem;
    color: var(--success);
    border-radius: var(--radius-md);
    margin-bottom: 1.5rem;
    white-space: pre-line;
  }
  
  .csv-file-list {
    margin-top: 1.5rem;
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
  }
  
  th, td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid var(--light-border);
  }
  
  th {
    font-weight: 600;
    background-color: var(--light-bg);
  }
  
  .actions-column {
    width: 150px;
  }
  
  .actions {
    display: flex;
    gap: 0.5rem;
  }
  
  button {
    cursor: pointer;
  }
  
  button.primary {
    background-color: var(--primary);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: var(--radius-md);
    font-weight: 500;
  }
  
  button.primary:hover {
    background-color: var(--primary-dark);
  }
  
  button.primary:disabled {
    background-color: var(--light-bg-darker);
    cursor: not-allowed;
  }
  
  .tips-section {
    background-color: white;
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    padding: 2rem;
  }
  
  .tips-section h2 {
    margin-top: 0;
    margin-bottom: 1.5rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--light-border);
  }
  
  .tips-content h3 {
    font-size: 1.1rem;
    margin: 1.5rem 0 0.75rem;
  }
  
  .tips-content ul {
    margin-bottom: 1.5rem;
  }
  
  .tips-content li {
    margin-bottom: 0.5rem;
  }
  
  pre {
    background-color: var(--light-bg-darker);
    padding: 1rem;
    border-radius: var(--radius-md);
    overflow-x: auto;
    font-family: var(--font-mono);
    font-size: 0.9rem;
    line-height: 1.5;
    white-space: pre-wrap;
    word-break: break-word;
  }
  
  /* Responsive adjustments */
  @media (max-width: 768px) {
    .content-card, .tips-section {
      padding: 1.5rem;
    }
  }
</style>
