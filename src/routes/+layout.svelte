<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import '../styles/global.css';
  import { apiClient } from "../lib/api-client";
  import { goto } from '$app/navigation';

  // The initial menu structure
  let menuItems: any = [
    { id: 'ai-chat', label: 'AI-CHAT', path: '/ai-chat', icon: '💬' },
    { id: 'dashboard', label: 'DASHBOARD', path: '/dashboard', icon: '📊' },
    { id: 'add-sqlcomponent', label: 'ADD_SQLCOMPONENT', path: '/add-sqlcomponent', icon: '➕' },
    { 
      id: 'sqlcomponents', 
      label: 'SQLCOMPONENTS', 
      icon: '📝',
      children: []  // This will be dynamically populated
    },
    { id: 'add-data-collector', label: 'ADD_DATA-COLLECTOR', path: '/add-data-collector', icon: '🔍' },
    { 
      id: 'data-collectors', 
      label: 'DATA-COLLECTOR', 
      icon: '📥',
      children: []  // This will be dynamically populated
    },
    { 
      id: 'database', 
      label: 'DATABASE', 
      icon: '🗄️',
      children: [
        { id: 'database-tables', label: 'M_TABLES', path: '/database/tables', icon: '📋' },
        { id: 'database-load', label: 'LOAD_DATA', path: '/database/load', icon: '⬆️' }
      ]
    },
    { id: 'settings', label: 'SETTINGS', path: '/settings', icon: '⚙️' }
  ];

  // SQLコンポーネントのデータ
  let sqlComponents: any[] = [];
  let dataCollectors = [
    { id: 'collector1', name: 'Bank CSV Import', path: '/data-collectors/bank-csv' },
    { id: 'collector2', name: 'Credit Card Data', path: '/data-collectors/credit-card' }
  ];

  // Load data and populate menu
  onMount(async () => {
    updateMenus();
  });

  async function updateMenus() {
    try {

      // Ensure API client is initialized
      if (!apiClient.isInitialized()) {
        await apiClient.initialize();
      }

      // SQLコンポーネントを読み込む
      //const { invoke } = await import('@tauri-apps/api/core');
      //const result = await invoke('get_sql_components');
      const result = await apiClient.getSqlComponents();
      //console.log("result",result)
      let parsedResult = result;

      if (Array.isArray(parsedResult)) {
        sqlComponents = parsedResult.map(comp => ({
          id: comp.name,
          name: comp.name,
          path: `/sqlcomponents/${comp.name}`
        }));
      }
    } catch (error) {
      console.error("Failed to load SQL components:", error);
      sqlComponents = [];
    }
    // メニューを更新
    menuItems = menuItems.map((item: { id: string; }) => {

      if (item.id === 'sqlcomponents') {
        return {
          ...item,
          children: sqlComponents.map(comp => ({
            id: comp.id,
            label: comp.name,
            path: comp.path,
            icon: '📊'
          }))
        };
      }
      
      if (item.id === 'data-collectors') {
        return {
          ...item,
          children: dataCollectors.map(collector => ({
            id: collector.id,
            label: collector.name,
            path: collector.path,
            icon: '📥'
          }))
        };
      }
      
      return item;
    });

  }

  // State for tracking expanded menu items
  let expandedMenus = new Set(['database']); // Default expanded items

  function toggleMenu(id: string) {
    updateMenus();
    if (expandedMenus.has(id)) {
      expandedMenus.delete(id);
    } else {
      expandedMenus.add(id);
    }
    expandedMenus = expandedMenus; // Trigger reactivity
    goto(`/${id}`);
  }

  // Get the current active path
  $: currentPath = $page.url.pathname;
</script>

<div class="app-container">
  <div class="left-pane">
    <div class="app-title">
      <h1>AI-KAKEIBO-APP</h1>
    </div>
    
    <nav class="main-nav">
      <ul class="menu-list">
        {#each menuItems as item}
          <li class="menu-item">
            {#if item.children}
              <!-- Menu with submenu -->
              <div 
                class="menu-header" 
                on:click={() => toggleMenu(item.id)}
                on:keydown={(e) => e.key === 'Enter' && toggleMenu(item.id)}
                class:active={currentPath && currentPath.startsWith(`/${item.id}`)}
                role="button"
                tabindex="0"
                aria-expanded={expandedMenus.has(item.id)}
              >
                <span class="menu-icon">{item.icon}</span>
                <span class="menu-label">{item.label}</span>
                <span class="expand-icon">{expandedMenus.has(item.id) ? '▼' : '▶'}</span>
              </div>
              
              {#if expandedMenus.has(item.id)}
                <ul class="submenu-list">
                  {#each item.children as child}
                    <li class="submenu-item">
                      <a 
                        href={child.path}
                        class:active={currentPath === child.path}
                      >
                        <span class="menu-icon">{child.icon}</span>
                        <span class="menu-label">{child.label}</span>
                      </a>
                    </li>
                  {/each}
                </ul>
              {/if}
            {:else}
              <!-- Regular menu item -->
              <a 
                href={item.path}
                class:active={currentPath === item.path}
              >
                <span class="menu-icon">{item.icon}</span>
                <span class="menu-label">{item.label}</span>
              </a>
            {/if}
          </li>
        {/each}
      </ul>
    </nav>
  </div>
  
  <div class="main-content">
    <slot />
  </div>
</div>

<style>
  .app-container {
    display: flex;
    height: 100vh;
    width: 100%;
    overflow: hidden;
  }
  
  .left-pane {
    width: 250px;
    background-color: #1e293b;
    color: #f8fafc;
    height: 100%;
    overflow-y: auto;
    padding: 0;
    box-shadow: 2px 0 5px rgba(0, 0, 0, 0.1);
    display: flex;
    flex-direction: column;
  }
  
  .app-title {
    padding: 1.5rem 1rem;
    border-bottom: 1px solid #334155;
  }
  
  .app-title h1 {
    font-size: 1.5rem;
    margin: 0;
    font-weight: 600;
    color: #f8fafc;
  }
  
  .main-nav {
    flex: 1;
    padding: 1rem 0;
  }
  
  .menu-list, .submenu-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  
  .menu-item {
    margin-bottom: 0.25rem;
  }
  
  .menu-item a, .menu-header {
    display: flex;
    align-items: center;
    padding: 0.75rem 1rem;
    color: #cbd5e1;
    text-decoration: none;
    transition: all 0.2s;
    cursor: pointer;
  }
  
  .menu-item a:hover, .menu-header:hover {
    background-color: #334155;
    color: #f8fafc;
  }
  
  .menu-item a.active {
    background-color: #2563eb;
    color: #ffffff;
  }
  
  .menu-icon {
    margin-right: 0.75rem;
    font-size: 1.1rem;
    width: 24px;
    text-align: center;
  }
  
  .menu-label {
    flex: 1;
  }
  
  .expand-icon {
    font-size: 0.7rem;
  }
  
  .submenu-list {
    margin-left: 1rem;
    border-left: 1px solid #334155;
  }
  
  .submenu-item a {
    padding: 0.5rem 1rem;
  }
  
  .main-content {
    flex: 1;
    padding: 1.5rem;
    overflow-y: auto;
    background-color: #f8fafc;
  }
  
  /* Responsive adjustments */
  @media (max-width: 768px) {
    .app-container {
      flex-direction: column;
    }
    
    .left-pane {
      width: 100%;
      height: auto;
      max-height: 300px;
    }
  }
</style>
