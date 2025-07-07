---
id: task-11
title: Implement Bridge
status: To Do
assignee: []
created_date: '2025-07-07'
labels: []
dependencies:
  - task-8
  - task-9
---

## Description

Implement the Bridge struct. It will manage multiple transports via enums, own the central routing table, and contain the main event loop (run()) for forwarding packets between its managed endpoints.

## Acceptance Criteria

- [ ] Bridge can manage multiple heterogeneous transports.
- [ ] Bridge correctly uses the routing table to forward packets.
- [ ] The 'host' feature flag controls the inclusion of this module.
