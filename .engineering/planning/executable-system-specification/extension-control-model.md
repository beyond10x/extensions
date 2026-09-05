---
format: aep.planning-md/1
id: executable-system-specification:extension-control-model
kind: executable-system-specification
status: draft
title: Extension control domain model
summary: Public structural ESS model for extension releases, tenant installations and contribution registration.
revision: 2
---
# Extension control domain model

## Outcome

The ESS sources under ess/ declare stable extension identity, immutable release declarations, named tenant-owned installations, contribution declarations and installation-specific registrations. The extensions component owns this control domain. Foundation-owned agents, runs, Connections, credentials and business data retain their existing owners.

## Scope and status

This is a validated structural draft with a reproducible compiler gate and no installation API or runtime. See [model decisions and UNMAPPED semantics](../../../docs/model.md) and [operational scenarios](../../../docs/scenarios.md). Implementation decomposition follows resolution of the semantics needed by each proposed change.

## Verification

`cargo xtask check` passed using ESS 0.9.2 built from exact public source revision 6ef4af76b99a8d2cd861a3cc76140c88c1361129. The compiler validated three fragments and five entities. Two canonical compilations and two schema generations were byte-identical, and all 18 generated artifacts matched contracts/. Mutations introducing an unknown relation target, a wrong carrier type and a second owner each produced the required semantic refusal. Formatting and strict Clippy passed.

The gate originally recognized the mistyped-carrier diagnostic too narrowly; matching the compiler's observed type_mismatch code corrected the check. The unmodified model remained valid. Passing these checks does not establish operational installation, policy, recovery or authorization behavior.
