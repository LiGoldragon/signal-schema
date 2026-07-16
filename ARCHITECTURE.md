# signal-schema architecture

This pure typed wire contract makes `TypeSchema` the first implemented document root, then exposes the accepted `SignalContract`, `NexusRuntime`, and `SemaStorage` roots through the shared typed `DeclarationRoot`. The schema daemon owns ingestion, actors, and persistence through the central Sema daemon.

## Revisable lean
The three post-TypeSchema roots initially share one generic declaration record. Richer root-specific semantics wait for the accepted review-later document-kind design.
