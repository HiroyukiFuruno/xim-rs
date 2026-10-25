# Publishing the GPUI compound-text fix

The decoder regression is fixed by the published `xim-ctext` 0.4.1. This
release preserves Zed's `reset_ic` and error-handling APIs. It raises the stated
minimum Rust version from 1.64 to 1.88 (the currently resolved
`encoding_rs` 0.8.42 requires Rust 1.88), so the proposed version is 0.5.0-zed.
The original upstream `xim` 0.5 cannot simply replace this fork: its client API
no longer provides `reset_ic`.

## Release order

1. Review and merge the dependency/regression PR; require the Check workflow,
   including the Rust 1.88 build, to pass.
2. A canonical crate owner runs **Publish zed-xim** on the merged `main` commit
   with the exact manifest version and `publish=false`. This checks the actual
   distributable package against registry dependencies.
3. The owner reruns that same commit with `publish=true`. The workflow permits
   actual publication only in `zed-industries/xim-rs` on `main` when dispatched
   by the recorded individual owners `mikayla-maki` or `MrSubidubi`, uses the
   repository's existing `CARGO_REGISTRY_TOKEN` secret, and checks the resulting
   registry checksum against the packaged crate. No token is needed for the
   dry run. Configure the secret through the owner's existing secure process;
   never put its value in an issue, log, or PR.
4. Only after the registry version exists, update GPUI's immutable git revision
   and `version` requirement together. A git source being merged does not mean
   the crates.io dependency is available.
5. The current Zed main splits Linux support into `gpui_linux` and sets workspace
   `publish=false`. The GPUI maintainer must also backport the dependency change
   to the compatible 0.2.x publication source (tag `gpui-v0.2.2`), run the GPUI
   packaging checks, and publish a compatible GPUI release. A main-branch fix
   alone does not fix existing crates.io GPUI 0.2.2 consumers.
6. Downstream KCU resolves the published GPUI release from crates.io, reruns its
   native IME and quality checks, and publishes through its own GitHub Actions.
   No final downstream path/git override is permitted.

The canonical crate's recorded owners are `mikayla-maki`, `MrSubidubi`, and
`github:zed-industries:crates-io`. Administrative rights on a contribution fork
do not grant publication rights to that package. The contribution PR can
complete its implementation, tests, review, and release preparation without
claiming that canonical publication already happened.

Team-only publishers must first add their individual login to the reviewed
workflow allowlist, backed by registry ownership evidence. Fork administrators
and ordinary repository writers are not allowed by this workflow to publish.
