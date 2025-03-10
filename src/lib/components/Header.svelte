<script lang="ts">
  import { goto } from '$app/navigation';
</script>

<header>
  <div class="flex">
    <div class="text-2xl text-nord-aurora-orange">reverb_</div>
    <div class="grow"></div>
    <div class="text-nord-aurora-orange">
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
        class="transition duration-500 hover:rotate-180"
        ><i class="bi bi-gear text-2xl"></i></button
      >
    </div>
  </div>
</header>
