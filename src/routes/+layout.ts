// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import '../styles/global.css';
import { apiClient } from '../lib/api-client';

// Check if we're in the browser environment before initializing
const isBrowser = typeof window !== 'undefined';

// Initialize the API client only in browser environment
if (isBrowser) {
  const initialize = async () => {
    try {
      await apiClient.initialize();
      console.log('API client initialized successfully');
    } catch (error) {
      console.error('Failed to initialize API client:', error);
    }
  };

  // Call the initialize function
  initialize();
}

export const prerender = false; // Changed to false for dynamic API calls
export const ssr = false;
