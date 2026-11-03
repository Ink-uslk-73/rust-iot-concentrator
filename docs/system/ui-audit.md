# UI: Network Audit (P3.44)

**Статус:** accepted (P3.44).
**Связанные:** [ui-chat.md](ui-chat.md), [satellite-vm.md](satellite-vm.md).

## Роль

Аудитор: собирает доказательства, отчитывается о разнице (as-designed vs as-built).
Не чинит, не проектирует, не настраивает. Read-only.

> The moment an auditor starts fixing things, they stop being able to audit them.

## Три метода

| Метод | Что |
|---|---|
| EXAMINE | Читает артефакты: config exports, rulebase, log extracts, ACL, ticket |
| INTERVIEW | Спрашивает людей: review реально происходит или только calendar invite? |
| TEST | Проверяет операционную эффективность: MFA coverage, JML, privileged access, segregation of duties |

## Что проверяет

- Access control: JML в реальности (не в policy), MFA coverage, break-glass, service accounts;
- Compliance: SOX 404 ITGC, ISO 27001 (PCI DSS / HIPAA — по требованию);
- Change management: кто аппрувил, когда, rollback-план;
- Network segmentation: соответствует ли as-built design-документу;
- Logging & monitoring: покрывают ли логи требования.

## Где

- UI — CORE (axum + SSE, веб, IPv6 ULA), вкладка в общем чате ЦУ;
- LLM — локальный (Ollama/llama.cpp) в AUDIT-VM (изоляция, read-only к CORE);
- Отчёты/сводки/рекомендации — через чат (LLM читает артефакты, не чинит).

## Открытые вопросы

- [ ] LLM: Ollama или llama.cpp (AUDIT-VM)?
- [ ] SOX 404 ITGC — базовый (или по требованию)?
- [ ] AUDIT-VM: read-only доступ к CORE (как? rsync/API?)
