<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
</script>

<header>
  <div class="flex">
    <div class="text-2xl text-nord-aurora-orange">reverb_</div>
    <div class="grow"></div>
    <div class="text-nord-aurora-orange">
      {#if !page.url.href.endsWith('setup')}
        <button
          aria-label="back"
          on:click={() => {
            window.history.back();
          }}
        >
          <i
            class="bi bi-arrow-left-circle-fill text-2xl mr-2 text-nord-aurora-orange"
          ></i>
        </button>
        <button
          aria-label="settings"
          on:click={() => {
            goto('/settings');
          }}
          class="mr-2 transition duration-500 hover:rotate-180"
          ><i class="bi bi-gear text-2xl"></i></button
        >
        <button
          aria-label="back"
          on:click={() => {
            goto('/dashboard');
          }}
        >
          <i class="bi bi-house-door-fill text-2xl text-nord-aurora-orange"></i>
        </button>
      {/if}
    </div>
  </div>
</header>
