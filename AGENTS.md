# Extensions

This public repository owns the extension control domain: installation records, release and
contribution references, bindings, policy and activation reconciliation. Foundation services
retain their execution, domain resources and authority.

## Serves

- O1: Governed reach through explicit bindings and ordinary runtime admission.
- O2: Validated specifications and lifecycle evidence.
- O4: Independently installed extensions on the foundation.
- O5: Agent and workflow contributions retain their foundation owners.

## Working here

Use managed worktrees for changes. Keep executable model sources under `ess/` and update planning
artifacts only through AEP. Public source, documentation and gates must work without private
repositories, credentials or local sibling checkouts.

Agents declare their own `AEP_ACTOR=agent:<task-id>` for planning writes; do not let the CLI
attribute an agent's changes to the local login name.

Run `cargo xtask check` before publishing changes. ESS owns parsing, validation and generated
projections; never hand-edit `contracts/`. Run `cargo xtask generate` to regenerate them.
The gate's pinned toolchain is documented in `README.md`.

This is a structural model scaffold. Do not add implementation stories around an undeclared
entity, guess a relation's ownership/cardinality, or treat `Recorded` as operational activation.
Resolve the relevant `UNMAPPED:` items in `docs/model.md` before implementing their behavior.

<!-- b10x-docs-operations:start -->
## Public documentation operations

This repository owns the public source and presentation allowlist in `b10x.docs.yaml`. The generated credential-free `.github/workflows/b10x-docs-bundle.yml` passively packages only those declared files for the exact successful `main` commit; it must never run repository code. Atlas selects the latest successful bundle with every other catalog source, and Website plus Docs System own rendering, shared components, search, and feeds. Do not add a standalone docs deployer or put App credentials in this public repository. If Atlas catalogs a former Pages workflow, that file remains repository-owned validation: preserve its bespoke checks while keeping exact read-only permissions, an unconditional pull-request trigger, and no deployment primitives. Project Pages at `/extensions/` is only the generated stable redirect façade in `.github/workflows/b10x-docs-pages.yml`; content-only publication never rebuilds it.

From the complete organization workspace, verify the contract with a clean Atlas checkout at the current remote `main`. Set `B10X_ATLAS_CHECKOUT` to a managed Atlas worktree when the primary checkout is dirty or stale; never infer command availability from the primary alone.

```bash
atlas_checkout="${B10X_ATLAS_CHECKOUT:-atlas}"
atlas_head="$(git -C "$atlas_checkout" rev-parse HEAD)"
atlas_main="$(git -C "$atlas_checkout" ls-remote origin refs/heads/main | awk '{print $1}')"
test -z "$(git -C "$atlas_checkout" status --porcelain)"
test "$atlas_head" = "$atlas_main"
cargo run --manifest-path "$atlas_checkout/Cargo.toml" --locked -q -- \
  --store "$atlas_checkout/catalog/store" docs reconcile --workspace . --check
```

Keep internal plans, stories, ADRs, decisions, worklogs, security material, and research out of the public allowlist unless a repository authority explicitly declares them public.
<!-- b10x-docs-operations:end -->

<!-- b10x-release-operations:start -->
## Release completion

An ordinary release completes after this repository's exact tag, required source checks,
published release and required artifacts are verified. A pushed tag with unfinished checks or
uploads is queued; report it as released only after those requirements succeed.

Atlas reconciliation and public documentation publication run asynchronously. Do not wait for
Atlas or Website, update Website source locks or bootstrap snapshots, promote consumer pins,
release shared docs tooling, or redeploy documentation façades as part of an ordinary source
release. Report documentation as pending unless its publication was actually verified. A background
documentation failure does not invalidate a successful source release.

Keep this repository's provenance, correctness, security, compatibility and artifact verification
requirements. Shared rendering, routing or delivery-control changes still require their relevant
integration gates. A release request does not authorize deployment or downstream releases.
Repositories without a release unit retain their existing publication policy. This completion
boundary supersedes older instructions that attach synchronous documentation ceremony to each
source release.
<!-- b10x-release-operations:end -->
