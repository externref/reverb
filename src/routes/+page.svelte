<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import type { AppData } from '$lib/interfaces';
  import { goto } from '$app/navigation';
  onMount(async () => {
    await invoke('init_discord');
    await invoke('update_discord_presence', {
      state: 'checking for saved users...',
      details: 'Loading application',
    });
    const response: AppData | null = await invoke('get_user');
    if (!response) {
      goto('/setup');
    } else {
      goto('/dashboard');
    }
  });
</script>
