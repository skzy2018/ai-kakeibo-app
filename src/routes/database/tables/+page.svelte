<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  
  // State variables
  let loading = true;
  let error = null;
  let activeTable = "accounts"; // Default active table
  let tables = [
    { id: "accounts", name: "口座", icon: "💰" },
    { id: "categories", name: "カテゴリ", icon: "🏷️" },
    { id: "tags", name: "タグ", icon: "🔖" }
  ];
  
  // Mock data for each table
  let accountData = [
    { id: 1, name: "現金", type: "現金", balance: 25000 },
    { id: 2, name: "銀行口座", type: "銀行", balance: 450000 },
    { id: 3, name: "クレジットカード", type: "カード", balance: -75000 },
    { id: 4, name: "電子マネー", type: "その他", balance: 15000 }
  ];
  
  let categoryData = [
    { id: 1, name: "食費", type: "支出", parent_id: null },
    { id: 2, name: "外食", type: "支出", parent_id: 1 },
    { id: 3, name: "食料品", type: "支出", parent_id: 1 },
    { id: 4, name: "住居費", type: "支出", parent_id: null },
    { id: 5, name: "給料", type: "収入", parent_id: null },
    { id: 6, name: "ボーナス", type: "収入", parent_id: null },
    { id: 7, name: "交通費", type: "支出", parent_id: null },
    { id: 8, name: "娯楽", type: "支出", parent_id: null }
  ];
  
  let tagData = [
    { id: 1, name: "固定費" },
    { id: 2, name: "変動費" },
    { id: 3, name: "必須" },
    { id: 4, name: "娯楽" },
    { id: 5, name: "旅行" },
    { id: 6, name: "贈り物" }
  ];
  
  // New item form data
  let newAccount = { name: "", type: "銀行", balance: 0 };
  let newCategory = { name: "", type: "支出", parent_id: null };
  let newTag = { name: "" };
  
  let showAddForm = false;
  let formError = "";
  
  onMount(async () => {
    try {
      // In a real app, we would load the actual data from the database
      // For now, we'll just simulate loading
      await new Promise(resolve => setTimeout(resolve, 500));
      
      // Simulated database calls
      /* 
      const accountResult = await invoke("get_accounts");
      accountData = JSON.parse(accountResult);
      
      const categoryResult = await invoke("get_categories");
      categoryData = JSON.parse(categoryResult);
      
      const tagResult = await invoke("get_tags");
      tagData = JSON.parse(tagResult);
      */
      
      loading = false;
    } catch (err) {
      console.error("Failed to load data:", err);
      error = err;
      loading = false;
    }
  });
  
  function setActiveTable(tableId) {
    activeTable = tableId;
    showAddForm = false;
    formError = "";
    
    // Reset form data
    newAccount = { name: "", type: "銀行", balance: 0 };
    newCategory = { name: "", type: "支出", parent_id: null };
    newTag = { name: "" };
  }
  
  async function addItem() {
    formError = "";
    
    if (activeTable === "accounts" && !newAccount.name) {
      formError = "口座名を入力してください";
      return;
    } else if (activeTable === "categories" && !newCategory.name) {
      formError = "カテゴリ名を入力してください";
      return;
    } else if (activeTable === "tags" && !newTag.name) {
      formError = "タグ名を入力してください";
      return;
    }
    
    try {
      // In a real app, we would save the new item to the database
      // For now, we'll just add it to our mock data
      
      if (activeTable === "accounts") {
        const newId = Math.max(...accountData.map(item => item.id)) + 1;
        accountData = [...accountData, { id: newId, ...newAccount }];
        newAccount = { name: "", type: "銀行", balance: 0 };
      } else if (activeTable === "categories") {
        const newId = Math.max(...categoryData.map(item => item.id)) + 1;
        categoryData = [...categoryData, { id: newId, ...newCategory }];
        newCategory = { name: "", type: "支出", parent_id: null };
      } else if (activeTable === "tags") {
        const newId = Math.max(...tagData.map(item => item.id)) + 1;
        tagData = [...tagData, { id: newId, ...newTag }];
        newTag = { name: "" };
      }
      
      showAddForm = false;
    } catch (err) {
      console.error("Failed to add item:", err);
      formError = `アイテムの追加に失敗しました: ${err}`;
    }
  }
  
  function toggleAddForm() {
    showAddForm = !showAddForm;
    formError = "";
  }
  
  async function deleteItem(id) {
    if (!confirm("本当に削除しますか？この操作は元に戻せません。")) {
      return;
    }
    
    try {
      // In a real app, we would delete the item from the database
      // For now, we'll just remove it from our mock data
      
      if (activeTable === "accounts") {
        accountData = accountData.filter(item => item.id !== id);
      } else if (activeTable === "categories") {
        categoryData = categoryData.filter(item => item.id !== id);
      } else if (activeTable === "tags") {
        tagData = tagData.filter(item => item.id !== id);
      }
    } catch (err) {
      console.error("Failed to delete item:", err);
      alert(`削除に失敗しました: ${err}`);
    }
  }
