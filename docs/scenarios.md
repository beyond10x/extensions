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
