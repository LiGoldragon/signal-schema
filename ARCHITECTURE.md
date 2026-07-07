# signal-schema — Architecture

`signal-schema` is the ordinary Signal contract for live schema package loading
and Rust emission. It is a schema-derived `WireContract` crate: `schema/lib.schema`
is the authored source, and `schema-rust` emits the checked-in `src/schema/lib.rs`
wire surface.

## Direction

This contract serves the replacement path for the TrueSchema stack, not a
compatibility layer over the older shape. The end target is
`TrueSchema = CoreTrueSchema + TextProjection`; `SchemaEvolution X_to_Y` is a
separate concept, and a rename is always an evolution no-op. This first scaffold
uses the current `schema`/`schema-rust` TrueSchema branch as an execution bridge
only. It does not define the final `CoreTrueSchema` model and does not present
the current string-bearing `TrueSchema::content_hash()` as the final core hash.

The contract follows settled workspace intent: deterministic derivable behavior
belongs in schema/code machinery (Spirit `w312`), structured data goes through
canonical codecs (Spirit `qvb3`), closed typed data is preferred over strings for
fixed vocabularies (Spirit `16jw`), and replacement is preferred over additive
legacy compatibility when the older shape is wrong (Spirit `10pz`).

## Boundary

The runtime component owns the in-memory `SchemaSlot` store, current-stack
package loading/parsing, Rust emission, actors, sockets, and any future durable
state. This crate owns only the peer-callable wire nouns:

- `LoadPackage` loads a named package version and its module sources into a slot.
- `EmitRust` asks the runtime to emit Rust for one module from a loaded slot.
- `PackageLoaded` acknowledges the loaded package and module roster.
- `RustEmitted` returns emitted Rust text for the requested module.
- `Rejected` gives a typed reason for a rejected `LoadPackage` or `EmitRust`.

## Invariants

- The contract has no daemon runtime dependencies: no actors, `tokio`, sockets,
  storage, or current-stack parser ownership.
- Failure vocabulary is closed through `RejectionReason`; no catch-all string
  error decides control flow.
- Schema and Rust source text are typed payloads at the wire boundary, not final
  semantic core identities.
- Every request and reply variant has rkyv frame and NOTA round-trip witnesses.

## Code map

```text
schema/lib.schema       schema-rust source of truth for the wire surface
src/schema/lib.rs       generated schema-rust artifact
src/lib.rs              contract entry and generated-surface re-exports
build.rs                ContractCrateBuild freshness gate
tests/round_trip.rs     request/reply rkyv frame and NOTA witnesses
tests/dependency_boundary.rs default dependency boundary witness
examples/canonical.nota compact human-readable wire examples
```