</script>

<div class="page-container">
  <h1>マスターテーブル編集</h1>
  <p>家計簿アプリの基本データを編集します。これらのデータは取引記録の分類や管理に使用されます。</p>
  
  {#if loading}
    <div class="loading">
      <p>データを読み込み中...</p>
    </div>
  {:else if error}
    <div class="error-message">
      <p>エラーが発生しました: {error}</p>
      <button on:click={() => window.location.reload()}>再試行</button>
    </div>
  {:else}
    <div class="database-container">
      <div class="table-navigation">
        <ul>
          {#each tables as table}
            <li class:active={activeTable === table.id}>
              <button on:click={() => setActiveTable(table.id)}>
                <span class="table-icon">{table.icon}</span>
                <span class="table-name">{table.name}</span>
              </button>
            </li>
          {/each}
        </ul>
      </div>
      
      <div class="table-content">
        <div class="table-header">
          <h2>
            {tables.find(t => t.id === activeTable)?.icon} 
            {tables.find(t => t.id === activeTable)?.name}テーブル
          </h2>
          <button class="add-button" on:click={toggleAddForm}>
            {showAddForm ? '追加をキャンセル' : '新規追加'}
          </button>
        </div>
        
        {#if showAddForm}
          <div class="add-form">
            {#if formError}
              <div class="form-error">{formError}</div>
            {/if}
            
            {#if activeTable === "accounts"}
              <div class="form-grid">
                <div class="form-group">
                  <label for="accountName">口座名</label>
                  <input 
                    type="text" 
                    id="accountName" 
                    bind:value={newAccount.name} 
                    placeholder="例: 三菱UFJ銀行" 
                    required
                  />
                </div>
                
                <div class="form-group">
                  <label for="accountType">種類</label>
                  <select id="accountType" bind:value={newAccount.type}>
                    <option value="現金">現金</option>
                    <option value="銀行">銀行</option>
                    <option value="カード">クレジットカード</option>
                    <option value="電子マネー">電子マネー</option>
                    <option value="その他">その他</option>
                  </select>
                </div>
                
                <div class="form-group">
                  <label for="accountBalance">残高</label>
                  <input 
                    type="number" 
                    id="accountBalance" 
                    bind:value={newAccount.balance} 
                    placeholder="0"
                  />
                </div>
              </div>
            {:else if activeTable === "categories"}
              <div class="form-grid">
                <div class="form-group">
                  <label for="categoryName">カテゴリ名</label>
                  <input 
                    type="text" 
                    id="categoryName" 
                    bind:value={newCategory.name} 
                    placeholder="例: 光熱費" 
                    required
                  />
                </div>
                
                <div class="form-group">
                  <label for="categoryType">種類</label>
                  <select id="categoryType" bind:value={newCategory.type}>
                    <option value="収入">収入</option>
                    <option value="支出">支出</option>
                  </select>
                </div>
                
                <div class="form-group">
                  <label for="categoryParent">親カテゴリ (オプション)</label>
                  <select 
                    id="categoryParent" 
                    bind:value={newCategory.parent_id}
                  >
                    <option value={null}>なし (親カテゴリ)</option>
                    {#each categoryData.filter(c => c.parent_id === null) as category}
                      <option value={category.id}>{category.name}</option>
                    {/each}
                  </select>
                </div>
              </div>
            {:else if activeTable === "tags"}
              <div class="form-group">
                <label for="tagName">タグ名</label>
                <input 
                  type="text" 
                  id="tagName" 
                  bind:value={newTag.name} 
                  placeholder="例: 医療費" 
                  required
                />
              </div>
            {/if}
            
            <div class="form-actions">
              <button class="cancel" on:click={toggleAddForm}>キャンセル</button>
              <button class="submit" on:click={addItem}>追加</button>
            </div>
          </div>
        {/if}
        
        <div class="data-table">
          {#if activeTable === "accounts"}
            <table>
              <thead>
                <tr>
                  <th>ID</th>
                  <th>口座名</th>
                  <th>種類</th>
                  <th class="text-right">残高</th>
                  <th class="actions-column">操作</th>
                </tr>
              </thead>
              <tbody>
                {#each accountData as account}
                  <tr>
                    <td>{account.id}</td>
                    <td>{account.name}</td>
                    <td>{account.type}</td>
                    <td class="text-right">
                      <span class={account.balance < 0 ? 'negative' : ''}>
                        ¥{account.balance.toLocaleString()}
                      </span>
                    </td>
                    <td class="actions">
                      <button class="icon-button edit">✏️</button>
                      <button 
                        class="icon-button delete" 
                        on:click={() => deleteItem(account.id)}
                      >
                        🗑️
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {:else if activeTable === "categories"}
            <table>
              <thead>
                <tr>
                  <th>ID</th>
                  <th>カテゴリ名</th>
                  <th>種類</th>
                  <th>親カテゴリ</th>
                  <th class="actions-column">操作</th>
                </tr>
              </thead>
              <tbody>
                {#each categoryData as category}
                  <tr>
                    <td>{category.id}</td>
                    <td>
                      {#if category.parent_id}
                        <span class="indent">└ {category.name}</span>
                      {:else}
                        <strong>{category.name}</strong>
                      {/if}
                    </td>
                    <td>
                      <span class="badge {category.type === '収入' ? 'badge-success' : 'badge-primary'}">
                        {category.type}
                      </span>
                    </td>
                    <td>
                      {#if category.parent_id}
                        {categoryData.find(c => c.id === category.parent_id)?.name || '-'}
                      {:else}
                        -
                      {/if}
                    </td>
                    <td class="actions">
                      <button class="icon-button edit">✏️</button>
                      <button 
                        class="icon-button delete" 
                        on:click={() => deleteItem(category.id)}
                      >
                        🗑️
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {:else if activeTable === "tags"}
            <table>
              <thead>
                <tr>
                  <th>ID</th>
                  <th>タグ名</th>
                  <th class="actions-column">操作</th>
                </tr>
              </thead>
              <tbody>
                {#each tagData as tag}
                  <tr>
                    <td>{tag.id}</td>
                    <td>{tag.name}</td>
                    <td class="actions">
                      <button class="icon-button edit">✏️</button>
                      <button 
                        class="icon-button delete" 
                        on:click={() => deleteItem(tag.id)}
                      >
                        🗑️
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .page-container {
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .loading, .error-message {
    text-align: center;
    padding: 2rem;
    background-color: white;
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    margin-top: 2rem;
  }
  
  .error-message {
    color: var(--danger);
  }
  
  .database-container {
    display: flex;
    margin-top: 1.5rem;
    background-color: white;
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    overflow: hidden;
  }
  
  .table-navigation {
    width: 200px;
    background-color: var(--light-bg-darker);
    border-right: 1px solid var(--light-border);
  }
  
  .table-navigation ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  
  .table-navigation li {
    margin: 0;
  }
  
  .table-navigation button {
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
  
  .table-navigation button:hover {
    background-color: rgba(0, 0, 0, 0.05);
  }
  
  .table-navigation li.active button {
    background-color: white;
    color: var(--primary);
    font-weight: 600;
  }
  
  .table-icon {
    margin-right: 0.75rem;
    font-size: 1.25rem;
    width: 1.5rem;
    text-align: center;
  }
  
  .table-content {
    flex: 1;
    padding: 1.5rem;
    overflow-x: auto;
  }
  
  .table-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }
  
  .table-header h2 {
    margin: 0;
    font-size: 1.25rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  
  .add-button {
    padding: 0.5rem 1rem;
  }
  
  .add-form {
    background-color: var(--light-bg);
    padding: 1.5rem;
    border-radius: var(--radius-md);
    margin-bottom: 1.5rem;
    border: 1px solid var(--light-border);
  }
  
  .form-error {
    color: var(--danger);
    margin-bottom: 1rem;
    padding: 0.5rem;
    background-color: rgba(239, 68, 68, 0.1);
    border-radius: var(--radius-sm);
    font-size: 0.9rem;
  }
  
  .form-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }
  
  .form-group {
    margin-bottom: 1rem;
  }
  
  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    margin-top: 1rem;
  }
  
  button.cancel {
    background-color: transparent;
    color: var(--light-text);
    border: 1px solid var(--light-border);
  }
  
  button.submit {
    background-color: var(--primary);
  }
  
  .data-table {
    overflow-x: auto;
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
  
  .text-right {
    text-align: right;
  }
  
  .negative {
    color: var(--danger);
  }
  
  .actions-column {
    width: 100px;
  }
  
  .actions {
    display: flex;
    gap: 0.5rem;
  }
  
  .icon-button {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 1rem;
    padding: 0.25rem;
    border-radius: var(--radius-sm);
    transition: background-color 0.2s;
  }
  
  .icon-button:hover {
    background-color: var(--light-bg);
  }
  
  .icon-button.delete:hover {
    background-color: rgba(239, 68, 68, 0.1);
  }
  
  .indent {
    padding-left: 1rem;
  }
  
  /* Responsive adjustments */
  @media (max-width: 768px) {
    .database-container {
      flex-direction: column;
    }
    
    .table-navigation {
      width: 100%;
      border-right: none;
      border-bottom: 1px solid var(--light-border);
    }
    
    .table-navigation ul {
      display: flex;
      overflow-x: auto;
    }
    
    .table-navigation button {
      padding: 0.75rem;
      border-bottom: none;
      border-right: 1px solid var(--light-border);
      flex-direction: column;
      text-align: center;
    }
    
    .table-icon {
      margin-right: 0;
      margin-bottom: 0.25rem;
    }
    
    .form-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
