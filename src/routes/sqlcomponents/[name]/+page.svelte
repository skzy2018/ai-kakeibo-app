<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { apiClient } from "../../../lib/api-client";
  
  // SQLコンポーネントのデータ
  let component: { environment_variables: any; name: any; d3code: any; description: any; sql: any; } | null = null;
  let isLoading = true;
  let errorMessage = "";
  
  // 環境変数の編集用
  let environmentVariables: any[] = [];
  let isEditingEnvVars = false;
  let envVarsChanged = false;
  
  // 実行結果
  let sqlResult: { data: string | any[]; columns: any; } | null = null;
  let isExecuting = false;
  let executeError = "";
  
  // iframe参照
  let iframeRef: HTMLIFrameElement;
  
  onMount(async () => {
    await loadComponent();
  });
  
  async function loadComponent() {
    try {
      isLoading = true;
      errorMessage = "";
      
      const componentName = $page.params.name;
      
      //const { invoke } = await import('@tauri-apps/api/core');
      //const result = await invoke('get_sql_component', { name: componentName });
      const result = await apiClient.getSqlComponent(componentName);

      // APIから返ってきた結果を安全にパースする
      let parsedResult;
      if (typeof result === 'string') {
        try {
          // 単純にJSONとしてパース
          parsedResult = JSON.parse(result);
        } catch (e) {
          console.error("Initial JSON parse failed:", e);
          try {
            // シングルクォートの問題を回避するため文字列を手動でクリーン
            // シングルクォートをダブルクォートに置換してJSON形式に修正
            const cleanedResult = result
              .replace(/'/g, '"')
              .replace(/\\'/g, "\\'") // エスケープされたシングルクォートは保持
              .trim();
            parsedResult = JSON.parse(cleanedResult);
          } catch (e2) {
            console.error("Failed to parse cleaned JSON:", e2);
            parsedResult = { success: false, error: "JSON解析エラー" };
          }
        }
      } else {
        parsedResult = result;
      }
      
      if (!parsedResult.success) {
        errorMessage = parsedResult.error || "コンポーネントの読み込みに失敗しました";
        return;
      }
      
      component = parsedResult.component;
      
      // 環境変数の初期化
      if (component?.environment_variables) {
        environmentVariables = [...component.environment_variables];
      } else {
        environmentVariables = [];
      }
      
    } catch (error) {
      console.error("Failed to load SQL component:", error);
      errorMessage = `コンポーネントの読み込みに失敗しました: ${error}`;
    } finally {
      isLoading = false;
    }
  }
  
  async function runSQLComponent() {
    if (!component) return;
    if (!component.name) return;
    
    try {
      isExecuting = true;
      executeError = "";
      
      // 環境変数をオブジェクトに変換
      const envVars:any = {};
      environmentVariables.forEach((v: { name: string; defaultValue: any; }) => {
        if (v.name) {
          envVars[v.name] = v.defaultValue;
        }
      });
      
      //const { invoke } = await import('@tauri-apps/api/core');
      //const result = await invoke('run_sql_component', {
      //  name: component.name,
      //  envVars: envVars
      //});
      const result = await apiClient.runSqlComponent(component.name, envVars);
      
      const parsedResult = typeof result === 'string' ? JSON.parse(result) : result;
      
      if (!parsedResult.success) {
        executeError = parsedResult.error || "SQLの実行に失敗しました";
        return;
      }
      
      sqlResult = parsedResult;
      
      // D3.jsで可視化
      setTimeout(() => {
        if (iframeRef && iframeRef.contentWindow && component?.d3code && sqlResult?.data) {
          const d3HTML = getD3HTML(sqlResult.data, component.d3code);
          iframeRef.contentWindow.document.open();
          iframeRef.contentWindow.document.write(d3HTML);
          iframeRef.contentWindow.document.close();
        }
      }, 100);
      
    } catch (error) {
      console.error("Failed to run SQL component:", error);
      executeError = `SQLの実行に失敗しました: ${error}`;
    } finally {
      isExecuting = false;
    }
  }
  
  function getD3HTML(data: string | any[], d3code: any) {
    return `
      <!DOCTYPE html>
      <html>
      <head>
        <title>D3 Visualization</title>
        <meta charset="utf-8">
        <script src="https://d3js.org/d3.v7.min.js"><\/script>
        <style>
          body {
            font-family: sans-serif;
            margin: 0;
            padding: 0;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100%;
            width: 100%;
          }
          #visualization {
            width: 100%;
            height: 100%;
            display: flex;
            justify-content: center;
            align-items: center;
          }
        </style>
      </head>
      <body>
        <div id="visualization"></div>
        <script>
          // SQLクエリ結果のデータ
          const data = ${JSON.stringify(data)};
          // D3コード実行
          ${d3code}
        <\/script>
      </body>
      </html>
    `;
  }
  
  function startEditingEnvVars() {
    isEditingEnvVars = true;
  }
  
  function cancelEditingEnvVars() {
    // 元の値に戻す
    if (component?.environment_variables) {
      environmentVariables = [...component.environment_variables];
    } else {
      environmentVariables = [];
    }
    isEditingEnvVars = false;
    envVarsChanged = false;
  }
  
  function handleEnvVarChange() {
    envVarsChanged = true;
  }
  
  async function saveEnvironmentVariables() {
    try {
      // 環境変数を保存する処理
      // ここでは簡易的に、コンポーネント全体を保存し直す
      if (!component) return;
      
      const updatedComponent = {
        ...component,
        environment_variables: environmentVariables
      };
      
      const { invoke } = await import('@tauri-apps/api/core');
      const result = await invoke('save_sql_component', { component: updatedComponent });
      
      const parsedResult = typeof result === 'string' ? JSON.parse(result) : result;
      
      if (!parsedResult.success) {
        errorMessage = parsedResult.error || "環境変数の保存に失敗しました";
        return;
      }
      
      // 保存成功
      component = updatedComponent;
      isEditingEnvVars = false;
      envVarsChanged = false;
      
    } catch (error) {
      console.error("Failed to save environment variables:", error);
      errorMessage = `環境変数の保存に失敗しました: ${error}`;
    }
  }
</script>

<svelte:head>
  <script src="https://d3js.org/d3.v7.min.js" defer></script>
</svelte:head>

<div class="page-container">
  {#if isLoading}
    <div class="loading">
      <p>コンポーネントを読み込み中...</p>
    </div>
  {:else if errorMessage}
    <div class="error-message">
      <p>{errorMessage}</p>
      <button class="secondary" on:click={loadComponent}>再試行</button>
    </div>
  {:else if component}
    <h1>{component.name}</h1>
    
    {#if component.description}
      <div class="description">
        <h2>説明</h2>
        <p>{component.description}</p>
      </div>
    {/if}
    
    <div class="component-container">
      <div class="environment-vars-section">
        <div class="section-header">
          <h2>環境変数</h2>
          {#if !isEditingEnvVars}
            <button class="secondary small" on:click={startEditingEnvVars}>
              <span class="material-icons">edit</span>編集
            </button>
          {/if}
        </div>
        
        {#if isEditingEnvVars}
          <div class="env-vars-editor">
            {#if environmentVariables.length === 0}
              <p>環境変数はありません</p>
            {:else}
              <div class="env-vars-grid">
                {#each environmentVariables as envVar, i}
                  <div class="env-var-row">
                    <div class="env-var-name">{envVar.name}</div>
                    <input
                      type="text"
                      bind:value={envVar.defaultValue}
                      on:input={handleEnvVarChange}
                      placeholder="値を入力してください"
                    />
                  </div>
                {/each}
              </div>
            {/if}
            
            <div class="actions">
              <button class="secondary" on:click={cancelEditingEnvVars}>キャンセル</button>
              <button class="primary" on:click={saveEnvironmentVariables} disabled={!envVarsChanged}>保存</button>
            </div>
          </div>
        {:else}
          {#if environmentVariables.length === 0}
            <p>環境変数はありません</p>
          {:else}
            <div class="env-vars-grid">
              {#each environmentVariables as envVar}
                <div class="env-var-row">
                  <div class="env-var-name">{envVar.name}</div>
                  <div class="env-var-value">{envVar.defaultValue || ''}</div>
                </div>
              {/each}
            </div>
          {/if}
        {/if}
      </div>
      
      <div class="execution-section">
        <div class="section-header">
          <h2>実行</h2>
          <button class="primary" on:click={runSQLComponent} disabled={isExecuting}>
            {#if isExecuting}
              実行中...
            {:else}
              <span class="material-icons">play_arrow</span>実行
            {/if}
          </button>
        </div>
        
        {#if executeError}
          <div class="error-message">
            <p>{executeError}</p>
          </div>
        {/if}
        
        {#if sqlResult}
          <div class="visualization-container">
            <iframe
              bind:this={iframeRef}
              title="D3 Visualization"
              sandbox="allow-scripts allow-same-origin"
              width="100%"
              height="400"
              frameborder="0"
            ></iframe>
          </div>
          
          <div class="result-data">
            <h3>実行結果</h3>
            {#if sqlResult.data && sqlResult.data.length > 0}
              <div class="table-container">
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
                          <td>{(row as Record<string, any>)[column] !== undefined && (row as Record<string, any>)[column] !== null ? (row as Record<string, any>)[column] : ''}</td>
                        {/each}
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            {:else}
              <p>結果がありません</p>
            {/if}
          </div>
        {/if}
      </div>
      
      <div class="code-section">
        <div class="sql-container">
          <h3>SQLクエリ</h3>
          <pre><code>{component.sql || ''}</code></pre>
        </div>
        
        <div class="d3-container">
          <h3>D3.jsコード</h3>
          <pre><code>{component.d3code || ''}</code></pre>
        </div>
      </div>
    </div>
  {:else}
    <div class="error-message">
      <p>コンポーネントが見つかりませんでした</p>
      <a href="/sqlcomponents" class="button secondary">一覧に戻る</a>
    </div>
  {/if}
</div>

<style>
  .page-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1rem;
  }
  
  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 300px;
  }
  
  .error-message {
    background-color: rgba(239, 68, 68, 0.1);
    border-left: 4px solid var(--danger);
    padding: 1rem;
    color: var(--danger);
    border-radius: var(--radius-md);
    margin: 1rem 0;
  }
  
  .component-container {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    margin: 1.5rem 0;
  }
  
  .description {
    background-color: white;
    border-radius: var(--radius-lg);
    padding: 1.5rem;
    box-shadow: var(--shadow-md);
  }
  
  .description h2 {
    font-size: 1.25rem;
    margin-top: 0;
    margin-bottom: 0.5rem;
    border-bottom: 1px solid var(--light-border);
    padding-bottom: 0.5rem;
  }
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }
  
  .section-header h2 {
    margin: 0;
    font-size: 1.25rem;
  }
  
  .environment-vars-section, .execution-section, .code-section {
    background-color: white;
    border-radius: var(--radius-lg);
    padding: 1.5rem;
    box-shadow: var(--shadow-md);
  }
  
  .env-vars-grid {
    display: grid;
    gap: 0.75rem;
  }
  
  .env-var-row {
    display: grid;
    grid-template-columns: 1fr 2fr;
    gap: 1rem;
    align-items: center;
  }
  
  .env-var-name {
    font-weight: 500;
    font-family: var(--font-mono);
    background-color: var(--light-bg-darker);
    padding: 0.5rem;
    border-radius: var(--radius-sm);
  }
  
  .env-var-value {
    font-family: var(--font-mono);
    background-color: var(--light-bg);
    padding: 0.5rem;
    border-radius: var(--radius-sm);
  }
  
  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 1.5rem;
  }
  
  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.25rem;
    padding: 0.5rem 1rem;
    border-radius: var(--radius-md);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  button.small {
    padding: 0.25rem 0.5rem;
    font-size: 0.875rem;
  }
  
  button.primary {
    background-color: var(--primary);
    color: white;
    border: none;
  }
  
  button.primary:hover:not(:disabled) {
    background-color: var(--primary-dark);
  }
  
  button.primary:disabled {
    background-color: var(--light-bg-darker);
    color: var(--light-text-muted);
    cursor: not-allowed;
  }
  
  button.secondary {
    background-color: var(--light-bg);
    color: var(--light-text);
    border: 1px solid var(--light-border);
  }
  
  button.secondary:hover:not(:disabled) {
    background-color: var(--light-bg-darker);
  }
  
  .material-icons {
    font-size: 1rem;
  }
  
  .visualization-container {
    margin: 1rem 0;
    border: 1px solid var(--light-border);
    border-radius: var(--radius-md);
    overflow: hidden;
  }
  
  .result-data {
    margin-top: 1.5rem;
  }
  
  .table-container {
    overflow-x: auto;
    max-height: 300px;
    border: 1px solid var(--light-border);
    border-radius: var(--radius-md);
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.9rem;
  }
  
  th, td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid var(--light-border);
  }
  
  th {
    background-color: var(--light-bg);
    font-weight: 600;
    position: sticky;
    top: 0;
    z-index: 10;
  }
  
  tr:nth-child(even) {
    background-color: var(--light-bg);
  }
  
  pre {
    background-color: var(--light-bg);
    padding: 1rem;
    border-radius: var(--radius-md);
    overflow-x: auto;
    margin: 0;
  }
  
  code {
    font-family: var(--font-mono);
    font-size: 0.9rem;
  }
  
  .code-section {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1.5rem;
  }
  
  @media (min-width: 768px) {
    .code-section {
      grid-template-columns: 1fr 1fr;
    }
  }
  
  .button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem 1rem;
    border-radius: var(--radius-md);
    font-weight: 500;
    text-decoration: none;
    transition: all 0.2s ease;
    cursor: pointer;
  }
  
  .button.secondary {
    background-color: var(--light-bg-darker);
    color: var(--light-text);
    border: 1px solid var(--light-border);
  }
  
  .button.secondary:hover {
    background-color: var(--light-bg-darkest);
  }
</style>
