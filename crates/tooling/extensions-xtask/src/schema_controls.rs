use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Cases {
    schema: String,
    valid: Vec<Value>,
    invalid: Vec<Value>,
}

/// Validate examples against ESS-owned projections, never a second hand-written schema.
/// HTTP/file resolution is disabled in the dependency; these projections are self-contained.
pub fn check(root: &Path) -> Result<()> {
    let cases: Vec<Cases> =
        serde_json::from_slice(&fs::read(root.join("fixtures/schema-cases.json"))?)?;
    ensure!(!cases.is_empty(), "missing schema controls");
    let mut accepted = 0;
    let mut refused = 0;
    for case in cases {
        ensure!(
            !case.valid.is_empty() && !case.invalid.is_empty(),
            "{} needs a positive control and a refusal",
            case.schema
        );
        let schema: Value =
            serde_json::from_slice(&fs::read(root.join("contracts/schema").join(&case.schema))?)?;
        let validator = jsonschema::options()
            .with_draft(jsonschema::Draft::Draft202012)
            .build(&schema)
            .with_context(|| format!("compile {}", case.schema))?;
        for value in case.valid {
            let errors: Vec<_> = validator
                .iter_errors(&value)
                .map(|e| e.to_string())
                .collect();
            ensure!(errors.is_empty(), "{}: {errors:?}", case.schema);
            accepted += 1;
        }
        for value in case.invalid {
            ensure!(
                !validator.is_valid(&value),
                "{} accepted a refusal control: {value}",
                case.schema
            );
            refused += 1;
        }
    }
    // ESS 0.9.2 projects these predicates as annotations, not JSON Schema assertions.
    // Keep that boundary executable: a passing schema check must not be sold as admission.
    let mut gaps = 0;
    for (name, predicate, invalid_domain_value) in [
        ("ConfigurationRevision", "value > 0", 0),
        ("InstallationCount", "value >= 0", -1),
        ("BindingRevision", "value > 0", 0),
        ("ReconciliationGeneration", "value > 0", 0),
        ("RetryAttempt", "value > 0", 0),
    ] {
        let qualified = format!("extensions.control.{name}");
        let schema: Value = serde_json::from_slice(&fs::read(
            root.join(format!("contracts/schema/types/{qualified}.schema.json")),
        )?)?;
        ensure!(
            schema["$defs"][&qualified]["x-ess-invariants"]
                .as_array()
                .is_some_and(|items| items.contains(&Value::String(predicate.into()))),
            "{name} lost its ESS invariant annotation"
        );
        let validator = jsonschema::validator_for(&schema)?;
        ensure!(
            validator.is_valid(&Value::from(invalid_domain_value)),
            "ESS invariant projection changed; reassess the documented admission boundary"
        );
        gaps += 1;
    }
    println!(
        "schema controls: {accepted} accepted, {refused} refused; {gaps} invariant annotation gaps confirmed"
    );
    Ok(())
}
