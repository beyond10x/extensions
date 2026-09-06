# Extensions

Public specifications for the foundation extension control service.

This repository provides a **validated causal model draft**, generated JSON Schemas and a
reproducible gate. Eighteen commands describe activation, contribution registration, staged upgrade,
removal, control detachment and separate scoped data destruction, including failure and retry. It does not yet provide an installation server or runtime reconciler. The model is a draft contract; `v1` names its ESS system version, not a released API.

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

[Binding and activation contracts](docs/model.md#binding-and-activation-proposal) distinguish stable
owner registration identity from generation/attempt and keep current action authority separate
from compatibility references. [Upgrade and retention contracts](docs/model.md#upgrade-removal-and-retained-data-proposal)
keep the selected release intact until candidate confirmation and separate control cleanup from
authorized data destruction. [Model decisions and open semantics](docs/model.md) explain what remains
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
generates schemas twice and compares them against the committed outputs. Six mutated models
must be refused for an unknown relation target, a mistyped relation carrier, a second owner, missing
transition causation, a mistyped command instance carrier and a state-mutating refusal. The gate
also asserts all eighteen command transitions, typed input/event mappings and refusal outcomes.
These are compiler and structural-contract checks; runtime scenarios remain unimplemented.

Schema fixtures also exercise configuration references, preinstallation identity, independent
locks, scoped limits, count membership, both binding target forms, required/optional declarations,
activation/upgrade/removal progress, candidate readiness, retained-data policy, scoped destruction
and all eighteen command inputs and events. Numeric ESS invariants remain annotations in these
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
Typed configuration, policy, bindings, activation, upgrade, removal and scoped destruction are draft
model contracts.
The next designs must settle authenticated owner adapters, cross-record readiness, stable-key
recovery, storage isolation, admission/update commands, atomic selection/count changes and owner
migration/detachment/data-destruction guarantees. SDK evolution,
extension repository migrations and host implementation are separate work.

## License

[Apache-2.0](LICENSE).

<!-- b10x-docs:start -->
## Documentation

[Extensions documentation](https://beyond10x.github.io/docs/extensions/) · [Start](https://beyond10x.github.io/) · [Ecosystem](https://beyond10x.github.io/ecosystem/) · [Impact](https://beyond10x.github.io/changes/) · [Releases](https://beyond10x.github.io/releases/)
<!-- b10x-docs:end -->
