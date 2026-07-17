# signal-schema architecture

This pure typed wire contract makes `TypeSchema` the first implemented document root, then exposes the accepted `SignalContract`, `NexusRuntime`, and `SemaStorage` roots through the shared typed `DeclarationRoot`. The schema daemon owns ingestion, actors, and persistence through the central Sema daemon.

## Revisable leans
- The three post-TypeSchema roots initially share one generic declaration record. Richer root-specific semantics wait for the accepted review-later document-kind design.
- **Signal-frame bypass.** This contract exposes raw rkyv `encode_request`/`encode_reply` payloads, and its daemon frames them with a hand-rolled u32-length + rkyv envelope rather than the workspace's shared `signal-frame` kernel. That trades away `signal-frame`'s short-header tap-anywhere observability — the uniform exchange framing readable at any hop. The lean holds while the prototype's point-to-point socket suffices. Revise it when cross-hop observability, shared handshake or version negotiation, or a common frame taxonomy is needed, adopting `signal-frame`'s `ExchangeFrame` as the transport.
