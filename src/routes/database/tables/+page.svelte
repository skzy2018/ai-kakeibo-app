<script lang="ts">
  import { onMount } from "svelte";
  //import { invoke } from "@tauri-apps/api/core";
  //import { confirm } from "@tauri-apps/plugin-dialog";
  import { apiClient } from "../../../lib/api-client";
  
  // Define types for database entries
  interface Account {
    //account_id: number;
    id: number; // alias for account_id for UI display
    name: string;
    account_type: string;
    //type: string; // alias for account_type for UI display
    currency: string;
    created_at?: string;
    // Additional frontend-only properties
    //balance?: number;
  }
  
  interface Category {
    //category_id: number;
    id: number; // alias for category_id for UI display
    name: string;
    type: string;
    created_at?: string;
    // Additional frontend-only properties
    //parent_id?: number | null;
  }
  
  interface Tag {
    //tag_id: number;
    id: number; // alias for tag_id for UI display
    name: string;
    created_at?: string;
  }
  
  type TableId = "accounts" | "categories" | "tags";
  
  // State variables
  let activeTable: "accounts" | "categories" | "tags" = "accounts"; // Default active table
  const tables = [
    { id: "accounts", name: "口座", icon: "💰" },
    { id: "categories", name: "カテゴリ", icon: "🏷️" },
    { id: "tags", name: "タグ", icon: "🔖" }
  ];
  
  // Database data
  let accountData: Account[] = [];
  let categoryData: Category[] = [];
  let tagData: Tag[] = [];
  
  // Selected items for deletion
  let selectedItems: {
    accounts: Set<number>;
    categories: Set<number>;
    tags: Set<number>;
  } = {
    accounts: new Set<number>(),
    categories: new Set<number>(),
    tags: new Set<number>()
  };
  
  // New item form data
  //let newAccount = { name: "", account_type: "銀行", currency: "JPY", type: "銀行", balance: 0 };
  let newAccount = { name: "", account_type: "銀行", currency: "JPY" };
  //let newCategory = { name: "", type: "支出", parent_id: null as number | null };
  let newCategory = { name: "", type: "支出" };
  let newTag = { name: "" };
  
  let showAddForm = false;
  let loading = true;
  let error: any = null;
  let errors: string[] = [];
  let formError = "";
  let successMessage = "";
  let successTimer: number | null = null;
  let errorTimer: number | null = null;
  
  // Load all data from database
  async function loadAllData() {
    loading = true;
    error = null;
    
    try {
      // Ensure API client is initialized
      if (!apiClient.isInitialized()) {
        await apiClient.initialize();
      }
      
      // Load accounts using API client
      const accounts = await apiClient.getAccounts();
      accountData = accounts.map((account: any) => ({
        ...account,
        id: account.account_id // Set the id alias for UI
      }));
      
      // Load categories using API client
      const categories = await apiClient.getCategories();
      categoryData = categories.map((category: any) => ({
        ...category,
        id: category.category_id // Set the id alias for UI
      }));
      
      // Load tags using API client
      const tags = await apiClient.getTags();
      tagData = tags.map((tag: any) => ({
        ...tag,
        id: tag.tag_id // Set the id alias for UI
      }));
      
      loading = false;
    } catch (err) {
      console.error("Failed to load data:", err);
      error = err;
      loading = false;
    }
  }
  
  onMount(async () => {
    await loadAllData();
  });
  
  function setActiveTable(tableId: "accounts" | "categories" | "tags") {
    activeTable = tableId;
    showAddForm = false;
    formError = "";
    successMessage = "";
    
    // Reset form data
    //newAccount = { name: "", account_type: "銀行", currency: "JPY", type: "銀行", balance: 0 };
    newAccount = { name: "", account_type: "銀行", currency: "JPY" };
    newCategory = { name: "", type: "支出",};
    newTag = { name: "" };
  }
  
  async function addItem() {
    formError = "";
    successMessage = "";
    
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
      // Ensure API client is initialized
      if (!apiClient.isInitialized()) {
        await apiClient.initialize();
      }
      
      let result;
      
      if (activeTable === "accounts") {
        result = await apiClient.addAccount(
          newAccount.name,
          newAccount.account_type,
          newAccount.currency
        );
        
        // Clear form
        newAccount = { name: "", account_type: "銀行", currency: "JPY"};
      } else if (activeTable === "categories") {
        result = await apiClient.addCategory(
          newCategory.name,
          newCategory.type
        );
        
        // Clear form
        newCategory = { name: "", type: "支出" };
      } else if (activeTable === "tags") {
        result = await apiClient.addTag(
          newTag.name
        );
        
        // Clear form
        newTag = { name: "" };
      }
      
      if (result && result.success) {
        // Show success message
        successMessage = `${activeTable === "accounts" ? "口座" : 
                          activeTable === "categories" ? "カテゴリ" : "タグ"}
                          を追加しました`;
        
        // Clear message after a delay
        if (successTimer) clearTimeout(successTimer);
        successTimer = setTimeout(() => {
          successMessage = "";
        }, 3000);
        
        // Reload data
        await loadAllData();
        
        // Close form
        showAddForm = false;
      } else {
        formError = (result as any).error || "追加に失敗しました";
      }
    } catch (err) {
      console.error("Failed to add item:", err);
      formError = `アイテムの追加に失敗しました: ${err}`;
    }
  }
  
  function toggleAddForm() {
    showAddForm = !showAddForm;
    errors = []
    formError = "";
    successMessage = "";
  }
  
  //function toggleItemSelection(tableId: TableId, itemId: number) {
  //  if (selectedItems[tableId].has(itemId)) {
  //    selectedItems[tableId].delete(itemId);
  //  } else {
  //    selectedItems[tableId].add(itemId);
  //  }
  //  selectedItems = { ...selectedItems }; // Trigger reactivity
  //}

  // Single item delete function
  function deleteItem(id: number) {
    // Add the item to the selection and then delete it
    selectedItems[activeTable].add(id);
    //console.log("selectedItems",selectedItems[activeTable])
    deleteSelectedItems();
  }
  
  async function deleteSelectedItems() {
    const tableType = activeTable === "accounts" ? "口座" : 
                      activeTable === "categories" ? "カテゴリ" : "タグ";
    
    const count = selectedItems[activeTable].size;
    if (count === 0) return;
    
    let deleted = 0;
    errors = [];
    
    try {
      // Ensure API client is initialized
      if (!apiClient.isInitialized()) {
        await apiClient.initialize();
      }
      
      for (const id of selectedItems[activeTable]) {
        let result;
        
        if (activeTable === "accounts") {
          result = await apiClient.deleteAccount(id);
        } else if (activeTable === "categories") {
          result = await apiClient.deleteCategory(id);
        } else if (activeTable === "tags") {
          result = await apiClient.deleteTag(id);
        }
        
        if (result && result.success) {
          deleted++;
        } else {
          errors.push(`ID ${id}: ${result.error || '不明なエラー'}`);
        }
      }
      
      // Clear selection
      selectedItems[activeTable] = new Set();
      selectedItems = { ...selectedItems };
      
      // Show result
      if (deleted > 0) {
        successMessage = `${deleted}個の${tableType}を削除しました`;
        
        // Clear message after a delay
        if (successTimer) clearTimeout(successTimer);
        successTimer = setTimeout(() => {
          successMessage = "";
        }, 3000);
        
        // Reload data
        await loadAllData();
      }
      
      if (errors.length > 0) {
        formError = `${errors.length}個の${tableType}を削除できませんでした: ${errors.join(', ')}`;
        if (errorTimer) clearTimeout(errorTimer);
        errorTimer = setTimeout(() => {
          errors = [];
          formError = "";
        }, 3000);
      }
    } catch (err) {
      console.error("Failed to delete items:", err);
      formError = `削除に失敗しました: ${err}`;
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
    {#if errors.length > 0 || formError }
      <div class="error-message">
        {#if errors.length > 0}<p>エラーが発生しました: {errors.join(', ')}</p>{/if}
        {#if formError}<p>{formError}</p>{/if}
      </div>
    {/if}
    <div class="database-container">
      <div class="table-navigation">
        <ul>
    {#each tables as table}
      <li class:active={activeTable === table.id as TableId}>
        <button on:click={() => setActiveTable(table.id as TableId)}>
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
        
        {#if successMessage}
          <div class="success-message">
            <p>{successMessage}</p>
          </div>
        {/if}
        
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
                  <select id="accountType" bind:value={newAccount.account_type}>
                    <option value="現金">現金</option>
                    <option value="銀行">銀行</option>
                    <option value="カード">クレジットカード</option>
                    <option value="電子マネー">電子マネー</option>
                    <option value="その他">その他</option>
                  </select>
                </div>
                
                <!-- <div class="form-group">
                  <label for="accountBalance">残高</label>
                  <input 
                    type="number" 
                    id="accountBalance" 
                    bind:value={newAccount.balance} 
                    placeholder="0"
                  />
                </div> -->
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
                
                <!-- <div class="form-group">
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
                </div>  -->
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
                  <!-- <th class="text-right">残高</th> -->
                  <th class="actions-column">操作</th>
                </tr>
              </thead>
              <tbody>
                {#each accountData as account}
                  <tr>
                    <td>{account.id}</td>
                    <td>{account.name}</td>
                    <!-- <td>{account.type}</td> -->
                    <td>{account.account_type}</td>
                    <!-- <td class="text-right">
                      <span class={account.balance !== undefined && account.balance < 0 ? 'negative' : ''}>
                        ¥{account.balance !== undefined ? account.balance.toLocaleString() : '0'}
                      </span>
                    </td> -->
                    <td class="actions">
                      <!-- <button class="icon-button edit">✏️</button>a  -->
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
                  <!-- <th>親カテゴリ</th> -->
                  <th class="actions-column">操作</th>
                </tr>
              </thead>
              <tbody>
                {#each categoryData as category}
                  <tr>
                    <td>{category.id}</td>
                    <td>{category.name}</td>
                    <!-- <td>
                      {#if category.parent_id}
                        <span class="indent">└ {category.name}</span>
                      {:else}
                        <strong>{category.name}</strong>
                      {/if}
                    </td> -->
                    <td>
                      <span class="badge {category.type === '収入' ? 'badge-success' : 'badge-primary'}">
                        {category.type}
                      </span>
                    </td>
                    <!-- <td>
                      {#if category.parent_id}
                        {categoryData.find(c => c.id === category.parent_id)?.name || '-'}
                      {:else}
                        -
                      {/if}
                    </td> -->
                    <td class="actions">
                      <!-- <button class="icon-button edit">✏️</button> -->
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
                      <!-- <button class="icon-button edit">✏️</button> -->
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
  
  .success-message {
    text-align: center;
    padding: 1rem;
    background-color: rgba(16, 185, 129, 0.1);
    color: rgb(16, 185, 129);
    border-radius: var(--radius-md);
    margin-bottom: 1.5rem;
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
