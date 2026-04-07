# Trinity

Deterministic state enforcement engine for reactor and grid systems.

## Scope

- bounded state validation
- explicit fault emission
- hash-linked event journal

## Model

State is a key/value map of floating-point metrics.

Limits define hard boundaries:
value < min OR value > max → fault

## Components

- state.rs — mutable state container
- bounds.rs — constraint evaluation
- reactor.rs — decision layer
- journal.rs — integrity chain
- main.rs — execution driver

## Journal

Each event is chained via SHA-256:

hash = sha256(prev_hash || event)

Provides:
- append-only log
- tamper visibility

## Execution
