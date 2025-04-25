<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  
  // Form data
  let collectorName = "";
  let collectorType = "csv"; // Default type
  let description = "";
  let configData = ""; // Configuration data (e.g., CSV column mappings)
  let isCreating = false;
  let createSuccess = false;
  let errorMessage = "";
  
  // CSV specific settings
  let csvSettings = {
    delimiter: ",",
    hasHeader: true,
    dateFormat: "YYYY-MM-DD",
    columnMappings: [
      { source: "Date", target: "transaction_date", required: true },
      { source: "Description", target: "description", required: true },
      { source: "Amount", target: "amount", required: true },
      { source: "Category", target: "category", required: false },
      { source: "Account", target: "account", required: false }
    ]
  };
  
  // Web scraper settings
  let webSettings = {
    url: "",
    selector: "",
    loginRequired: false,
    username: "",
    password: ""
  };
  
  // API settings
  let apiSettings = {
    url: "",
    method: "GET",
    headers: "",
    body: "",
    authType: "none",
    token: ""
  };
  
  $: {
    // Update configData based on the selected type
    if (collectorType === "csv") {
      configData = JSON.stringify(csvSettings, null, 2);
    } else if (collectorType === "web") {
      configData = JSON.stringify(webSettings, null, 2);
    } else if (collectorType === "api") {
      configData = JSON.stringify(apiSettings, null, 2);
    }
  }
  
  // Examples for different collector types
  let examples = {
    csv: `{
  "delimiter": ",",
  "hasHeader": true,
  "dateFormat": "YYYY-MM-DD",
  "columnMappings": [
    { "source": "日付", "target": "transaction_date", "required": true },
    { "source": "摘要", "target": "description", "required": true },
    { "source": "金額", "target": "amount", "required": true },
    { "source": "カテゴリ", "target": "category", "required": false },
    { "source": "口座", "target": "account", "required": false }
  ]
}`,
    web: `{
  "url": "https://example.com/mybank",
  "selector": ".transaction-row",
  "dataSelectors": {
    "date": ".transaction-date",
    "description": ".transaction-desc",
    "amount": ".transaction-amount"
  },
  "loginRequired": true,
  "loginSelectors": {
    "usernameField": "#username",
    "passwordField": "#password",
    "submitButton": "#login-button"
  }
}`,
    api: `{
  "url": "https://api.example.com/transactions",
  "method": "GET",
  "headers": {
    "Authorization": "Bearer {token}",
    "Content-Type": "application/json"
  },
  "responseMapping": {
    "items": "data.transactions",
    "date": "transaction_date",
    "description": "memo",
    "amount": "value"
  }
}`
  };
  
  function updateConfigWithExample() {
    configData = examples[collectorType];
  }
  
  async function createDataCollector() {
    if (!collectorName.trim()) {
      errorMessage = "コレクタ名を入力してください";
      return;
    }
    
    try {
      // Validate JSON
      JSON.parse(configData);
    } catch (e) {
      errorMessage = "設定データが有効なJSONではありません";
      return;
    }
    
    isCreating = true;
    errorMessage = "";
    
    try {
      // In a real app, this would call a backend function
      // For now, we'll just simulate success
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // Simulated API call to save the collector
      /* 
      await invoke("save_data_collector", {
        name: collectorName,
        type: collectorType,
        description: description,
        config: configData
      });
      */
      
      createSuccess = true;
      
      // Reset form after short delay
      setTimeout(() => {
        collectorName = "";
        description = "";
        createSuccess = false;
        
        // Reset to default config
        if (collectorType === "csv") {
          configData = JSON.stringify(csvSettings, null, 2);
        } else if (collectorType === "web") {
          configData = JSON.stringify(webSettings, null, 2);
        } else if (collectorType === "api") {
          configData = JSON.stringify(apiSettings, null, 2);
        }
      }, 3000);
      
    } catch (error) {
      console.error("Failed to create data collector:", error);
      errorMessage = `コレクタの作成に失敗しました: ${error}`;
    } finally {
      isCreating = false;
    }
  }
  
  function testConfig() {
    // In a real app, this would test the configuration
    console.log("Testing configuration:", configData);
    // For now, we'll just show a success message
    alert("設定のテストは実装予定です。現在はモックモードで動作しています。");
  }
</script>

