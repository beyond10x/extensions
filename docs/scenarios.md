# Architecture validation scenarios

These are requirements for later designs and implementations. The current gate validates the
causal draft, projected shapes and compiler refusals; it does not execute these scenarios.

| Scenario | Required observation | Principal owners |
|---|---|---|
| Todo | A declarative service and generated UI install without rebuilding the host. | SDK, Extensions, Todo, host |
| Phone | Backend and browser contributions install independently; panel movement preserves the call session. | SDK, Extensions, Phone, host |
| Two Phone installations | Configuration, routing, state and events remain separate in one tenant, including with shared processes. | SDK, Extensions, Phone, storage adapters |
| Preinstalled frozen extension | Lifecycle and version locks hold while explicitly editable settings remain editable. | Extensions, deployment tooling |
| Installation limit | Concurrent requests, including preinstallation reconciliation, stay within the applicable scoped count. | Extensions, persistence |
| Agent and workflow contribution | Definitions register with foundation owners; execution retains admission, authority and evidence. | Extensions, Agent Platform, Workflow |
| Cross-extension interaction | A compatible target installation resolves and caller authority is enforced. | Extensions, SDK, runtime owners |
| Interrupted activation | Retries create no duplicate agents, subscriptions, grants or resources and expose recoverable progress. | Extensions, runtime owners |
| Upgrade and removal | Exact provenance and retained data survive; dependencies preventing a transition are reported. | Extensions, domain/runtime owners |
| Unsupported manifest | Missing runtime capabilities are identified before activation. | ESS, SDK, Extensions |

Independent installation is required for the first unified hosting milestone. Operator-installed
executable releases may supply separate backends and dynamic UI assets. Tenant uploads use
declarative packages over admitted capabilities; adding a runtime-provider type requires a
platform upgrade.

Deployment tooling bootstraps the foundation before Extensions manages extensions. Preinstalled
instances count toward applicable limits. Installation references remain tenant-authorized domain
selectors, and installation identity must survive routing, persistence, idempotency, bindings,
subscriptions and contribution registration.

## Configuration and policy acceptance cases

