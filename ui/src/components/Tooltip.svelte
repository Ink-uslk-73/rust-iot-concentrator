<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { computePosition, autoUpdate, offset, flip, shift } from "@floating-ui/dom";

  export let text: string;

  let triggerEl: HTMLElement;
  let floatingEl: HTMLElement | null = null;
  let visible = false;
  let timer: ReturnType<typeof setTimeout> | null = null;
  let cleanupAutoUpdate: (() => void) | null = null;

  onMount(() => {
    const onDocPointerDown = (e: PointerEvent) => {
      if (!visible) return;
      const target = e.target as Node;
      if (triggerEl?.contains(target)) return;
      if (floatingEl?.contains(target)) return;
      hide();
    };
    document.addEventListener("pointerdown", onDocPointerDown);
    return () => document.removeEventListener("pointerdown", onDocPointerDown);
  });

  onDestroy(() => {
    if (timer) clearTimeout(timer);
    if (cleanupAutoUpdate) cleanupAutoUpdate();
  });

  function show() { if (visible) return; visible = true; }
  function hide() { if (!visible) return; visible = false; }

  // Мышь/перо: hover с задержкой 300 мс. Тач: игнорируем.
  function onPointerEnter(e: PointerEvent) {
    if (e.pointerType === "touch") return;
    if (timer) clearTimeout(timer);
    timer = setTimeout(show, 300);
  }
  function onPointerLeave(e: PointerEvent) {
    if (e.pointerType === "touch") return;
    if (timer) clearTimeout(timer);
    hide();
  }

  // Тач: тап открывает/закрывает.
  function onPointerDown(e: PointerEvent) {
    if (e.pointerType !== "touch") return;
    e.stopPropagation();
    if (visible) hide(); else show();
  }

  function onFocus() { show(); }
  function onBlur() { hide(); }

  function float(node: HTMLElement) {
    floatingEl = node;
    const update = () => {
      computePosition(triggerEl, node, {
        placement: "bottom",
        strategy: "fixed",
        middleware: [offset(6), flip(), shift({ padding: 8 })],
      }).then(({ x, y }) => {
        Object.assign(node.style, {
          left: `${x}px`,
          top: `${y}px`,
          opacity: "1",
        });
      });
    };
    cleanupAutoUpdate = autoUpdate(triggerEl, node, update);
    return {
      destroy() {
        if (cleanupAutoUpdate) { cleanupAutoUpdate(); cleanupAutoUpdate = null; }
        floatingEl = null;
      },
    };
  }
</script>

<span
  bind:this={triggerEl}
  class="tt"
  role="tooltip"
  aria-label={text}
  tabindex="0"
  on:pointerenter={onPointerEnter}
  on:pointerleave={onPointerLeave}
  on:pointerdown={onPointerDown}
  on:focus={onFocus}
  on:blur={onBlur}
>
  <slot />
</span>

{#if visible}
  <span class="popup" use:float aria-hidden="true">{text}</span>
{/if}

<style>
  .tt { position: relative; display: inline-block; cursor: help; outline: none; }
  .popup {
    position: fixed;
    top: 0; left: 0;
    opacity: 0;
    width: max-content;
    max-width: 300px;
    background: #1e293b;
    color: #e2e8f0;
    border: 1px solid #475569;
    padding: 0.5rem 0.75rem;
    border-radius: 6px;
    font-size: 0.8rem;
    z-index: 1000;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.6);
  }
</style>
