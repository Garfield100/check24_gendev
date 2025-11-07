# Temp Notes
<!-- TODO finalise -->

## Architectural choices

* Hexagonal architecture pattern
* E​​vent-driven/push cache: products update the home service when sth new happens instead of the latter polling regularly
* Server-driven UI: json description of UI, front-end renders that instead of having templates for every product -> "content updates can and should remain dynamic"

## Tech choices

### Home service backend
<!-- TODO yap about Rust -->
* Rust ofc
* Axum:
  * Fast, estabilished, maintained, ergonomic, Tower/Tokio ecosystem
  *
