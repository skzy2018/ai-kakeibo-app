<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  
  // State variables
  let loading = false;
  let uploadProgress = 0;
  let error = null;
  let success = false;
  let successMessage = "";
  
  // Form data
  let selectedFile = null;
  let fileName = "";
  let fileSize = 0;
  let fileType = "";
  let dataSource = "csv";
  let targetTable = "transactions";
  let mappingMode = "auto";
  let columnMappings = {};
  
  // Available tables
  let tables = [
    { id: "transactions", name: "取引テーブル" },
    { id: "accounts", name: "口座テーブル" },
    { id: "categories", name: "カテゴリテーブル" },
    { id: "tags", name: "タグテーブル" }
  ];
  
  // Sample data from uploaded file (first few rows)
  let sampleData = null;
  
  // Column mapping options
  let targetColumns = {
    transactions: [
      { id: "transaction_date", name: "取引日" },
      { id: "description", name: "説明" },
      { id: "amount", name: "金額" },
      { id: "category_id", name: "カテゴリ" },
      { id: "account_id", name: "口座" }
    ],
    accounts: [
      { id: "name", name: "口座名" },
      { id: "type", name: "種類" },
      { id: "balance", name: "残高" }
    ],
    categories: [
      { id: "name", name: "カテゴリ名" },
      { id: "type", name: "種類" },
      { id: "parent_id", name: "親カテゴリ" }
    ],
    tags: [
      { id: "name", name: "タグ名" }
    ]
  };
  
  function handleFileChange(event) {
    const fileInput = event.target;
    if (fileInput.files && fileInput.files.length > 0) {
      selectedFile = fileInput.files[0];
      fileName = selectedFile.name;
      fileSize = selectedFile.size;
      fileType = selectedFile.type;
      
      // Reset previous state
      error = null;
      success = false;
      sampleData = null;
      
      // For CSV files, read sample data
      if (fileType === "text/csv" || fileName.endsWith(".csv")) {
        const reader = new FileReader();
        reader.onload = (e) => {
          try {
            const csvContent = e.target.result;
            parseCsvSample(csvContent.toString());
          } catch (err) {
            error = `CSVファイルの解析に失敗しました: ${err}`;
          }
        };
        reader.readAsText(selectedFile);
      }
    } else {
      selectedFile = null;
      fileName = "";
      fileSize = 0;
      fileType = "";
      sampleData = null;
    }
  }
  
  function parseCsvSample(csvContent) {
    // Simple CSV parsing for preview (in a real app, use a proper CSV parser)
    const lines = csvContent.split('\n').filter(line => line.trim());
    if (lines.length < 2) {
      error = "CSVファイルが空か、ヘッダーのみです";
      return;
    }
    
    const headers = lines[0].split(',').map(h => h.trim());
    const rows = [];
    
    // Get up to 5 rows for preview
    for (let i = 1; i < Math.min(lines.length, 6); i++) {
      const row = {};
      const values = lines[i].split(',').map(v => v.trim());
      
      headers.forEach((header, index) => {
        row[header] = index < values.length ? values[index] : '';
      });
      
      rows.push(row);
    }
    
    sampleData = { headers, rows };
    
    // Auto-map columns if mode is auto
    if (mappingMode === "auto") {
      autoMapColumns();
    }
  }
  
  function autoMapColumns() {
    if (!sampleData || !sampleData.headers) return;
    
    const targetCols = targetColumns[targetTable];
    const mappings = {};
    
    // Try to auto-map based on name similarity
    sampleData.headers.forEach(sourceCol => {
      const sourceLower = sourceCol.toLowerCase();
      
      // Try to find matching target column
      let matched = false;
      
      targetCols.forEach(targetCol => {
        const targetLower = targetCol.name.toLowerCase();
        const targetIdLower = targetCol.id.toLowerCase();
        
        // Check if source column name contains target column name (or vice versa)
        if (
          !matched && 
          (sourceLower.includes(targetLower) || 
           targetLower.includes(sourceLower) ||
           sourceLower.includes(targetIdLower) ||
           targetIdLower.includes(sourceLower))
        ) {
          mappings[sourceCol] = targetCol.id;
          matched = true;
        }
      });
      
      // If no match found, set to null (skip this column)
      if (!matched) {
        mappings[sourceCol] = null;
      }
    });
    
    columnMappings = mappings;
  }
  
  function updateColumnMapping(sourceCol, targetColId) {
    columnMappings = {
      ...columnMappings,
      [sourceCol]: targetColId
    };
  }
  
  function getReadableFileSize(bytes) {
    if (bytes === 0) return "0 Bytes";
    
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }
  
  async function uploadAndProcess() {
    if (!selectedFile) {
      error = "ファイルを選択してください";
      return;
    }
    
    if (mappingMode === "manual" && Object.keys(columnMappings).length === 0) {
      error = "列のマッピングを設定してください";
      return;
    }
    
    loading = true;
    uploadProgress = 0;
    error = null;
    success = false;
    
    try {
      // Simulate upload process
      for (let i = 1; i <= 10; i++) {
        await new Promise(resolve => setTimeout(resolve, 300));
        uploadProgress = i * 10;
      }
      
      // In a real app, we would call a backend function to process the file
      /* 
      const result = await invoke("load_data_to_table", {
        file: selectedFile,
        targetTable: targetTable,
        mappings: columnMappings
      });
      */
      
      // Simulate success
      success = true;
      successMessage = `${fileName} からデータが正常にロードされました。100件のレコードがインポートされました。`;
      
      // Reset form
      selectedFile = null;
      fileName = "";
      fileSize = 0;
      fileType = "";
      sampleData = null;
      
    } catch (err) {
      console.error("Failed to upload file:", err);
      error = `ファイルの処理に失敗しました: ${err}`;
    } finally {
      loading = false;
    }
  }
