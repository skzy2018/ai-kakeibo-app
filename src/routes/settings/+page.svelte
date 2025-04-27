<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  
  // Settings categories
  let activeCategory = "general";
  let categories = [
    { id: "general", name: "一般設定", icon: "⚙️" },
    { id: "appearance", name: "表示設定", icon: "🎨" },
    { id: "data", name: "データ管理", icon: "💾" },
    { id: "about", name: "アプリ情報", icon: "ℹ️" }
  ];
  
  // Settings form data
  let generalSettings = {
    defaultCurrency: "JPY",
    dateFormat: "YYYY-MM-DD",
    startDayOfMonth: "1",
    language: "ja"
  };
  
  let appearanceSettings = {
    theme: "light",
    fontSize: "medium",
    dashboardLayout: "grid"
  };
  
  let dataSettings = {
    backupFrequency: "weekly",
    backupLocation: "./backups",
    retainBackups: "5",
    exportFormat: "csv"
  };
  
  // Application info
  let appInfo = {
    name: "AI-KAKEIBO-APP",
    version: "0.1.0",
    lastUpdated: "2025-04-26",
    database: {
      size: "12.5 MB",
      tables: 6,
      records: 1240
    }
  };
  
  let isSaving = false;
  let saveSuccess = false;
  let errorMessage = "";
  
  onMount(async () => {
    try {
      // In a real app, we would load the actual settings from storage
      // For now, we'll use mock data
      
      // Simulate loading settings
      await new Promise(resolve => setTimeout(resolve, 300));
      
      // Simulated API call
      /*
      const settings = await invoke("get_app_settings");
      if (settings) {
        generalSettings = { ...generalSettings, ...settings.general };
        appearanceSettings = { ...appearanceSettings, ...settings.appearance };
        dataSettings = { ...dataSettings, ...settings.data };
      }
      */
      
    } catch (error) {
      console.error("Failed to load settings:", error);
      errorMessage = `設定の読み込みに失敗しました: ${error}`;
    }
  });
  
  async function saveSettings() {
    isSaving = true;
    errorMessage = "";
    
    try {
      // In a real app, we would save settings to storage
      // For now, we'll just simulate success
      await new Promise(resolve => setTimeout(resolve, 800));
      
      // Simulated API call
      /*
      await invoke("save_app_settings", {
        settings: {
          general: generalSettings,
          appearance: appearanceSettings,
          data: dataSettings
        }
      });
      */
      
      saveSuccess = true;
      
      // Reset success message after delay
      setTimeout(() => {
        saveSuccess = false;
      }, 3000);
    } catch (error) {
      console.error("Failed to save settings:", error);
      errorMessage = `設定の保存に失敗しました: ${error}`;
    } finally {
      isSaving = false;
    }
  }
  
  async function exportData(format: string) {
    try {
      alert(`${format.toUpperCase()}形式でデータをエクスポートします...\nこの機能は実際のアプリで実装されます。`);
      
      // In a real app, we would call a backend function
      /*
      await invoke("export_data", {
        format: format
      });
      */
    } catch (error) {
      console.error("Failed to export data:", error);
      alert(`エクスポートに失敗しました: ${error}`);
    }
  }
  
  async function importData() {
    try {
      alert("データインポート機能を開始します...\nこの機能は実際のアプリで実装されます。");
      
      // In a real app, we would call a backend function
      /*
      await invoke("import_data");
      */
    } catch (error) {
      console.error("Failed to import data:", error);
      alert(`インポートに失敗しました: ${error}`);
    }
  }
  
  async function resetApp() {
    if (confirm("本当にアプリケーションをリセットしますか？\nすべてのデータが削除され、この操作は元に戻せません。")) {
      try {
        alert("アプリケーションリセット機能を実行します...\nこの機能は実際のアプリで実装されます。");
        
        // In a real app, we would call a backend function
        /*
        await invoke("reset_app");
        */
      } catch (error) {
        console.error("Failed to reset app:", error);
        alert(`リセットに失敗しました: ${error}`);
      }
    }
  }
  
  function setActiveCategory(categoryId: string) {
    activeCategory = categoryId;
  }
  
  function formatTimestamp(date: string) {
    return new Date(date).toLocaleString('ja-JP');
  }
</script>

