<script lang="ts">
  import '../app.css';
  import Header from '../lib/components/Header.svelte';
</script>

<div class="m-3">
  <Header></Header>
  <slot></slot>
</div>
