# Unnamed Entity Component System v2

## Features

- Sparse-set based component storage.
- Spawn entities.
- Systems are just functions, as with any Rust ECS libraries.

## Missing features

I'm not actively working on this project, but I'll probably come back to it at some point.

- Despawn entities
  Entity ids can be deallocated and reallocated, but this isn't exposed in the API. The eventual API should remove all components from an entity before despawning it.
- Iteration over multiple component types.
  This is kinda tricky (to do efficiently). I don't want to use archetypes.
- Filtering queries
- Scheduling, system sets, etc.
  Scheduling is probably overkill.
- Events