<div class="page-container">
  <h1>設定</h1>
  <p>アプリケーションの設定を変更します。</p>
  
  <div class="settings-container">
    <div class="settings-sidebar">
      <ul>
        {#each categories as category}
          <li class:active={activeCategory === category.id}>
            <button on:click={() => setActiveCategory(category.id)}>
              <span class="category-icon">{category.icon}</span>
              <span class="category-name">{category.name}</span>
            </button>
          </li>
        {/each}
      </ul>
    </div>
    
    <div class="settings-content">
      {#if errorMessage}
        <div class="error-message">
          {errorMessage}
        </div>
      {/if}
      
      {#if saveSuccess}
        <div class="success-message">
          設定が正常に保存されました。
        </div>
      {/if}
      
      {#if activeCategory === "general"}
        <div class="settings-section">
          <h2>一般設定</h2>
          <div class="settings-form">
            <div class="form-group">
              <label for="defaultCurrency">通貨</label>
              <select id="defaultCurrency" bind:value={generalSettings.defaultCurrency}>
                <option value="JPY">円 (JPY)</option>
                <option value="USD">米ドル (USD)</option>
                <option value="EUR">ユーロ (EUR)</option>
                <option value="GBP">英ポンド (GBP)</option>
              </select>
            </div>
            
            <div class="form-group">
              <label for="dateFormat">日付形式</label>
              <select id="dateFormat" bind:value={generalSettings.dateFormat}>
                <option value="YYYY-MM-DD">YYYY-MM-DD</option>
                <option value="DD/MM/YYYY">DD/MM/YYYY</option>
                <option value="MM/DD/YYYY">MM/DD/YYYY</option>
                <option value="YYYY年MM月DD日">YYYY年MM月DD日</option>
              </select>
            </div>
            
            <div class="form-group">
              <label for="startDayOfMonth">月の開始日</label>
              <select id="startDayOfMonth" bind:value={generalSettings.startDayOfMonth}>
                <option value="1">1日</option>
                <option value="15">15日</option>
                <option value="20">20日</option>
                <option value="25">25日</option>
                <option value="28">28日</option>
              </select>
              <div class="form-hint">月次レポートの集計期間の開始日を指定します。</div>
            </div>
            
            <div class="form-group">
              <label for="language">言語</label>
              <select id="language" bind:value={generalSettings.language}>
                <option value="ja">日本語</option>
                <option value="en">English</option>
                <option value="zh">中文</option>
                <option value="ko">한국어</option>
              </select>
            </div>
          </div>
        </div>
      {:else if activeCategory === "appearance"}
        <div class="settings-section">
          <h2>表示設定</h2>
          <div class="settings-form">
            <div class="form-group">
              <label for="theme">テーマ</label>
              <select id="theme" bind:value={appearanceSettings.theme}>
                <option value="light">ライト</option>
                <option value="dark">ダーク</option>
                <option value="system">システム設定に合わせる</option>
              </select>
            </div>
            
            <div class="form-group">
              <label for="fontSize">フォントサイズ</label>
              <select id="fontSize" bind:value={appearanceSettings.fontSize}>
                <option value="small">小</option>
                <option value="medium">中</option>
                <option value="large">大</option>
              </select>
            </div>
            
            <div class="form-group">
              <label for="dashboardLayout">ダッシュボードレイアウト</label>
              <select id="dashboardLayout" bind:value={appearanceSettings.dashboardLayout}>
                <option value="grid">グリッド</option>
                <option value="list">リスト</option>
                <option value="compact">コンパクト</option>
              </select>
            </div>
          </div>
        </div>
      {:else if activeCategory === "data"}
        <div class="settings-section">
          <h2>データ管理</h2>
          <div class="settings-form">
            <div class="form-group">
              <label for="backupFrequency">バックアップ頻度</label>
              <select id="backupFrequency" bind:value={dataSettings.backupFrequency}>
                <option value="daily">毎日</option>
                <option value="weekly">毎週</option>
                <option value="monthly">毎月</option>
                <option value="never">手動</option>
              </select>
            </div>
            
            <div class="form-group">
              <label for="backupLocation">バックアップ保存先</label>
              <div class="input-with-button">
                <input 
                  type="text" 
                  id="backupLocation" 
                  bind:value={dataSettings.backupLocation} 
                  placeholder="./backups"
                />
                <button class="secondary small">参照...</button>
              </div>
            </div>
            
            <div class="form-group">
              <label for="retainBackups">保持するバックアップ数</label>
              <input 
                type="number" 
                id="retainBackups" 
                bind:value={dataSettings.retainBackups} 
                min="1" 
                max="30"
              />
              <div class="form-hint">古いバックアップは自動的に削除されます。</div>
            </div>
            
            <div class="form-group">
              <label for="exportFormat">エクスポート形式</label>
              <select id="exportFormat" bind:value={dataSettings.exportFormat}>
                <option value="csv">CSV</option>
                <option value="json">JSON</option>
                <option value="excel">Excel</option>
              </select>
            </div>
            
            <div class="action-buttons">
              <button 
                class="secondary"
                on:click={() => exportData(dataSettings.exportFormat)}
              >
                データをエクスポート
              </button>
              <button 
                class="secondary"
                on:click={importData}
              >
                データをインポート
              </button>
              <button 
                class="danger"
                on:click={resetApp}
              >
                アプリをリセット
              </button>
            </div>
          </div>
        </div>
      {:else if activeCategory === "about"}
        <div class="settings-section">
          <h2>アプリケーション情報</h2>
          
          <div class="app-info">
            <div class="app-logo">AI</div>
            <div class="app-details">
              <h3>{appInfo.name}</h3>
              <p class="app-version">バージョン {appInfo.version}</p>
              <p class="app-update">最終更新: {appInfo.lastUpdated}</p>
            </div>
          </div>
          
          <div class="info-section">
            <h3>データベース情報</h3>
            <ul class="info-list">
              <li>
                <span class="info-label">サイズ:</span>
                <span class="info-value">{appInfo.database.size}</span>
              </li>
              <li>
                <span class="info-label">テーブル数:</span>
                <span class="info-value">{appInfo.database.tables}</span>
              </li>
              <li>
                <span class="info-label">レコード数:</span>
                <span class="info-value">{appInfo.database.records}</span>
              </li>
            </ul>
          </div>
          
          <div class="info-section">
            <h3>開発者情報</h3>
            <p>AI-KAKEIBO-APPは、あなたの家計管理をより簡単かつ効率的にするために開発されました。カスタマイズ可能なダッシュボード、データ分析、AIアシスタントを通じて、家計の状況を常に把握できます。</p>
          </div>
          
          <div class="info-section">
            <h3>ライセンス</h3>
            <p>© 2025 AI-KAKEIBO-APP Team</p>
            <p>このアプリケーションはMITライセンスの下で提供されています。</p>
          </div>
        </div>
      {/if}
      
      {#if activeCategory !== "about"}
        <div class="form-actions">
          <button 
            class="primary"
            on:click={saveSettings}
            disabled={isSaving}
          >
            {#if isSaving}
              保存中...
            {:else}
              設定を保存
            {/if}
          </button>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .page-container {
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .settings-container {
    display: flex;
    margin-top: 1.5rem;
    background-color: white;
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    overflow: hidden;
  }
  
  .settings-sidebar {
    width: 220px;
    background-color: var(--light-bg-darker);
    border-right: 1px solid var(--light-border);
  }
  
  .settings-sidebar ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  
  .settings-sidebar li {
    margin: 0;
  }
  
  .settings-sidebar button {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 1rem;
    text-align: left;
    background: none;
    border: none;
    color: var(--light-text);
    cursor: pointer;
    border-bottom: 1px solid var(--light-border);
    transition: background-color 0.2s;
  }
  
  .settings-sidebar button:hover {
    background-color: rgba(0, 0, 0, 0.05);
  }
  
  .settings-sidebar li.active button {
    background-color: white;
    color: var(--primary);
    font-weight: 600;
  }
  
  .category-icon {
    margin-right: 0.75rem;
    font-size: 1.25rem;
    width: 1.5rem;
    text-align: center;
  }
  
  .settings-content {
    flex: 1;
    padding: 2rem;
  }
  
  .settings-section h2 {
    margin-top: 0;
    margin-bottom: 1.5rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--light-border);
  }
  
  .settings-form {
    max-width: 600px;
  }
  
  .form-group {
    margin-bottom: 1.5rem;
  }
  
  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
  }
  
  .form-hint {
    margin-top: 0.25rem;
    font-size: 0.875rem;
    color: var(--light-text-muted);
  }
  
  .form-actions {
    margin-top: 2rem;
    padding-top: 1rem;
    border-top: 1px solid var(--light-border);
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
  
  .input-with-button {
    display: flex;
    gap: 0.5rem;
  }
  
  .input-with-button input {
    flex: 1;
  }
  
  .action-buttons {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    margin-top: 2rem;
  }
  
  /* App Info styles */
  .app-info {
    display: flex;
    align-items: center;
    margin-bottom: 2rem;
  }
  
  .app-logo {
    width: 80px;
    height: 80px;
    background-color: var(--primary);
    color: white;
    border-radius: var(--radius-lg);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    font-weight: bold;
    margin-right: 1.5rem;
  }
  
  .app-details h3 {
    margin: 0;
    font-size: 1.5rem;
  }
  
  .app-version, .app-update {
    margin: 0.25rem 0;
    color: var(--light-text-muted);
  }
  
  .info-section {
    margin-bottom: 2rem;
  }
  
  .info-section h3 {
    font-size: 1.1rem;
    margin-bottom: 0.75rem;
  }
  
  .info-list {
    list-style: none;
    padding: 0;
  }
  
  .info-list li {
    display: flex;
    justify-content: space-between;
    padding: 0.5rem 0;
    border-bottom: 1px solid var(--light-border);
  }
  
  .info-label {
    font-weight: 500;
  }
  
  .info-value {
    color: var(--light-text-muted);
  }
  
  /* Responsive adjustments */
  @media (max-width: 768px) {
    .settings-container {
      flex-direction: column;
    }
    
    .settings-sidebar {
      width: 100%;
      border-right: none;
      border-bottom: 1px solid var(--light-border);
    }
    
    .settings-sidebar ul {
      display: flex;
      overflow-x: auto;
    }
    
    .settings-sidebar li {
      flex-shrink: 0;
    }
    
    .settings-sidebar button {
      padding: 0.75rem;
      border-right: 1px solid var(--light-border);
      border-bottom: none;
    }
    
    .action-buttons {
      flex-direction: column;
    }
    
    .action-buttons button {
      width: 100%;
    }
  }
</style>
