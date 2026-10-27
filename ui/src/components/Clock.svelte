<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  const DAYS = ["Вс", "Пн", "Вт", "Ср", "Чт", "Пт", "Сб"];
  let now = new Date();
  let timer: ReturnType<typeof setInterval>;

  function pad(n: number): string {
    return n < 10 ? "0" + n : String(n);
  }

  function tick() {
    now = new Date();
  }

  onMount(() => {
    timer = setInterval(tick, 1000);
  });
  onDestroy(() => clearInterval(timer));

  $: time = `${pad(now.getHours())}:${pad(now.getMinutes())}:${pad(now.getSeconds())}`;
  $: date = `${pad(now.getDate())}.${pad(now.getMonth() + 1)}.${now.getFullYear()}`;
  $: day = DAYS[now.getDay()];
</script>

<div class="clock">
  <div class="time">{time}</div>
  <div class="date">{date} <span class="day">{day}</span></div>
</div>

<style>
  .clock { text-align: right; font-family: monospace; line-height: 1.1; }
  .time { font-size: 0.5rem; font-weight: 600; letter-spacing: 0.5px; color: #cbd5e1; }
  .date { font-size: 0.37rem; color: #64748b; margin-top: 0.05rem; }
  .day { color: #93c5fd; font-weight: 600; }
</style>
