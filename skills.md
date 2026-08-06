# Working in meta-signal-router

Read `ARCHITECTURE.md`, the ordinary Router Interface, and the shared standard
Interface before changing this contract.

- Treat `ethos/interface.ethos` as the sole schema authority.
- Preserve the exact producer identities; add an import instead of copying a
  producer declaration.
- Keep the Interface role-free and keep generated Rust encoded-only.
- Mint explicit declaration and variant seats; never derive identity from a
  spelling, source position, or hash.
- Put structural, Dotos, rkyv, route, and Signal behavior in
  `src/schema/lib/behavior.rs`, never in the generated projection.
- Do not add runtime actors, policy stores, sockets, or daemon behavior here.
- Regenerate only with
  `META_SIGNAL_ROUTER_UPDATE_INTERFACE_ARTIFACTS=1 cargo build --all-features`,
  then prove an ordinary build is fresh.
- Run default and all-feature tests, formatting, clippy with warnings denied,
  rustdoc with warnings denied, and `nix flake check --all-systems`.
