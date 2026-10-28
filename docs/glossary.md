# Глоссарий

**Satellite (сателлит)**  одна из трёх плат (A/B/C), независимый
процесс-адаптер. Пишет ``alive.<PLATE>`` в Registry, чтобы heartbeat
мог подтвердить её живость.

**PLATE-A / PLATE-B / PLATE-C**  HARD (10 мс) / SOFT (100 мс) / LIFE (1000 мс).

**alive.<PLATE>**  gauge-метрика: timestamp (ms) последнего опроса,
пишется адаптером из его цикла ``start()``. Используется ``health_check``.

**heartbeat(name)**  отметка в ``SatelliteRegistry``: сателлит жив и
только что подтвердился. Вызывается из ``spawn_heartbeat`` при условии
``health_check == true``.

**health_check(&Registry, timeout_ms)**  проверка ``alive.<PLATE>`` на
свежесть. ``now - alive_ts < timeout_ms``  живой. Иначе  stale.

**Triple redundancy (тройное резервирование)**  3 сателлитные платы
(A/B/C) + AIG-502 (аппаратный резерв) + CORE (Redis hot-path + Raw CBOR +
TimescaleDB). Отказ любого слоя не роняет концентратор.

**AIG-502**  резервный Modbus TCP шлюз. Держит snapshot последней
телеметрии, включается при недоступности CORE.

**CORE: три уровня данных**:
- Redis  hot-path, TTL 60 с, O(1);
- Raw CBOR  бинарный лог без потерь (10 МБ сегменты, ротация 24 ч);
- TimescaleDB  агрегаты 1s/1m/1h, долговременное хранение.

**spawn_restartable(name, factory)**  задача с автоперезапуском:
``catch_unwind`` внутри spawn, при панике  backoff 1 с и повторный вызов
``factory()``. Для доменов с уникальными ресурсами (mpsc::Receiver) 
не применять, только внутренний reconnect-loop.

**PublishFn**  ``Arc<dyn Fn(Priority, f64, &str) + Send + Sync>``,
адаптер пушит телеметрию в Bus.

**Язык проекта**  русский (документация, комментарии, commit messages).
Код (имена типов/функций)  английский.
