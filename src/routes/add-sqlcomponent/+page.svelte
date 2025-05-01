<script lang="ts">
  import { onMount } from "svelte";
  import { browser } from "$app/environment";
  import { apiClient } from "../../lib/api-client";
  
  // Form data
  let componentName = "";
  let sqlQuery = "";
  let d3Code = "";
  let description = "";
  let environmentVariables: {name: string, defaultValue: string}[] = [{ name: "", defaultValue: "" }];
  let isCreating = false;
  let createSuccess = false;
  let errorMessage = "";
  let previewResult = "";
  let aceEditorSQL: any;
  let aceEditorD3: any;
  
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
  
  // Sample SQL queries
  const sampleSQL = `-- 月別支出合計 (棒グラフ)
SELECT 
  strftime('%Y-%m', transaction_date) as month,
  SUM(CASE WHEN amount < 0 THEN ABS(amount) ELSE 0 END) as total_expense
FROM transactions
WHERE transaction_date >= $startDate AND transaction_date <= $endDate
GROUP BY month
ORDER BY month
LIMIT 12;`;

  // Sample D3 code
  const sampleD3 = `// D3.js visualization code
// This example creates a bar chart with the SQL query results
(function(data) {
  // Clear any previous svg
  d3.select("#visualization").html("");
  
  // Set the dimensions and margins of the graph
  const margin = {top: 30, right: 30, bottom: 70, left: 60},
      width = 600 - margin.left - margin.right,
      height = 400 - margin.top - margin.bottom;
  
  // Append the svg object to the body of the page
  const svg = d3.select("#visualization")
    .append("svg")
      .attr("width", width + margin.left + margin.right)
      .attr("height", height + margin.top + margin.bottom)
    .append("g")
      .attr("transform", \`translate(\${margin.left},\${margin.top})\`);
  
  // X axis
  const x = d3.scaleBand()
    .range([0, width])
    .domain(data.map(d => d.month))
    .padding(0.2);
  svg.append("g")
    .attr("transform", \`translate(0,\${height})\`)
    .call(d3.axisBottom(x))
    .selectAll("text")
      .attr("transform", "translate(-10,0)rotate(-45)")
      .style("text-anchor", "end");
  
  // Add Y axis
  const y = d3.scaleLinear()
    .domain([0, d3.max(data, d => +d.total_expense)])
    .range([height, 0]);
  svg.append("g")
    .call(d3.axisLeft(y));
  
  // Bars
  svg.selectAll("mybar")
    .data(data)
    .enter()
    .append("rect")
      .attr("x", d => x(d.month))
      .attr("y", d => y(d.total_expense))
      .attr("width", x.bandwidth())
      .attr("height", d => height - y(d.total_expense))
      .attr("fill", "#69b3a2");
})(data);`;
  
  onMount(async () => {
    if (browser) {
      const aceModule = await import('ace-builds');
      await import('ace-builds/src-noconflict/mode-sql');
      await import('ace-builds/src-noconflict/mode-javascript');
      await import('ace-builds/src-noconflict/theme-monokai');
      
      // Initialize SQL editor
      aceEditorSQL = aceModule.default.edit('sqlEditor');
      aceEditorSQL.setTheme('ace/theme/monokai');
      aceEditorSQL.session.setMode('ace/mode/sql');
      aceEditorSQL.setValue(sampleSQL);
      aceEditorSQL.on('change', function() {
        sqlQuery = aceEditorSQL.getValue();
      });
      
      // Initialize D3 code editor
      aceEditorD3 = aceModule.default.edit('d3Editor');
      aceEditorD3.setTheme('ace/theme/monokai');
      aceEditorD3.session.setMode('ace/mode/javascript');
      aceEditorD3.setValue(sampleD3);
      aceEditorD3.on('change', function() {
        d3Code = aceEditorD3.getValue();
      });
      
      // Set initial values
      sqlQuery = sampleSQL;
      d3Code = sampleD3;
    }
  });
  
  function addEnvironmentVariable() {
    environmentVariables = [...environmentVariables, { name: "", defaultValue: "" }];
  }
  
  function removeEnvironmentVariable(index: number) {
    environmentVariables = environmentVariables.filter((_, i) => i !== index);
  }
  
  // Safe invoke function that works with TypeScript
  async function safeInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
    try {
      // @ts-ignore - Tauri API might not be fully typed
      const api = await import('@tauri-apps/api');
      // @ts-ignore - invoke exists but TypeScript doesn't know about it
      let ret = await api.invoke(command, args);
      console.log('safeInvoke',ret)
      return  ret;
    } catch (error) {
      console.error(`Failed to invoke ${command}:`, error);
      throw error;
    }
  }

  async function createSQLComponent() {
    if (!componentName.trim() || !sqlQuery.trim() || !d3Code.trim()) {
      errorMessage = "コンポーネント名、SQL、D3コードを入力してください";
      return;
    }
    
    // Validate component name (only alphanumeric and no spaces)
    const nameRegex = /^[A-Za-z0-9_-]+$/;
    if (!nameRegex.test(componentName)) {
      errorMessage = "コンポーネント名は英数字とアンダースコア、ハイフンのみ使用できます";
      return;
    }
    
    // Validate environment variables
    const validEnvironmentVariables = environmentVariables.filter(v => v.name.trim() !== "");
    const duplicateNames = validEnvironmentVariables
      .map(v => v.name)
      .filter((name, index, array) => array.indexOf(name) !== index);
    
    if (duplicateNames.length > 0) {
      errorMessage = `環境変数名が重複しています: ${duplicateNames.join(", ")}`;
      return;
    }
    
    isCreating = true;
    errorMessage = "";
    
    try {
      // Create the SQL component object
      const sqlComponent = {
        name: componentName,
        environment_variables: validEnvironmentVariables,
        description: description,
        sql: sqlQuery,
        d3code: d3Code
      };
      
      // Call the Tauri backend to save the component
      //const { invoke } = await import('@tauri-apps/api/core');
      //console.log('sqlComponent',sqlComponent)
      //const result = await invoke('save_sql_component', { component: sqlComponent });
      console.log('result??=',sqlComponent)
      //const result = await apiClient.saveSqlComponent({component:sqlComponent});
      const result = await apiClient.saveSqlComponent(sqlComponent);
      console.log('result=',result)

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
            const cleanedResult = (result as string)
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
      
      if (parsedResult.success) {
        createSuccess = true;
        
        // Reset form after short delay
        setTimeout(() => {
          componentName = "";
          description = "";
          environmentVariables = [{ name: "", defaultValue: "" }];
          aceEditorSQL.setValue(sampleSQL);
          aceEditorD3.setValue(sampleD3);
          createSuccess = false;
        }, 3000);
      } else {
        errorMessage = parsedResult.error || "コンポーネントの保存中に不明なエラーが発生しました";
      }
      
    } catch (error) {
      console.error("Failed to create SQL component:", error);
      errorMessage = `コンポーネントの作成に失敗しました: ${error}`;
    } finally {
      isCreating = false;
    }
  }
  
  let iframeRef: HTMLIFrameElement;
  
  async function previewSQLComponent() {
    if (!sqlQuery.trim() || !d3Code.trim()) {
      errorMessage = "SQLとD3コードを入力してください";
      return;
    }
    
    errorMessage = "";
    
    try {
      // In a real implementation, this would call the Tauri backend
      // For demonstration, we'll simulate a SQL query execution
      
      // Mock data based on the SQL query
      let mockData;
      if (sqlQuery.includes('month') && sqlQuery.includes('total_expense')) {
        mockData = [
          { month: '2025-01', total_expense: 125000 },
          { month: '2025-02', total_expense: 98000 },
          { month: '2025-03', total_expense: 110000 },
          { month: '2025-04', total_expense: 145000 }
        ];
      } else if (sqlQuery.includes('category_name')) {
        mockData = [
          { category_name: '食費', total_amount: 45000 },
          { category_name: '住居費', total_amount: 85000 },
          { category_name: '光熱費', total_amount: 25000 },
          { category_name: '交通費', total_amount: 15000 }
        ];
      } else {
        mockData = [
          { column1: 'データ1', column2: 100 },
          { column1: 'データ2', column2: 200 }
        ];
      }
      
      // Create a complete HTML document for the iframe
      const previewHTML = `
        <!DOCTYPE html>
        <html>
        <head>
          <title>D3 Preview</title>
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
            // The data from SQL query results
            const data = ${JSON.stringify(mockData)};
            // Execute the D3 code
            ${d3Code}
          <\/script>
        </body>
        </html>
      `;
      
      // Set a flag to indicate we have a preview to show
      previewResult = "ready";
      
      // Set the iframe content after a brief delay to ensure the DOM is updated
      setTimeout(() => {
        if (iframeRef && iframeRef.contentWindow) {
          // Create a blob with the HTML content and use it as the iframe's src
          // This approach avoids using the deprecated document.write() method
          const blob = new Blob([previewHTML], { type: 'text/html' });
          const url = URL.createObjectURL(blob);
          
          // Set the iframe's src to the blob URL
          iframeRef.src = url;
          
          // Clean up the URL object when it's no longer needed
          setTimeout(() => URL.revokeObjectURL(url), 1000);
        }
      }, 100);
      
    } catch (error) {
      console.error("Failed to preview SQL component:", error);
      errorMessage = `プレビューの作成に失敗しました: ${error}`;
    }
  }
