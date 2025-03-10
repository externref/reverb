<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import type { AppData } from '$lib/interfaces';
  import { goto } from '$app/navigation';
  onMount(async () => {
    const response: AppData | null = await invoke('get_configs');
    if (!response) {
      goto('/setup');
    }
  });
</script>
