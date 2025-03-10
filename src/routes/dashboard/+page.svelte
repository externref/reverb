<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  onMount(async () => {
    await invoke('update_discord_presence', {
      state: 'exploring',
      details: 'On dashboard',
    });
  });
</script>

meow
