<script lang="ts">
  import { onMount } from "svelte";
  
  // State variables
  let initialized = false;
  let loading = true;
  let error: Error | null = null;
  
  // Mock dashboard data
  let recentTransactions = [
    { id: 1, date: '2025-04-25', description: 'スーパーでの買い物', amount: -5800, category: '食費', account: '現金' },
    { id: 2, date: '2025-04-24', description: '電気代の支払い', amount: -12500, category: '住居費', account: '銀行口座' },
    { id: 3, date: '2025-04-23', description: '給料', amount: 280000, category: '収入', account: '銀行口座' },
    { id: 4, date: '2025-04-22', description: 'カフェでのランチ', amount: -1500, category: '食費', account: 'クレジットカード' },
    { id: 5, date: '2025-04-20', description: '映画チケット', amount: -3600, category: '娯楽', account: 'クレジットカード' }
  ];
  
  let monthlySummary = {
    income: 350000,
    expenses: 127500,
    balance: 222500
  };
  
  let categorySummary = [
    { name: '食費', amount: 42500, percentage: 33.3 },
    { name: '住居費', amount: 35000, percentage: 27.5 },
    { name: '交通費', amount: 15000, percentage: 11.8 },
    { name: '娯楽', amount: 20000, percentage: 15.7 },
    { name: 'その他', amount: 15000, percentage: 11.7 }
  ];
  
  onMount(async () => {
    try {
      // In a real app with Tauri, we would initialize the database
      // For development, we'll use mock data
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      loading = false;
      initialized = true;
    } catch (err) {
      console.error("Initialization error:", err);
      error = err instanceof Error ? err : new Error(String(err));
      loading = false;
    }
  });
</script>

<div class="page-container">
  <h1>ダッシュボード</h1>
  
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
    <div class="dashboard-grid">
      <!-- Monthly summary widget -->
      <div class="card">
        <h3>今月の概要</h3>
        <div class="summary-stats">
          <div class="stat-item">
            <span class="stat-label">収入</span>
            <span class="stat-value income">¥{monthlySummary.income.toLocaleString()}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">支出</span>
            <span class="stat-value expense">¥{monthlySummary.expenses.toLocaleString()}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">残高</span>
            <span class="stat-value balance">¥{monthlySummary.balance.toLocaleString()}</span>
          </div>
        </div>
      </div>
      
      <!-- Recent transactions widget -->
      <div class="card">
        <h3>最近の取引</h3>
        <div class="transaction-list">
          {#if recentTransactions.length === 0}
            <p class="empty-message">取引履歴がありません</p>
          {:else}
            <table>
              <thead>
                <tr>
                  <th>日付</th>
                  <th>説明</th>
                  <th>カテゴリ</th>
                  <th class="text-right">金額</th>
                </tr>
              </thead>
              <tbody>
                {#each recentTransactions as transaction}
                  <tr>
                    <td>{transaction.date}</td>
                    <td>{transaction.description}</td>
                    <td>
                      <span class="badge 
                        {transaction.category === '収入' ? 'badge-success' : 'badge-primary'}">
                        {transaction.category}
                      </span>
                    </td>
                    <td class="text-right {transaction.amount < 0 ? 'expense' : 'income'}">
                      ¥{Math.abs(transaction.amount).toLocaleString()}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {/if}
        </div>
      </div>
      
      <!-- Category summary widget -->
      <div class="card">
        <h3>カテゴリ別支出</h3>
        <div class="category-summary">
          {#each categorySummary as category}
            <div class="category-item">
              <div class="category-info">
                <span class="category-name">{category.name}</span>
                <span class="category-amount">¥{category.amount.toLocaleString()}</span>
              </div>
              <div class="category-bar">
                <div class="category-progress" style="width: {category.percentage}%"></div>
              </div>
              <span class="category-percentage">{category.percentage}%</span>
            </div>
          {/each}
        </div>
      </div>
      
      <!-- Placeholder for SQL components -->
      <div class="card">
        <div class="flex justify-between items-center mb-3">
          <h3>グラフウィジェット</h3>
          <span class="badge badge-primary">SQLコンポーネント</span>
        </div>
        <div class="mock-chart">
          <p class="text-center text-muted">ここにSQLコンポーネントの<br>グラフが表示されます</p>
          <div class="mock-bars">
            <div class="mock-bar" style="height: 40px;"></div>
            <div class="mock-bar" style="height: 65px;"></div>
            <div class="mock-bar" style="height: 30px;"></div>
            <div class="mock-bar" style="height: 80px;"></div>
            <div class="mock-bar" style="height: 50px;"></div>
            <div class="mock-bar" style="height: 70px;"></div>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .page-container {
    padding: 1rem;
  }
  
  .dashboard-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1.5rem;
    margin-top: 1.5rem;
  }
  
  @media (max-width: 768px) {
    .dashboard-grid {
      grid-template-columns: 1fr;
    }
  }
  
  .loading, .error-message {
    text-align: center;
    padding: 2rem;
    background-color: white;
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
  }
  
  .error-message {
    color: var(--danger);
  }
  
  .summary-stats {
    display: flex;
    justify-content: space-between;
    margin-top: 1rem;
  }
  
  .stat-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    flex: 1;
    padding: 0.5rem;
  }
  
  .stat-label {
    font-size: 0.875rem;
    color: var(--light-text-muted);
    margin-bottom: 0.25rem;
  }
  
  .stat-value {
    font-size: 1.5rem;
    font-weight: 600;
  }
  
  .income {
    color: var(--success);
  }
  
  .expense {
    color: var(--danger);
  }
  
  .balance {
    color: var(--primary);
  }
  
  .transaction-list {
    margin-top: 1rem;
    max-height: 300px;
    overflow-y: auto;
  }
  
  .empty-message {
    text-align: center;
    color: var(--light-text-muted);
    padding: 2rem 0;
  }
  
  .category-summary {
    margin-top: 1rem;
  }
  
  .category-item {
    margin-bottom: 1rem;
  }
  
  .category-info {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.25rem;
  }
  
  .category-name {
    font-weight: 500;
  }
  
  .category-amount {
    font-weight: 600;
  }
  
  .category-bar {
    height: 8px;
    background-color: var(--light-bg-darker);
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 0.25rem;
  }
  
  .category-progress {
    height: 100%;
    background-color: var(--primary);
    border-radius: 4px;
  }
  
  .category-percentage {
    font-size: 0.75rem;
    color: var(--light-text-muted);
    text-align: right;
    display: block;
  }
  
  .mock-chart {
    height: 200px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: 1rem;
    border: 1px dashed var(--light-border);
    border-radius: var(--radius-md);
    margin-top: 1rem;
  }
  
  .mock-bars {
    display: flex;
    align-items: flex-end;
    justify-content: space-around;
    width: 100%;
    height: 100px;
    margin-top: 1rem;
  }
  
  .mock-bar {
    width: 30px;
    background-color: var(--primary-light);
    border-radius: 4px 4px 0 0;
  }
</style>
