---
format: aep.planning-md/1
id: executable-system-specification:extension-control-model
kind: executable-system-specification
status: draft
title: Extension control domain model
summary: Public causal ESS draft for extension activation, staged upgrades, confirmed removal and scoped data destruction.
revision: 9
---
# Extension control domain model

## Outcome

The ESS sources under ess/ declare stable extension identity, immutable release declarations,
named tenant-owned installations, contribution declarations and installation-specific registrations.
The extensions component owns these five control entities and their five ownership/reference
relations. Foundation-owned agents, runs, Connections, credentials and business data retain their
existing owners.

## Current scope and maturity

This is an unreleased causal ESS model draft. It specifies configuration and policy, installation
bindings, activation and registration recovery, upgrade selection, confirmed removal with data
retention, and separately authorized data destruction. It contains no installation API or running
service. See [model decisions and UNMAPPED semantics](../../../docs/model.md) and
[operational scenarios](../../../docs/scenarios.md).

The pinned ESS compiler validates lifecycle moves and typed causal event payloads. The model
does not claim field assignment, authenticated owner admission, cross-record consistency,
durable replay, atomic transactions or physical deletion. Runtime decomposition must first resolve
the owner and persistence contracts needed by the proposed implementation.

## Configuration, policy and binding contract

Each release declares its configuration schema and technical policy constraints. Each installation
retains revisioned configuration and binding snapshots, trusted deployment identity, origin,
admission retry key, effective policy and Held/Released count membership. Independent locks combine
by logical OR, editable setting paths by exact intersection, and applicable count limits by the
minimum in each scope. Admission reserves applicable scopes atomically; retries reuse identity.

Bindings select targets and compatibility evidence without transferring owner-resource custody.
Registration identity remains stable across retries of the same declaration intent. Activation
requires the selected release's required contributions to be ready. Candidate upgrade registrations
are separately scoped and do not grant selected-release readiness.

## Upgrade, removal and data retention

An upgrade stages exact current and candidate snapshots from the same Extension under a
stable operation identity. Confirmation requires matching compatibility, admitted migration or
preservation, and required-readiness evidence. Failed or refused work preserves the selected
release, original configuration, business data and provenance. Independent version/configuration
locks and current authority apply.

Removal inventories control registrations and resolves dependency, quiescence and detachment
obligations. Held count membership persists until confirmed removal and is released once.
Control detachment retains foundation-owned resources and business data. Retain is the default.
Destruction requires its own authorized post-removal intent, exact data scope and owner result;
ambiguous outcomes remain visible. The installation tombstone and provenance remain retained.

These are normative model requirements. Equality of evidence, authority checks, atomic field
updates, durable owner recovery, retention duration, physical deletion and backup guarantees remain
explicit runtime or UNMAPPED obligations.

## Historical verification

The initial structural pass validated three ESS fragments, five entities and 18 deterministic
schema artifacts. The configuration/policy pass expanded this to 36 schemas, 15 accepted and
26 refused examples, with two numeric invariant annotation-gap controls.

The published binding/activation model at f9688b9ff24e96b877522cdd3da613e6cb5977ed passed
220 controls: six exact causal-command checks, six compiler refusal mutations, 75 accepted
and 128 refused schema examples, and five numeric annotation-gap controls. It generated
77 deterministic schemas. The adversary added 58 of those examples and separately refused
36 causal IR mutants. These measurements describe that historical source, not the new model.

## Upgrade and retention verification

The reviewed unit at 7a3a2404a50ddc6b7de7d541a6559e910fd90b37 passed the complete model
gate with 646 controls: 18 exact causal-command controls, six compiler refusal mutations,
184 accepted and 433 refused schema examples, and five numeric annotation-gap controls.
All 128 generated schemas were deterministic and current; formatting and strict Clippy passed.
All 203 original schema examples remain unchanged.

The deciding checks were written first and failed against the six-command model and missing
new schema. Adversarial review retained 63 additional examples across two passes and separately
refused ten causal IR mutants. The first pass found a declared-contract gap: pending candidate
registrations could lack an admitted recovery path after upgrade failure, while removal required
their settlement. Correction cf49dafee532fff9d85680704abacd4a19356dc7 explicitly permits
authenticated observation-only recovery of the exact stored pending attempt after failure and
during removal, without new dispatch, selected-readiness credit or reopened control reach.
The second pass found no remaining or new defect. Its schema cases establish envelope shapes,
not execution of that external guard.

The unreleased draft adds optional progress/context fields and new state, refusal and failure
variants. Historical backfill must use actual evidence; absence cannot establish successful
readiness, cleanup, authorization or destruction. Strict old readers need a separately versioned
migration before accepting the extended contract.

The compiler remains ESS 0.9.2 from exact public source
6ef4af76b99a8d2cd861a3cc76140c88c1361129. Generated contracts are produced only through its
generator. Operational scenarios remain future runtime acceptance obligations; no schema or
compiler check is described as an executed runtime scenario. This model pass creates no tagged
release and performs no runtime implementation decomposition.
