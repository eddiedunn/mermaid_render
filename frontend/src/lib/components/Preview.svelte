<script>
  import { diagramStore } from '$lib/store.js';
  import mermaid from 'mermaid';
  import { onMount, tick } from 'svelte';

  let container;
  let error = '';

  onMount(() => {
    mermaid.initialize({
      startOnLoad: false,
      theme: 'default',
      securityLevel: 'loose',
    });
    render();
  });

  async function render() {
    if (!container || !$diagramStore) return;
    error = '';
    try {
      const { svg } = await mermaid.render('mermaid-preview', $diagramStore);
      container.innerHTML = svg;
    } catch (e) {
      error = e.message;
      container.innerHTML = '';
    }
  }

  $: if ($diagramStore, container) {
    render();
  }
</script>

<div class="h-full flex flex-col">
  <h2 class="text-lg font-semibold mb-2 p-2 bg-base-300 rounded-t-lg">Preview</h2>
  <div class="p-4 border border-base-300 rounded-b-lg flex-grow overflow-auto bg-white flex items-center justify-center">
    {#if error}
      <div class="alert alert-error">
        <span class="font-mono">{error}</span>
      </div>
    {:else}
      <div bind:this={container} class="w-full h-full"></div>
    {/if}
  </div>
</div>