<div class="page-container">
  <h1>データコレクタの作成</h1>
  <p>外部からのデータを家計簿データベースに取り込むためのデータコレクタを作成します。</p>
  
  <div class="form-container">
    <div class="form-section">
      <h2>基本情報</h2>
      
      <div class="form-group">
        <label for="collectorName">コレクタ名</label>
        <input 
          type="text" 
          id="collectorName" 
          bind:value={collectorName} 
          placeholder="例: 銀行CSVインポート"
          required
        />
      </div>
      
      <div class="form-group">
        <label for="collectorType">データソースタイプ</label>
        <select id="collectorType" bind:value={collectorType}>
          <option value="csv">CSVファイル</option>
          <option value="web">Webスクレイピング</option>
          <option value="api">API連携</option>
        </select>
      </div>
      
      <div class="form-group">
        <label for="description">説明 (オプション)</label>
        <textarea 
          id="description" 
          bind:value={description} 
          placeholder="このデータコレクタの目的や使い方を説明してください"
          rows="3"
        ></textarea>
      </div>
    </div>
    
    <div class="form-section">
      <div class="config-header">
        <h2>設定データ</h2>
        <button class="text small" on:click={updateConfigWithExample}>
          サンプル設定を表示
        </button>
      </div>
      
      <div class="config-description">
        {#if collectorType === "csv"}
          <p>CSVファイルの列と家計簿データの対応関係を設定します。必要な列には「required: true」を設定してください。</p>
        {:else if collectorType === "web"}
          <p>Webページからデータを取得するためのセレクタを設定します。ログインが必要な場合はログイン情報も設定してください。</p>
        {:else if collectorType === "api"}
          <p>APIエンドポイントからデータを取得するための設定を行います。認証が必要な場合は認証情報も設定してください。</p>
        {/if}
      </div>
      
      <div class="form-group">
        <div class="config-editor-header">
          <label for="configData">設定JSON</label>
          <button class="secondary small" on:click={testConfig}>テスト実行</button>
        </div>
        <textarea 
          id="configData" 
          bind:value={configData} 
          class="config-editor"
          rows="15"
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
          データコレクタが正常に作成されました！
        </div>
      {/if}
      
      <button 
        on:click={createDataCollector} 
        disabled={isCreating}
        class="primary"
      >
        {#if isCreating}
          作成中...
        {:else}
          データコレクタを作成
        {/if}
      </button>
    </div>
  </div>
  
  <div class="help-section">
    <h2>データコレクタの使い方</h2>
    
    {#if collectorType === "csv"}
      <div class="help-content">
        <h3>CSVインポートの手順</h3>
        <ol>
          <li>銀行やクレジットカード会社からCSVファイルをダウンロードします。</li>
          <li>ダッシュボードの「データコレクタ」セクションから、作成したコレクタを選択します。</li>
          <li>CSVファイルをアップロードし、インポートを実行します。</li>
          <li>マッピング設定に基づいて、データが家計簿データベースに取り込まれます。</li>
        </ol>
        
        <h3>CSVマッピングのヒント</h3>
        <ul>
          <li>「source」は実際のCSVファイルの列名に合わせてください。</li>
          <li>「target」はデータベースのフィールド名です（transaction_date, description, amount など）。</li>
          <li>日付形式は、YYYYは年、MMは月、DDは日を表します。銀行のCSV形式に合わせて調整してください。</li>
        </ul>
      </div>
    {:else if collectorType === "web"}
      <div class="help-content">
        <h3>Webスクレイピングの設定</h3>
        <ul>
          <li>「url」には取得したいデータが表示されるページのURLを入力します。</li>
          <li>「selector」には繰り返し要素（取引行など）のセレクタを指定します。</li>
          <li>「dataSelectors」には各データ要素（日付、説明、金額）のセレクタを指定します。</li>
          <li>ログインが必要な場合は、「loginRequired」をtrueにし、必要なセレクタを設定します。</li>
        </ul>
        
        <div class="warning-note">
          <p><strong>注意:</strong> セキュリティのため、パスワードなどの認証情報は暗号化して保存されます。ただし、自己責任でご利用ください。</p>
        </div>
      </div>
    {:else if collectorType === "api"}
      <div class="help-content">
        <h3>API連携の設定</h3>
        <ul>
          <li>「url」にはAPIエンドポイントのURLを入力します。</li>
          <li>「method」には HTTP メソッド（GET, POST など）を指定します。</li>
          <li>「headers」には必要なヘッダ情報を設定します。</li>
          <li>「responseMapping」にはAPIレスポンスの構造とデータベースフィールドの対応を設定します。</li>
        </ul>
        
        <h3>API連携の例</h3>
        <p>MoneyForward、Zaim、マネーツリーなどの家計簿サービスのAPIと連携できます。APIの仕様に合わせて設定を調整してください。</p>
      </div>
    {/if}
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
  
  .config-header, .config-editor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .config-description {
    font-size: 0.9rem;
    color: var(--light-text-muted);
    margin-bottom: 1rem;
  }
  
  .config-editor {
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
  
  .help-content {
    margin-top: 1rem;
  }
  
  .help-content h3 {
    font-size: 1.1rem;
    margin: 1.5rem 0 0.75rem 0;
  }
  
  .help-content ul, .help-content ol {
    padding-left: 1.5rem;
    margin-bottom: 1rem;
  }
  
  .help-content li {
    margin-bottom: 0.5rem;
  }
  
  .warning-note {
    background-color: rgba(245, 158, 11, 0.1);
    border-left: 4px solid var(--warning);
    padding: 0.75rem 1rem;
    margin: 1rem 0;
    border-radius: var(--radius-md);
  }
  
  /* Responsive adjustments */
  @media (max-width: 768px) {
    .form-container, .help-section {
      padding: 1.5rem;
    }
  }
</style>