</script>

<div class="page-container">
  <h1>データロード</h1>
  <p>CSVや他の形式のデータを家計簿データベースにインポートします。</p>
  
  <div class="content-card">
    <div class="form-section">
      <h2>データソース</h2>
      
      <div class="form-group">
        <label for="dataSource">インポート形式</label>
        <select id="dataSource" bind:value={dataSource}>
          <option value="csv">CSVファイル</option>
          <option value="excel">Excelファイル</option>
          <option value="json">JSONファイル</option>
        </select>
      </div>
      
      <div class="form-group">
        <label for="fileUpload">ファイルを選択</label>
        <div class="file-upload-container">
          <input 
            type="file" 
            id="fileUpload" 
            on:change={handleFileChange}
            accept={dataSource === 'csv' ? '.csv' : 
                   dataSource === 'excel' ? '.xlsx,.xls' : 
                   dataSource === 'json' ? '.json' : ''}
            class="hidden-file-input"
          />
          <div class="file-upload-ui">
            <button class="upload-button" on:click={() => document.getElementById('fileUpload').click()}>
              ファイルを選択
            </button>
            <div class="file-details">
              {#if fileName}
                <div class="file-name">{fileName}</div>
                <div class="file-size">{getReadableFileSize(fileSize)}</div>
              {:else}
                <div class="file-placeholder">ファイルが選択されていません</div>
              {/if}
            </div>
          </div>
        </div>
      </div>
      
      <div class="form-group">
        <label for="targetTable">インポート先テーブル</label>
        <select id="targetTable" bind:value={targetTable}>
          {#each tables as table}
            <option value={table.id}>{table.name}</option>
          {/each}
        </select>
      </div>
    </div>
    
    {#if sampleData}
      <div class="form-section">
        <div class="section-header">
          <h2>データプレビュー</h2>
          <div class="mapping-mode-selector">
            <label>
              <input type="radio" bind:group={mappingMode} value="auto">
              <span>自動マッピング</span>
            </label>
            <label>
              <input type="radio" bind:group={mappingMode} value="manual">
              <span>手動マッピング</span>
            </label>
          </div>
        </div>
        
        <div class="data-preview">
          <table>
            <thead>
              <tr>
                {#each sampleData.headers as header}
                  <th>
                    <div class="column-mapping">
                      <div class="source-column">{header}</div>
                      {#if mappingMode === "manual"}
                        <select 
                          bind:value={columnMappings[header]} 
                          on:change={(e) => updateColumnMapping(header, e.target.value)}
                        >
                          <option value={null}>-- マッピングなし --</option>
                          {#each targetColumns[targetTable] as column}
                            <option value={column.id}>{column.name}</option>
                          {/each}
                        </select>
                      {:else}
                        <div class="auto-mapping">
                          {#if columnMappings[header]}
                            → {targetColumns[targetTable].find(c => c.id === columnMappings[header])?.name || columnMappings[header]}
                          {:else}
                            <span class="mapping-skip">スキップ</span>
                          {/if}
                        </div>
                      {/if}
                    </div>
                  </th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each sampleData.rows as row}
                <tr>
                  {#each sampleData.headers as header}
                    <td>{row[header]}</td>
                  {/each}
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}
    
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
    
    {#if loading}
      <div class="progress-container">
        <div class="progress-bar">
          <div class="progress-fill" style="width: {uploadProgress}%"></div>
        </div>
        <div class="progress-text">{uploadProgress}% 完了</div>
      </div>
    {/if}
    
    <div class="form-actions">
      <button 
        class="primary" 
        on:click={uploadAndProcess} 
        disabled={!selectedFile || loading}
      >
        {loading ? 'インポート中...' : 'インポート実行'}
      </button>
    </div>
  </div>
  
  <div class="tips-section">
    <h2>データロードのヒント</h2>
    
    {#if dataSource === "csv"}
      <div class="tips-content">
        <h3>CSVファイルのフォーマット</h3>
        <ul>
          <li>CSVファイルは最初の行がヘッダー行である必要があります。</li>
          <li>日付は YYYY-MM-DD 形式を推奨します（例: 2025-04-25）。</li>
          <li>金額はマイナス符号（-）を使用して支出を表します（例: -5000）。</li>
          <li>カテゴリと口座は、存在する場合は名前で指定できます。存在しない場合は自動的に作成されます。</li>
        </ul>
        
        <h3>サンプルCSVフォーマット:</h3>
        <pre>日付,説明,金額,カテゴリ,口座
2025-04-25,スーパーでの買い物,-3500,食費,現金
2025-04-24,給料,280000,収入,銀行口座
2025-04-23,電気代,-15000,住居費,銀行口座</pre>
      </div>
    {:else if dataSource === "excel"}
      <div class="tips-content">
        <h3>Excelファイルのフォーマット</h3>
        <ul>
          <li>Excelファイルの最初のシートが処理されます。</li>
          <li>最初の行はヘッダー行として扱われます。</li>
          <li>日付はExcelの日付形式で指定できます。</li>
          <li>金額列には、支出を負の数値（マイナス）で入力してください。</li>
        </ul>
      </div>
    {:else if dataSource === "json"}
      <div class="tips-content">
        <h3>JSONファイルのフォーマット</h3>
        <ul>
          <li>JSONファイルは配列形式である必要があります。</li>
          <li>各オブジェクトは取引を表します。</li>
          <li>必要なフィールド: date, description, amount</li>
          <li>オプションフィールド: category, account</li>
        </ul>
        
        <div class="code-example">
          <h3>サンプルJSON形式:</h3>
          <p>以下のような形式でJSONファイルを準備してください：</p>
          <p>- date: 日付 (YYYY-MM-DD形式)</p>
          <p>- description: 取引の説明</p>
          <p>- amount: 金額 (支出はマイナス値)</p>
          <p>- category: カテゴリ名</p>
          <p>- account: 口座名</p>
        </div>
      </div>
    {/if}
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
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--light-border);
  }
  
  .section-header h2 {
    margin: 0;
    padding: 0;
    border: none;
  }
  
  .form-group {
    margin-bottom: 1.5rem;
  }
  
  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
  }
  
  .hidden-file-input {
    display: none;
  }
  
  .file-upload-container {
    width: 100%;
  }
  
  .file-upload-ui {
    display: flex;
    align-items: center;
    gap: 1rem;
    width: 100%;
  }
  
  .upload-button {
    white-space: nowrap;
  }
  
  .file-details {
    border: 1px dashed var(--light-border);
    padding: 0.75rem;
    border-radius: var(--radius-md);
    flex: 1;
    min-height: 60px;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }
  
  .file-placeholder {
    color: var(--light-text-muted);
    font-style: italic;
  }
  
  .file-name {
    font-weight: 500;
    word-break: break-all;
  }
  
  .file-size {
    font-size: 0.875rem;
    color: var(--light-text-muted);
    margin-top: 0.25rem;
  }
  
  .mapping-mode-selector {
    display: flex;
    gap: 1rem;
  }
  
  .mapping-mode-selector label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }
  
  .data-preview {
    overflow-x: auto;
    margin-bottom: 1.5rem;
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
    position: sticky;
    top: 0;
    background-color: var(--light-bg);
    z-index: 10;
  }
  
  .column-mapping {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .source-column {
    font-weight: 600;
  }
  
  .auto-mapping {
    font-size: 0.875rem;
    color: var(--primary);
  }
  
  .mapping-skip {
    color: var(--light-text-muted);
    font-style: italic;
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
  }
  
  .progress-container {
    margin-bottom: 1.5rem;
  }
  
  .progress-bar {
    height: 8px;
    background-color: var(--light-bg-darker);
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 0.5rem;
  }
  
  .progress-fill {
    height: 100%;
    background-color: var(--primary);
    border-radius: 4px;
    transition: width 0.3s ease;
  }
  
  .progress-text {
    font-size: 0.875rem;
    color: var(--light-text-muted);
    text-align: right;
  }
  
  .form-actions {
    margin-top: 2rem;
    padding-top: 1rem;
    border-top: 1px solid var(--light-border);
    display: flex;
    justify-content: flex-end;
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
  }
  
  .code-example {
    background-color: var(--light-bg-darker);
    padding: 1rem;
    border-radius: var(--radius-md);
    margin-top: 1rem;
  }
  
  .code-example h3 {
    margin-top: 0;
    margin-bottom: 0.75rem;
  }
  
  .code-example p {
    margin-bottom: 0.5rem;
    font-family: var(--font-mono);
    font-size: 0.9rem;
  }
  
  /* Responsive adjustments */
  @media (max-width: 768px) {
    .content-card, .tips-section {
      padding: 1.5rem;
    }
    
    .section-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }
    
    .file-upload-ui {
      flex-direction: column;
      align-items: stretch;
    }
  }
</style>
