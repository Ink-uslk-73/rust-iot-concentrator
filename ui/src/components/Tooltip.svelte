<script lang="ts">
  export let text: string;
  let visible = false;
  let timer: ReturnType<typeof setTimeout> | null = null;

  function show() {
    timer = setTimeout(() => (visible = true), 300);
  }
  function hide() {
    if (timer) clearTimeout(timer);
    visible = false;
  }
</script>

<span class="tt" on:mouseenter={show} on:mouseleave={hide}>
  <slot />
  {#if visible}
    <span class="popup">{text}</span>
  {/if}
</span>

<style>
  .tt { position: relative; display: inline-block; cursor: help; }
  .popup { position: absolute; bottom: 100%; left: 50%; transform: translateX(-50%);
    background: #0f172a; color: #e2e8f0; border: 1px solid #334155; padding: 0.5rem 0.75rem;
    border-radius: 6px; font-size: 0.8rem; white-space: nowrap; z-index: 100;
    box-shadow: 0 4px 12px rgba(0,0,0,0.4); }
</style>
