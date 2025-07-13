import { readText } from '@tauri-apps/api/clipboard';
import { diagramStore } from './store.js';

export async function initializeClipboard() {
  try {
    const text = await readText();
    if (text) {
      diagramStore.set(text);
    }
  } catch (error) {
    console.error('Failed to read clipboard:', error);
  }
}
