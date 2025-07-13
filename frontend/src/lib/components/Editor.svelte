<script>
  import { diagramStore } from '$lib/store.js';
</script>

<div class="h-full flex flex-col">
  <h2 class="text-lg font-semibold mb-2 p-2 bg-base-300 rounded-t-lg">Code</h2>
  <textarea
    class="textarea textarea-bordered w-full flex-grow font-mono text-lg resize-none"
    bind:value={$diagramStore}
    placeholder="Enter Mermaid.js code here..."
  ></textarea>
</div>
