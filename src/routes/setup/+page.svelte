<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  let username: string;
  async function createUser() {
    await invoke('create_user', { username });
    goto('/dashboard');
  }

  onMount(async () => {
    await invoke('update_discord_presence', {
      state: 'creating/syncing user',
      details: 'Setting up user',
    });
  });
</script>

<div class="flex justify-center items-center mt-34">
  <div class="text-center">
    <p class="text-4xl">
      welcome to <span class="text-nord-aurora-yellow">reverb</span> ,
    </p>
    <p class="my-4">
      No saved config file was found on this device, we'll create a new one.<br
      />
      Please enter a username to begin with ...
    </p>
    <p class="text-xl text-nord-aurora-yellow mb-3">Enter Username</p>

    <input
      type="text"
      bind:value={username}
      class="text-center text-sm bg-nord-polar-night-dark rounded text-white py-1 px-3 outline-none focus:ring-2 focus:ring-[#EBCB8B]"
      on:keydown={(ev) => {
        if (ev.key === 'Enter') {
          createUser();
        }
      }}
    />
    <p class="my-4">
      By proceeding, you agree to our <a
        class="text-nord-aurora-green"
        href="https://github.com/externref/reverb/wiki/Terms-of-Service"
        target="_blank">Terms of services</a
      >
    </p>
  </div>
</div>
