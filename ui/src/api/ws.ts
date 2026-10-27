export function connectEvents(onEvent: (data: any) => void): () => void {
  const es = new EventSource("/api/v1/events/stream");
  es.addEventListener("events", (e: MessageEvent) => {
    try {
      const data = JSON.parse(e.data);
      onEvent(data);
    } catch (err) {
      console.error("parse events:", err);
    }
  });
  return () => es.close();
}