</script>

<svelte:head>
  <!--<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/ace/1.9.6/ace.min.css"> -->
  <script src="https://d3js.org/d3.v7.min.js" defer></script>
</svelte:head>

<div class="page-container">
  <h1>SQLコンポーネントの作成</h1>
  <p>家計簿データベースに対するSQLクエリとD3.jsによる可視化を設定し、再利用可能なコンポーネントを作成します。</p>
  
  <div class="form-container">
    <div class="form-section">
      <h2>基本情報</h2>
      
      <div class="form-group">
        <label for="componentName">コンポーネント名</label>
        <input 
          type="text" 
          id="componentName" 
          bind:value={componentName} 
          placeholder="英数字記号のみ（例: monthly_expenses）"
          required
        />
        <p class="input-hint">英数字、アンダースコア、ハイフンのみ使用できます。空白は含まないでください。</p>
      </div>
      
      <div class="form-group">
        <label for="description">説明</label>
        <textarea 
          id="description" 
          bind:value={description} 
          placeholder="このSQLコンポーネントがどのような情報を提供するかを説明してください"
          rows="3"
        ></textarea>
        <p class="input-hint">AIチャットが分析時に参照するため、詳細な説明を記載してください。</p>
      </div>
      
      <div class="form-group">
        <label>環境変数</label>
        <div class="environment-variables">
          {#each environmentVariables as variable, index}
            <div class="env-variable-row">
              <input
                type="text"
                placeholder="変数名（例: startDate）"
                bind:value={variable.name}
              />
              <input
                type="text"
                placeholder="デフォルト値（例: 2025-01-01）"
                bind:value={variable.defaultValue}
              />
              <button class="icon-button" on:click={() => removeEnvironmentVariable(index)}>
                <span class="material-icons">delete</span>
              </button>
            </div>
          {/each}
        </div>
        <button class="secondary small" on:click={addEnvironmentVariable}>
          <span class="material-icons">add</span> 環境変数を追加
        </button>
        <p class="input-hint">SQL内で利用する変数を定義します。SQL内では "$変数名" の形式で参照できます。</p>
      </div>
    </div>
    
    <div class="form-section">
      <h2>SQLクエリ</h2>
      <div class="form-group">
        <label for="sqlEditor">SQLクエリ</label>
        <div id="sqlEditor" class="code-editor"></div>
        <p class="input-hint">家計簿データベースで実行するSQLを定義します。環境変数は "$変数名" の形式で指定できます。</p>
      </div>
    </div>
    
    <div class="form-section">
      <h2>D3.js 可視化コード</h2>
      <div class="form-group">
        <label for="d3Editor">D3コード</label>
        <div id="d3Editor" class="code-editor"></div>
        <p class="input-hint">SQLの実行結果を可視化するD3.jsコードを定義します。SQL結果は "data" という変数で参照できます。</p>
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
      
      <div class="button-group">
        <button 
          on:click={previewSQLComponent} 
          class="secondary"
        >
          プレビュー
        </button>
        
        <button 
          on:click={createSQLComponent} 
          disabled={isCreating}
          class="primary"
        >
          {#if isCreating}
            作成中...
          {:else}
            SQLコンポーネントを登録
          {/if}
        </button>
      </div>
    </div>
  </div>
  
  {#if previewResult}
    <div class="preview-section">
      <h2>プレビュー</h2>
      <div class="preview-container">
        <iframe 
          bind:this={iframeRef} 
          title="D3 Visualization Preview" 
          sandbox="allow-scripts allow-same-origin" 
          width="100%" 
          height="400"
          frameborder="0"
        ></iframe>
      </div>
    </div>
  {/if}
  
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
    padding: 1rem;
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
  
  .input-hint {
    font-size: 0.8rem;
    color: var(--light-text-muted);
    margin: 0;
  }
  
  .code-editor {
    height: 300px;
    border: 1px solid var(--light-border);
    border-radius: var(--radius-md);
  }
  
  .environment-variables {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }
  
  .env-variable-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
  
  .icon-button {
    background: none;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.25rem;
    border-radius: var(--radius-md);
  }
  
  .icon-button:hover {
    background-color: var(--light-bg-darker);
  }
  
  .material-icons {
    font-size: 1.2rem;
  }
  
  button.small {
    padding: 0.25rem 0.5rem;
    font-size: 0.875rem;
    display: flex;
    align-items: center;
    gap: 0.25rem;
    align-self: flex-start;
  }
  
  .actions {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .button-group {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
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
  
  .preview-section {
    margin-top: 2rem;
    background-color: white;
    border-radius: var(--radius-lg);
    padding: 2rem;
    box-shadow: var(--shadow-md);
  }
  
  .preview-container {
    border: 1px solid var(--light-border);
    border-radius: var(--radius-md);
    padding: 1rem;
    min-height: 400px;
    overflow: auto;
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
    
    .button-group {
      flex-direction: column;
    }
  }
</style>
