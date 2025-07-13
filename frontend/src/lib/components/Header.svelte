<script>
  import { diagramStore } from '$lib/store.js';
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  const themes = ["light", "dark", "cupcake", "bumblebee", "emerald", "corporate", "synthwave", "retro", "cyberpunk", "valentine", "halloween", "garden", "forest", "aqua", "lofi", "pastel", "fantasy", "wireframe", "black", "luxury", "dracula", "cmyk", "autumn", "business", "acid", "lemonade", "night", "coffee", "winter"];
  
  onMount(() => {
    document.documentElement.setAttribute('data-theme', 'dark');
  });

  function changeTheme(event) {
    document.documentElement.setAttribute('data-theme', event.target.value);
  }

  async function handleExport() {
    try {
      const message = await invoke('render_mermaid_to_file', { sourceText: $diagramStore });
      alert(message);
    } catch (error) {
      alert(`Error: ${error}`);
    }
  }
</script>

<header class="navbar bg-base-200 shadow-lg px-4">
  <div class="flex-1">
    <span class="text-xl font-bold">Mermaid Render</span>
  </div>
  <div class="flex-none gap-2">
    <select class="select select-bordered" on:change={changeTheme}>
      <option disabled selected>Theme</option>
      {#each themes as theme}
        <option value={theme}>{theme}</option>
      {/each}
    </select>
    <button class="btn btn-primary" on:click={handleExport}>Export to File</button>
  </div>
</header>
