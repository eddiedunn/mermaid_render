<script>
  import Header from '$lib/components/Header.svelte';
  import Editor from '$lib/components/Editor.svelte';
  import Preview from '$lib/components/Preview.svelte';
</script>

<div class="flex flex-col h-screen">
  <Header />
  <main class="flex-grow grid grid-cols-1 md:grid-cols-2 gap-4 p-4 overflow-auto">
    <Editor />
    <Preview />
  </main>
</div>
