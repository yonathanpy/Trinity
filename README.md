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

cargo run
 
## Determinism

- no concurrency
- no randomness
- no external dependencies at runtime

Behavior is reproducible for identical input streams.

## Constraints

- no recovery logic
- no control actuation
- no distributed coordination

System is strictly observational and validation-focused.

## Extension Points

- replace static limits with external configuration loader
- introduce async ingestion (Tokio)
- expose metrics endpoint (Prometheus)
- integrate persistent storage backend

## Positioning

This is a validation and traceability primitive.

Designed for:

- reactor monitoring pipelines
- grid stability analysis
- safety boundary enforcement layers

Not intended as a full control system.
