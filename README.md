# signal-schema

`signal-schema` is the wire contract for the live schema component. It carries
only schema-derived request and reply types for loading an authored schema
package into a runtime slot and asking for Rust emission from a loaded package.

The contract owns `LoadPackage`, `EmitRust`, `PackageLoaded`, `RustEmitted`, and
`Rejected`. Runtime state, actors, persistence, and the current schema/schema-rust
bridge live in the separate runtime component.