These refine the model proposal in [model.md](model.md#configuration-and-installation-policy-proposal).
They are future admission/runtime obligations. The current schema fixtures test shapes and
refusals only; they do not execute the interleavings below.

| Case | Setup and action | Required outcome |
|---|---|---|
| Frozen editable setting | All four locks true; both policy layers allow `/display_name`; update that setting at the expected configuration revision. | New validated configuration revision; all locks remain true. |
| Mixed frozen update | The same installation changes `/display_name` and `/provider` together. | Entire update refused; no partial configuration or revision change. |
| Policy intersection | Release allows `/display_name`; deployment allows `/provider`. | Empty effective exception list; both edits refused while locked. |
| Independent locks | Only version updates are locked; an authorized caller disables the installation. | Disable may proceed under its ordinary admission rules; a release change is refused. |
| Two installations | Two Phone installations use document key `config/rev-1`; update the first. | Only its tenant/installation namespace and configuration revision change. |
| Optimistic update | Two updates name configuration revision 3 with different desired documents. | At most one commits revision 4; the other gets a conflict. |
| Preinstallation retry | Bootstrap reconciliation repeats the same deployment/tenant/extension/declaration tuple. | One installation and one Held membership; existing configured values survive. |
| Preinstallation change | The same declaration selects a new release while version updates are locked. | Report the blocked update against the existing instance; create no replacement or extra count. |
| Last available slot | Tenant maximum is 1; two distinct requested admissions race from zero usage. | Exactly one Held installation commits; the loser has neither an installation nor a reservation. |
| Two scopes | Tenant capacity is available but Deployment capacity is exhausted, including other tenants. | Admission refuses without leaking a Tenant reservation. |
| Cross-deployment tenant limit | One tenant has a Held Phone in deployment A; its Tenant maximum is 1; it requests Phone in B. | Refused by the shared Tenant bucket even if B has Deployment capacity. |
| Failure and cleanup | Activation fails after a registration; retry, then request removal while cleanup is interrupted. | One Held count persists until confirmed cleanup; release capacity once, without destroying retained data. |
| Lowered limit | Current Held usage is 2; deployment lowers maximum to 1. | Keep both records, expose over-limit usage, refuse new admission; no-op retries do not consume capacity. |
| Crash after admission | Persist installation and multi-scope reservation, lose the response, then retry. | Return the recorded outcome without a second reservation. |
| Upgrade configuration | A candidate schema rejects an explicitly configured value and no migration applies. | Keep the current release and configuration pair; no activation switch or extra count. |
| Unsupported configuration | Schema dialect/reference closure is unsupported, or a document fails the admitted schema. | Visible refusal before that configuration becomes current. |

The later runtime must execute the declared activation commands and add the admission/update/removal
commands and durable evidence required by these cases.
In particular, ESS `Recorded` and JSON Schema acceptance are insufficient evidence for any row.


## Binding and activation acceptance cases

The current gate executes projected examples and compiler mutation controls for the draft in
[model.md](model.md#binding-and-activation-proposal). The rows below require a runtime, authenticated
owner adapters and persistence; **none of these runtime cases is executed by this repository**.

| Case | Setup and action | Required observation |
|---|---|---|
| Foundation binding | An installation selects an admitted foundation capability with an exact compatibility revision. | Capability and compatibility resolve; the owner still admits each current action. |
| Named installation target | Phone east selects Phone west's declared calling capability in the same tenant. | Resolve the selected installation after authentication; preserve target identity through routing. |
| Foreign or self target | A selector names another tenant's installation or itself. | Refused before activation; no tenant/state disclosure and no owner operation. |
| Missing or incompatible capability | The target is absent, unsupported, or mismatches the selected contract revision. | Visible binding failure; no activation through that unresolved dependency. |
| Authority revocation | A previously valid binding remains stored while owner policy revokes the action. | Next action refuses under current policy; compatibility and Active state grant no authority. |
| Exact realm | Run with absent realm, then with an authenticated realm, using two installations. | Preserve absence or exact value; installation selection never overwrites realm or tenant/actor/executor authority. |
| Two installation isolation | Two named instances share tenant, processes, document keys and operation names. | Separate state/configuration, routes, event visibility, subscriptions, registrations and idempotency results. |
| Declaration mismatch | Registration names a declaration from a release other than the installation's selected release. | ReleaseMismatch before owner dispatch; original records and progress stay unchanged. |
| Stale observation | Delayed success/failure names an old generation, attempt, configuration or binding snapshot. | StaleSnapshot; no progress overwrite, readiness credit or lifecycle move. |
| Wrong owner | An application caller or another runtime submits a plausible readiness receipt. | Ordinary admission refuses it; no owner readiness inferred from input shape. |
| Required readiness | One Required declaration has no registration, or has failed/not-ready evidence. | ConfirmActivation returns RequiredNotReady; no Active state or activation event. |
| Optional failure | All Required declarations are Ready; an Optional Agent or UI declaration failed. | Failure remains visible; optional category alone does not block Active. |
| Durable begin | Begin activation/registration succeeds, then the reconciler crashes before owner dispatch. | Persisted attempt/snapshot and event permit recovery of the same intent without duplicate work. |
| Lost owner response | Owner creates an agent/subscription/grant, then its response is lost. | Recover by the unchanged owner registration key; do not allocate another resource on timeout. |
| New generation | A failed activation retries as a new generation after ambiguous owner work. | Same installation/declaration registration key survives; old observations cannot mark the new generation ready. |
| Previously Ready contribution | Another required contribution failed; retry activation with a changed generation. | Preserve existing owner resources; require current owner re-attestation, and remain blocked if the owner contract cannot supply it. |
| Owner cannot recover | An owner has no lookup/replay guarantee for an ambiguous result. | Keep recoverable failure/progress visible and block blind retry; do not claim exactly-once resources. |
| Exact duplicate outcome | Lose a successful command response, then redeliver the exact authenticated intent after its state move. | Durable replay returns the recorded outcome without another event, transition or owner request. |
| Changed retry intent | Reuse a replay identity or owner key with different declaration/snapshot intent. | RetryConflict; no second resource or silent overwrite. |
| Fresh wrong-state request | Send a fresh BeginRegistration in Registering or Ready; send ConfirmActivation in Recorded. | Typed state conflict; no effect. Exact replay is distinguished before dispatch. |
| Failure evidence | Record authenticated owner failure during Registering/Activating. | Typed failure/provenance persists; count remains Held and partial owner resources remain visible. |
| Numeric annotation limit | Schema accepts zero generation/attempt/revision or negative installation count. | Runtime validation refuses the invalid domain value despite structural acceptance. |

No runtime row selects an owner resource count, physical namespace encoding, atomic storage/replay
mechanism, retention duration or upgrade protocol. Those owner contracts remain `UNMAPPED:`. SDK
storage wire and foundation authority are unchanged by this model pass.
