# ADR-0014: PCIe NTB backplane (fallback)

**Статус:** accepted
**Дата:** 2026-11-01
**Связанные:** [ADR-0006](ADR-0006-satellite-sovereignty.md), [pcie-topology.md](../system/pcie-topology.md), [ADR-0017](ADR-0017-alveo-p2p.md).

## Контекст

Cross-socket P2P на 2P AMD EPYC работает через xGMI (см. ADR-0017, pcie-topology.md).
NTB (Non-Transparent Bridge) — альтернатива: два root complex видят друг друга через PCIe,
без общей памяти. В проекте NTB **не** основной путь: xGMI быстрее и «из коробки».

## Решение

- **Основной путь**: cross-socket P2P через xGMI (ACS off, IOMMU off — компромисс).
- **Fallback**: NTB — если P2P недоступен (ACS/BIOS не позволяют) или нужна изоляция.
- NTB **не** используется в базовой конфигурации 4×4U; только как резерв.
- Тест: `lspci -vv`, P2P-бенчмарк, проверка ACS/IOMMU — до закладки в проект.

## Следствия

+ xGMI — быстрее, проще, «из коробки» на 2P EPYC.
+ NTB — fallback, если P2P сломан (ACS/BIOS).
- NTB добавляет сложность (два root complex, драйверы).
- Требует тестов до закладки (не «вслепую»).
