# Trinity

Deterministic state enforcement engine for reactor and grid systems.

## Scope

- bounded state validation
- explicit fault emission
- hash-linked event journal

## Model

State is represented as a key/value map of floating-point metrics.

Limits define hard boundaries:

value < min OR value > max → fault condition

No implicit assumptions. No soft thresholds.

## Architecture

- state.rs  
  mutable state container, applies incoming updates

- bounds.rs  
  evaluates state against strict limits

- reactor.rs  
  decision layer, emits system state (STABLE / FAULT)

- journal.rs  
  append-only hash-linked log

- main.rs  
  execution driver with deterministic input stream

## Journal Design

Each event is chained:

hash = sha256(prev_hash || event)

Properties:

- append-only
- tamper-evident
- order-preserving

Any mutation invalidates the chain from that point forward.

## Fault Model

A fault is triggered when:

- value is below minimum
- value is above maximum

Each violation emits:

FAULT:<field>=<value>

System state is evaluated per update cycle.

## Execution
