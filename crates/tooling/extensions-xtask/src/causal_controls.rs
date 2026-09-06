use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use std::collections::BTreeMap;

/// Assert the finite draft's complete causal surface, including the instance and announced values.
/// These are compiler-IR controls, not execution of an installation runtime.
pub fn check(ir: &Value) -> Result<()> {
    let commands = ir["commands"].as_object().context("commands object")?;
    ensure!(
        commands.len() == 18,
        "expected eighteen activation/upgrade/removal commands, got {}",
        commands.len()
    );
    ensure!(
        ir["events"].as_object().is_some_and(|v| v.len() == 18),
        "expected eighteen typed events"
    );
    let mut count = 0;
    let mut expected_commands = Vec::new();
    let mut expected_events = Vec::new();
    let mut expected_transitions: BTreeMap<&str, Vec<Value>> = BTreeMap::new();
    for (name, entity, identity, transition, from, to, event, attempt_type, extra) in [
        (
            "BeginActivation",
            "Installation",
            "installation_id",
            "begin-activation",
            vec!["ActivationFailed", "Recorded"],
            "Activating",
            "ActivationBegun",
            "ActivationAttempt",
            None,
        ),
        (
            "ConfirmActivation",
            "Installation",
            "installation_id",
            "confirm-active",
            vec!["Activating"],
            "Active",
            "InstallationActivated",
            "ActivationAttempt",
            None,
        ),
        (
            "FailActivation",
            "Installation",
            "installation_id",
            "fail-activation",
            vec!["Activating"],
            "ActivationFailed",
            "ActivationFailed",
            "ActivationAttempt",
            Some(("failure", "FailureEvidence")),
        ),
        (
            "BeginRegistration",
            "ContributionRegistration",
            "registration_id",
            "begin-registration",
            vec!["Recorded", "RegistrationFailed"],
            "Registering",
            "RegistrationBegun",
            "RegistrationAttempt",
            None,
        ),
        (
            "ConfirmRegistration",
            "ContributionRegistration",
            "registration_id",
            "confirm-ready",
            vec!["Registering"],
            "Ready",
            "ContributionReady",
            "RegistrationAttempt",
            Some(("receipt", "OwnerReceiptRef")),
        ),
        (
            "FailRegistration",
            "ContributionRegistration",
            "registration_id",
            "fail-registration",
            vec!["Registering"],
            "RegistrationFailed",
            "RegistrationFailed",
            "RegistrationAttempt",
            Some(("failure", "FailureEvidence")),
        ),
        (
            "BeginUpgrade",
            "Installation",
            "installation_id",
            "begin-upgrade",
            vec!["Active", "UpgradeFailed"],
            "Upgrading",
            "UpgradeBegun",
            "UpgradeAttempt",
            None,
        ),
        (
            "ConfirmUpgrade",
            "Installation",
            "installation_id",
            "confirm-upgrade",
            vec!["Upgrading"],
            "Active",
            "InstallationUpgraded",
            "UpgradeAttempt",
            Some(("verification", "UpgradeVerification")),
        ),
        (
            "FailUpgrade",
            "Installation",
            "installation_id",
            "fail-upgrade",
            vec!["Upgrading"],
            "UpgradeFailed",
            "UpgradeFailed",
            "UpgradeAttempt",
            Some(("failure", "FailureEvidence")),
        ),
        (
            "BeginRemoval",
            "Installation",
            "installation_id",
            "begin-removal",
            vec![
                "ActivationFailed",
                "Active",
                "Recorded",
                "RemovalFailed",
                "UpgradeFailed",
            ],
            "Removing",
            "RemovalBegun",
            "RemovalAttempt",
            None,
        ),
        (
            "ConfirmRemoval",
            "Installation",
            "installation_id",
            "confirm-removal",
            vec!["Removing"],
            "RemovedRetained",
            "InstallationRemoved",
            "RemovalAttempt",
            Some(("confirmation", "RemovalConfirmation")),
        ),
        (
            "FailRemoval",
            "Installation",
            "installation_id",
            "fail-removal",
            vec!["Removing"],
            "RemovalFailed",
            "RemovalFailed",
            "RemovalAttempt",
            Some(("failure", "FailureEvidence")),
        ),
        (
            "BeginDetachment",
            "ContributionRegistration",
            "registration_id",
            "begin-detachment",
            vec![
                "DetachmentFailed",
                "Ready",
                "Recorded",
                "RegistrationFailed",
            ],
            "Detaching",
            "DetachmentBegun",
            "DetachmentAttempt",
            None,
        ),
        (
            "ConfirmDetachment",
            "ContributionRegistration",
            "registration_id",
            "confirm-detachment",
            vec!["Detaching"],
            "Detached",
            "RegistrationDetached",
            "DetachmentAttempt",
            Some(("result", "ControlDetachmentResult")),
        ),
        (
            "FailDetachment",
            "ContributionRegistration",
            "registration_id",
            "fail-detachment",
            vec!["Detaching"],
            "DetachmentFailed",
            "DetachmentFailed",
            "DetachmentAttempt",
            Some(("failure", "FailureEvidence")),
        ),
        (
            "BeginDataDestruction",
            "Installation",
            "installation_id",
            "begin-data-destruction",
            vec!["DestructionFailed", "RemovedRetained"],
            "Destroying",
            "DataDestructionBegun",
            "DataDestructionAttempt",
            None,
        ),
        (
            "ConfirmDataDestruction",
            "Installation",
            "installation_id",
            "confirm-data-destruction",
            vec!["Destroying"],
            "DataScopeDestroyed",
            "DataScopeDestroyed",
            "DataDestructionAttempt",
            Some(("confirmation", "DataDestructionConfirmation")),
        ),
        (
            "FailDataDestruction",
            "Installation",
            "installation_id",
            "fail-data-destruction",
            vec!["Destroying"],
            "DestructionFailed",
            "DataDestructionFailed",
            "DataDestructionAttempt",
            Some(("failure", "FailureEvidence")),
        ),
    ] {
        let qualified = |s: &str| format!("extensions.control.{s}");
        expected_commands.push(qualified(name));
        expected_events.push(qualified(event));
        expected_transitions
            .entry(entity)
            .or_default()
            .push(json!({"name":transition,"from":from,"to":to}));
        let command = &commands[&qualified(name)];
        let outcomes = command["outcomes"].as_array().context("outcomes")?;
        ensure!(
            outcomes.len() == 3,
            "{name}: expected accepted, refused and wrong-state outcomes"
        );
        let accepted = &outcomes[0];
        ensure!(
            accepted["name"] == "accepted" && accepted["condition"]["kind"] == "otherwise",
            "{name}: accepted branch"
        );
        let subject = &accepted["subject"];
        ensure!(
            subject["entity"] == qualified(entity) && subject["effect"] == "moves",
            "{name}: entity move"
        );
        ensure!(
            subject["transition"] == json!({"name":transition,"from":from,"to":to}),
            "{name}: exact transition"
        );
        ensure!(
            subject["instance"]["from"] == "supplied"
                && subject["instance"]["field"]["name"] == identity
                && subject["instance"]["field"]["type_ref"]
                    == json!({"kind":"declared", "name":qualified(if entity == "Installation" {"InstallationId"} else {"RegistrationId"})}),
            "{name}: supplied instance carrier"
        );
        ensure!(
            accepted["emits"] == json!([qualified(event)]) && accepted.get("error").is_none(),
            "{name}: emitted event"
        );
        let mut fields = vec![
            (
                identity,
                if entity == "Installation" {
                    "InstallationId"
                } else {
                    "RegistrationId"
                },
            ),
            ("attempt", attempt_type),
        ];
        if entity == "ContributionRegistration" {
            fields.push(("owner_registration_key", "OwnerRegistrationKey"));
        }
        if let Some(field) = extra {
            fields.push(field);
        }
        let input = command["input"].as_array().context("input")?;
        let event_fields = ir["events"][qualified(event)]["fields"]
            .as_array()
            .context("event fields")?;
        let mappings = accepted["payload"].as_array().context("payload")?;
        ensure!(
            mappings.len() == 1 && mappings[0]["event"] == qualified(event),
            "{name}: payload event"
        );
        let mapped = mappings[0]["fields"].as_array().context("mapped fields")?;
        ensure!(
            input.len() == fields.len()
                && event_fields.len() == fields.len()
                && mapped.len() == fields.len(),
            "{name}: complete typed input/event mapping"
        );
        for (field, ty) in fields {
            let expected = json!({"kind":"declared", "name":qualified(ty)});
            for surface in [input, event_fields] {
                ensure!(
                    surface
                        .iter()
                        .any(|f| f["name"] == field && f["type_ref"] == expected),
                    "{name}: typed {field}"
                );
            }
            ensure!(
                mapped.iter().any(|f| f["target"] == field
                    && f["target_type"] == expected
                    && f["value"]
                        == json!({"kind":"input_field","field":field,"type_ref":expected})),
                "{name}: mapping {field}"
            );
        }
        for (outcome, outcome_name, condition, error) in [
            (&outcomes[1], "refused", "external", "ReconciliationRefused"),
            (
                &outcomes[2],
                "wrong-state",
                "wrong_state",
                if entity == "Installation" {
                    "InstallationStateConflict"
                } else {
                    "RegistrationStateConflict"
                },
            ),
        ] {
            ensure!(
                outcome["name"] == outcome_name
                    && outcome["condition"]["kind"] == condition
                    && outcome["error"] == qualified(error)
                    && outcome.get("subject").is_none()
                    && outcome["emits"] == json!([])
                    && outcome.get("payload").is_none(),
                "{name}: typed non-mutating {condition} refusal"
            );
        }
        count += 1;
    }
    expected_commands.sort();
    expected_events.sort();
    let components = ir["components"].as_object().context("components")?;
    ensure!(components.len() == 1, "one owning component");
    let component = &components["extensions"];
    ensure!(
        component["owns"] == json!(["extensions.control"])
            && component["accepts"] == json!(expected_commands)
            && component["publishes"] == json!(expected_events),
        "exact owned domain and component command/event surface"
    );
    for name in ["Extension", "Release", "ContributionDeclaration"] {
        ensure!(
            ir["entities"][format!("extensions.control.{name}")]["lifecycle"]
                == json!({"initial":"Recorded","states":["Recorded"],"terminal":["Recorded"]}),
            "{name}: immutable Recorded lifecycle"
        );
    }
    for (entity, relations) in [
        (
            "Extension",
            json!([{"name":"releases","kind":"owns","target":"extensions.control.Release","cardinality":"many","via":"extension_id"}]),
        ),
        (
            "Release",
            json!([{"name":"contributions","kind":"owns","target":"extensions.control.ContributionDeclaration","cardinality":"many","via":"release_id"}]),
        ),
        ("ContributionDeclaration", Value::Null),
        (
            "Installation",
            json!([
                {"name":"release","kind":"references","target":"extensions.control.Release","cardinality":"one","via":"release_id"},
                {"name":"registrations","kind":"owns","target":"extensions.control.ContributionRegistration","cardinality":"many","via":"installation_id"}
            ]),
        ),
        (
            "ContributionRegistration",
            json!([{"name":"declaration","kind":"references","target":"extensions.control.ContributionDeclaration","cardinality":"one","via":"contribution_id"}]),
        ),
    ] {
        ensure!(
            ir["entities"][format!("extensions.control.{entity}")]["relations"] == relations,
            "{entity}: exact existing relations"
        );
    }
    for (entity, states, terminal, transitions) in [
        (
            "Installation",
            json!([
                "Activating",
                "ActivationFailed",
                "Active",
                "DataScopeDestroyed",
                "Destroying",
                "DestructionFailed",
                "Recorded",
                "RemovalFailed",
                "RemovedRetained",
                "Removing",
                "UpgradeFailed",
                "Upgrading"
            ]),
            "DataScopeDestroyed",
            12,
        ),
        (
            "ContributionRegistration",
            json!([
                "Detached",
                "Detaching",
                "DetachmentFailed",
                "Ready",
                "Recorded",
                "Registering",
                "RegistrationFailed"
            ]),
            "Detached",
            6,
        ),
    ] {
        let lifecycle = &ir["entities"][format!("extensions.control.{entity}")]["lifecycle"];
        ensure!(
            lifecycle["initial"] == "Recorded"
                && lifecycle["states"] == states
                && lifecycle["terminal"] == json!([terminal])
                && lifecycle["transitions"] == json!(expected_transitions[entity])
                && lifecycle["transitions"]
                    .as_array()
                    .is_some_and(|v| v.len() == transitions),
            "{entity}: finite lifecycle surface"
        );
    }
    println!(
        "causal controls: {count} commands with exact transitions, instances, payloads and refusals"
    );
    Ok(())
}
