use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};

/// Assert the finite draft's complete causal surface, including the instance and announced values.
/// These are compiler-IR controls, not execution of an installation runtime.
pub fn check(ir: &Value) -> Result<()> {
    let commands = ir["commands"].as_object().context("commands object")?;
    ensure!(
        commands.len() == 6,
        "expected six activation/recovery commands, got {}",
        commands.len()
    );
    ensure!(
        ir["events"].as_object().is_some_and(|v| v.len() == 6),
        "expected six typed events"
    );
    let mut count = 0;
    for (name, entity, identity, transition, from, to, event, extra) in [
        (
            "BeginActivation",
            "Installation",
            "installation_id",
            "begin-activation",
            vec!["ActivationFailed", "Recorded"],
            "Activating",
            "ActivationBegun",
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
            Some(("failure", "FailureEvidence")),
        ),
    ] {
        let qualified = |s: &str| format!("extensions.control.{s}");
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
                && subject["instance"]["field"]["name"] == identity,
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
            (
                "attempt",
                if entity == "Installation" {
                    "ActivationAttempt"
                } else {
                    "RegistrationAttempt"
                },
            ),
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
        for (outcome, condition, error) in [
            (&outcomes[1], "external", "ReconciliationRefused"),
            (
                &outcomes[2],
                "wrong_state",
                if entity == "Installation" {
                    "InstallationStateConflict"
                } else {
                    "RegistrationStateConflict"
                },
            ),
        ] {
            ensure!(
                outcome["condition"]["kind"] == condition
                    && outcome["error"] == qualified(error)
                    && outcome.get("subject").is_none()
                    && outcome["emits"] == json!([]),
                "{name}: typed non-mutating {condition} refusal"
            );
        }
        count += 1;
    }
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
    for (entity, states, terminal) in [
        (
            "Installation",
            json!(["Activating", "ActivationFailed", "Active", "Recorded"]),
            "Active",
        ),
        (
            "ContributionRegistration",
            json!(["Ready", "Recorded", "Registering", "RegistrationFailed"]),
            "Ready",
        ),
    ] {
        let lifecycle = &ir["entities"][format!("extensions.control.{entity}")]["lifecycle"];
        ensure!(
            lifecycle["initial"] == "Recorded"
                && lifecycle["states"] == states
                && lifecycle["terminal"] == json!([terminal])
                && lifecycle["transitions"]
                    .as_array()
                    .is_some_and(|v| v.len() == 3),
            "{entity}: finite lifecycle surface"
        );
    }
    println!(
        "causal controls: {count} commands with exact transitions, instances, payloads and refusals"
    );
    Ok(())
}
