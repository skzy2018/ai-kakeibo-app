<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  
  let components: string | any[] = [];
  let isLoading = true;
  let errorMessage = "";
  
  onMount(async () => {
    await loadComponents();
  });
  
  async function loadComponents() {
    try {
      isLoading = true;
      errorMessage = "";
      
      const { invoke } = await import('@tauri-apps/api/core');
      const result = await invoke('get_sql_components');
      
      const parsedResult = typeof result === 'string' ? JSON.parse(result) : result;
      components = Array.isArray(parsedResult) ? parsedResult : [];
      
    } catch (error) {
      console.error("Failed to load SQL components:", error);
      errorMessage = `コンポーネントの読み込みに失敗しました: ${error}`;
    } finally {
      isLoading = false;
    }
  }
  
  function navigateToComponent(name: any) {
    goto(`/sqlcomponents/${name}`);
  }
</script>

<div class="page-container">
  <h1>SQLコンポーネント一覧</h1>
  
  <div class="content-container">
    {#if isLoading}
      <div class="loading">
        <p>コンポーネントを読み込み中...</p>
      </div>
    {:else if errorMessage}
      <div class="error-message">
        <p>{errorMessage}</p>
        <button class="secondary" on:click={loadComponents}>再試行</button>
      </div>
    {:else if components.length === 0}
      <div class="empty-state">
        <p>SQLコンポーネントがまだ作成されていません。</p>
        <p>「SQLコンポーネントの作成」メニューから新しいコンポーネントを作成してください。</p>
        <a href="/add-sqlcomponent" class="button primary">コンポーネントを作成</a>
      </div>
    {:else}
      <div class="component-grid">
        {#each components as component}
          <div class="component-card" on:click={() => navigateToComponent(component.name)}>
            <h3>{component.name}</h3>
            <p class="description">{component.description || "説明なし"}</p>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .page-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1rem;
  }
  
  .content-container {
    margin: 1.5rem 0;
    background-color: white;
    border-radius: var(--radius-lg);
    padding: 2rem;
    box-shadow: var(--shadow-md);
    min-height: 300px;
    display: flex;
    flex-direction: column;
  }
  
  .loading, .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 2rem;
    gap: 1rem;
    flex: 1;
  }
  
  .error-message {
    background-color: rgba(239, 68, 68, 0.1);
    border-left: 4px solid var(--danger);
    padding: 1rem;
    color: var(--danger);
    border-radius: var(--radius-md);
    margin: 1rem 0;
  }
  
  .component-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
  }
  
  .component-card {
    display: flex;
    flex-direction: column;
    border: 1px solid var(--light-border);
    border-radius: var(--radius-md);
    padding: 1.5rem;
    transition: all 0.2s ease;
    cursor: pointer;
    box-shadow: var(--shadow-sm);
  }
  
  .component-card:hover {
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
    border-color: var(--primary);
  }
  
  .component-card h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.2rem;
  }
  
  .description {
    color: var(--light-text-muted);
    font-size: 0.9rem;
    flex: 1;
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
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
  
  .button.primary {
    background-color: var(--primary);
    color: white;
    border: none;
  }
  
  .button.primary:hover {
    background-color: var(--primary-dark);
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
