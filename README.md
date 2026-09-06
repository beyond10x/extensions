# Extensions

Public specifications for the foundation extension control service.

This repository starts with a **validated structural model**, generated JSON Schemas and a
reproducible gate. It does not yet provide an installation server or an executable activation
lifecycle. The model is a draft contract; `v1` names its ESS system version, not a released API.

Extensions will own installation records, dependency resolution, bindings, policy and activation
reconciliation. Agent Platform, Workflow and Connectors retain their runtime resources, execution
and authority. Hosts such as Devcenter own UI placement and browser interaction.

## Model

The source is [ess/system.yaml](ess/system.yaml), with
[control records and relations](ess/domains/extensions.yaml) owned by
[the extensions component](ess/components/extensions.yaml).

| Record | Role |
|---|---|
| Extension | Stable identity independent of repository naming |
| Release | Immutable specification, artifacts, contributions, configuration contract and policy constraints |
| Installation | Named tenant-owned instance selecting a release, configuration, origin and effective policy |
| ContributionDeclaration | A release-owned declaration naming its category, runtime and compatibility contract |
| ContributionRegistration | An installation-owned control record referring to a contribution declaration |

[Model decisions and open semantics](docs/model.md) explain what is represented and what remains
`UNMAPPED:`. [Architecture scenarios](docs/scenarios.md) define the later operational evidence.
Generated [entity schemas](contracts/schema/entities) expose the structural contract.

## Validate

Install a Rust toolchain with Cargo, rustfmt and Clippy, then run:

```bash
cargo xtask check
```

The first run builds ESS 0.9.2 from exact public revision
`6ef4af76b99a8d2cd861a3cc76140c88c1361129`. The gate verifies Cargo's installation source
record and the compiler version before using it. A dedicated cache can be selected with
`EXTENSIONS_ESS_TOOLCHAIN_ROOT`; the same source check applies.

The gate checks formatting and Clippy, validates all ESS fragments, compiles the model twice,
generates schemas twice and compares them against the committed outputs. Three mutated models
must be refused for an unknown relation target, a mistyped relation carrier and a second owner.
These are compiler and structural-contract checks, not installation-runtime tests.

Schema fixtures also exercise configuration references, preinstallation identity, independent
locks, scoped limits and count membership. Numeric ESS invariants remain annotations in these
projections; the gate explicitly confirms that schema validation alone does not enforce them.
The [configuration and policy proposal](docs/model.md#configuration-and-installation-policy-proposal)
defines admission, retry and count requirements for the later runtime.

Regenerate the schema tree after an intentional model change:

```bash
cargo xtask generate
cargo xtask check
```

The pinned CLI uses `ess validate` and `ess compile`; it has no `ess specify` command group.

## Planning and implementation

The [AEP specification record](.engineering/planning/executable-system-specification/extension-control-model.md)
tracks model evidence. Its governing profile resolves from an exact public AEP revision.
No private repository, deployment configuration or credential is required to build or validate
this repository.

Before implementation decomposition, resolve the model semantics needed by each proposed change.
The next designs cover typed configuration and policy, binding targets, authenticated tenant
references, causal activation/recovery commands and retained-data transitions. SDK evolution,
extension repository migrations and host implementation are separate work.

## License

[Apache-2.0](LICENSE).

<!-- b10x-docs:start -->
## Documentation

[Extensions documentation](https://beyond10x.github.io/docs/extensions/) · [Start](https://beyond10x.github.io/) · [Ecosystem](https://beyond10x.github.io/ecosystem/) · [Impact](https://beyond10x.github.io/changes/) · [Releases](https://beyond10x.github.io/releases/)
<!-- b10x-docs:end -->
