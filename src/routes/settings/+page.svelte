<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
</script>

<div class="mt-4">
  <p class="text-3xl">settings</p>
  <div class="h-[2px] my-2 bg-nord-aurora-yellow"></div>
  <div>
    <p class="text-base mb-1 text-nord-aurora-purple">user related</p>
    <div class="flex text-sm">
      <span
        ><span class="text-lg">Backup User</span><br />
        Creates a backup of the user data and saves it as a zip file.</span
      >
      <div class="flex grow"></div>
      <span
        ><button
          on:click={async () => await invoke('backup_user')}
          class="mt-4 px-3 py-1 hover:pl-5 rounded-sm bg-nord-aurora-red h-anim"
          ><i class="bi bi-box-arrow-down"></i>&nbsp;Backup
        </button></span
      >
    </div>
    <div class="flex text-sm mt-2">
      <span
        ><span class="text-lg">Load User</span><br />
        Loads the user data from a backup file created by reverb.</span
      >
      <div class="flex grow"></div>
      <span
        ><button
          on:click={async () => await invoke('restore_user')}
          class="mt-4 px-3 py-1 hover:pl-5 rounded-sm bg-nord-aurora-green text-nord-polar-night-dark h-anim"
          ><i class="bi bi-box-arrow-in-down"></i>&nbsp;Load
        </button></span
      >
    </div>
  </div>
</div>

<style>
  .h-anim {
    transition: all 0.3s ease;
  }
</style>
