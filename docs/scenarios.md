# Architecture validation scenarios

These are requirements for later designs and implementations. The current gate validates the
structural model and compiler refusals; it does not execute these scenarios.

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

The later lifecycle must provide causal commands, events and durable evidence for these cases.
In particular, ESS `Recorded` and JSON Schema acceptance are insufficient evidence for any row.
