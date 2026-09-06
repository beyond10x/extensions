---
format: aep.planning-md/1
id: executable-system-specification:extension-control-model
kind: executable-system-specification
status: draft
title: Extension control domain model
summary: Public structural ESS model for extension releases, tenant installations and contribution registration.
revision: 7
---
# Extension control domain model

## Outcome

The ESS sources under ess/ declare stable extension identity, immutable release declarations, named tenant-owned installations, contribution declarations and installation-specific registrations. The extensions component owns this control domain. Foundation-owned agents, runs, Connections, credentials and business data retain their existing owners.

## Scope and status

This is a validated structural draft with a reproducible compiler gate and no installation API or runtime. See [model decisions and UNMAPPED semantics](../../../docs/model.md) and [operational scenarios](../../../docs/scenarios.md). Implementation decomposition follows resolution of the semantics needed by each proposed change.

## Verification

`cargo xtask check` passed using ESS 0.9.2 built from exact public source revision 6ef4af76b99a8d2cd861a3cc76140c88c1361129. The compiler validated three fragments and five entities. Two canonical compilations and two schema generations were byte-identical, and all 18 generated artifacts matched contracts/. Mutations introducing an unknown relation target, a wrong carrier type and a second owner each produced the required semantic refusal. Formatting and strict Clippy passed.

The gate originally recognized the mistyped-carrier diagnostic too narrowly; matching the compiler's observed type_mismatch code corrected the check. The unmodified model remained valid. Passing these checks does not establish operational installation, policy, recovery or authorization behavior.

## Configuration and policy pass

This interactive pass extends the existing draft in ess/domains/extensions.yaml, its generated contracts, docs/model.md, docs/scenarios.md and the validation tooling. It does not decompose runtime stories or claim operational activation. Proposed value contracts keep the five existing entity relations unchanged.

Model proposal: each release declares an exact JSON Schema 2020-12 configuration contract and technical policy constraints. Each installation carries a revisioned immutable configuration-document reference, trusted deployment identity, requested/preinstalled origin, admission retry key, effective policy snapshot and Held/Released count membership. Configuration documents belong to the installation namespace and remain distinct from executable release artifacts and foundation credential custody.

Policy proposals: independent version, enable/disable, removal and configuration locks combine by logical OR; editable setting paths combine by exact intersection; applicable per-extension tenant/deployment count limits combine by minimum in each scope. An admitted installation holds capacity through pending, failed, disabled and removal-in-progress conditions. Only confirmed removal releases it. Admission and reservations across all applicable scopes must commit atomically; retries and deployment reconciliation reuse the same identity. These are normative draft requirements, not behavior established by JSON Schema validation.

Scope excludes new runtime commands, physical persistence/reservation algorithms, foundation bindings and external identity relation cardinalities. Remaining unknowns retain UNMAPPED markers. Verification will include compiler validation, deterministic projections and schema-positive/refusal fixtures. The public model stays draft for review.

## Current verification

`cargo xtask check` exited 0 for the configuration/policy draft: three ESS fragments, five entities, five explicit relations, no runtime commands, and 36 deterministic/current generated schemas. The existing three relation mutations were refused. JSON Schema controls accepted 15 examples and refused 26 malformed examples. Two checks confirm that positive configuration revisions and nonnegative counts remain ESS invariant annotations rather than JSON Schema assertions. Formatting and strict Clippy passed.

The first local invariant spelling used an unsupported name/expr object and ESS refused it as unobservable_fact with dependent undeclared_reference diagnostics. Reading the pinned compiler's predicate contract established the supported scalar form; the sources now declare `value > 0` and `value >= 0`, and validate successfully. Generated files were produced only by the pinned ESS generator.

The proposal remains draft. The 16 concrete configuration/policy cases in docs/scenarios.md are admission/runtime acceptance obligations, not passing runtime tests. No implementation decomposition was performed, so a decomposition critic panel does not apply. This was an interactive modeling pass; no non-interactive approval or bypass record was created.

## Binding and activation model pass

This pass extends the existing five control entities with installation-scoped binding values and
causal activation/recovery contracts. The source remains an unreleased ESS draft. It defines
contribution readiness, durable retry identity and selected-release consistency as explicit model
requirements, keeping authentication and owner resources with their foundation services.

The model unit updates ess/, compiler-owned contracts/, schema fixtures, the Rust model gate and
public model/scenario documentation. The pinned ESS compiler can validate causal lifecycle/event
structure; field assignment, cross-record checks, physical namespace encoding and durable
transactions remain runtime obligations. No implementation server, wire migration, SDK rename,
upgrade/removal implementation or tagged release is included. Evidence will distinguish executed
compiler/schema refusals from the future runtime scenarios.

## Binding and activation verification

The candidate at 0c529f3b10dd84a111a303b9de1db8ef3941612a passed the complete model gate with
the unchanged pinned ESS compiler: five entities/five relations, six causal commands with exact
transitions/instances/event payloads/refusals, 77 deterministic schemas, six compiler refusal
mutations, 55 accepted and 90 refused schema examples, and five numeric annotation-gap controls.
Formatting and strict Clippy exited 0. The red-first causal check observed zero commands before
the six-command draft was added; the new binding schema was absent before generation.

The candidate adds binding target/compatibility values, required or optional contribution flags,
activation/registration progress and stable owner registration keys distinct from generation and
attempt counters. Commands operate on existing records; admission/allocation, configuration
mutation, upgrades, removal and data destruction remain outside this pass.

The original Recorded-progress fixture used null, but the pinned projection makes an Optional field
omittable and refuses explicit null. Corrected positive examples omit the field and negative
examples preserve the explicit-null refusal. This repairs the fixture's absence spelling rather
than changing the model to satisfy it.

Independent adversarial review and integration checks follow this candidate verification.
The operational model remains draft: structural checks cannot establish authenticated owner
admission, snapshot equality, required readiness, durable replay, atomic event/field persistence
or physical installation isolation. Those requirements and unresolved owner contracts remain
explicit in docs/model.md and docs/scenarios.md.

## Adversarial result and integrated contract

The independent pass found no defect in the bounded model. Its 58 added fixture cases cover
error states/reasons, nested authority fields and required provenance; all 145 previous examples
remain unchanged and selected. The complete gate now executes 220 controls: 75 accepted,
128 refused, five annotation-gap controls, six causal-command checks and six compiler refusals.
The reviewer separately exercised the actual causal checker with 36 altered IR documents;
all were refused. These probes remain distinct from the gate's case count.

The reviewed source is being integrated together with those retained regression cases. The
compiler still generates 77 deterministic schemas. No runtime scenario or running installation
service is claimed; owner admission, exact snapshot/state consistency, durable retry recovery and
physical storage isolation remain the explicit implementation obligations.
