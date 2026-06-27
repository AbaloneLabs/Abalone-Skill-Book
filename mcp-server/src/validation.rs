use std::collections::HashMap;
use std::path::PathBuf;

use serde::Serialize;

use crate::skill::{word_count, SkillDocument, REQUIRED_HEADINGS};

pub const MIN_SUBSTANTIVE_WORDS: usize = 800;
pub const LARGE_SKILL_WORDS: usize = 3500;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationReport {
    pub ok: bool,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    pub rules: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationIssue {
    pub code: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub message: String,
    pub rule: &'static str,
    pub fix: String,
}

impl ValidationReport {
    pub fn ok() -> Self {
        Self {
            ok: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            rules: authoring_rules(),
        }
    }

    pub fn from_errors(errors: Vec<ValidationIssue>) -> Self {
        Self {
            ok: errors.is_empty(),
            errors,
            warnings: Vec::new(),
            rules: authoring_rules(),
        }
    }
}

pub fn authoring_rules() -> Vec<&'static str> {
    vec![
        "Use YAML frontmatter with name and trigger-oriented description.",
        "Use exact required headings: ## Core Rules, ## Common Traps, ## Self-Check.",
        "Write a substantive guidance document, normally A4 2-5 pages.",
        "Merge tiny fragments into an existing broader skill.",
    ]
}

pub fn validate_skill_source(path: &str, source: &str) -> ValidationReport {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    let normalized_path = match normalize_skill_path(path) {
        Ok(path) => Some(path),
        Err(issue) => {
            errors.push(issue);
            None
        }
    };

    let Some(logical_path) = normalized_path else {
        return ValidationReport {
            ok: false,
            errors,
            warnings,
            rules: authoring_rules(),
        };
    };

    let document = match SkillDocument::parse(
        logical_path.clone(),
        PathBuf::from(format!("skills/{logical_path}/SKILL.md")),
        source.to_string(),
    ) {
        Ok(document) => document,
        Err(err) => {
            errors.push(issue(
                "invalid_skill_markdown",
                Some(logical_path),
                format!("Invalid SKILL.md structure: {err}"),
                "Every SKILL.md must start with YAML frontmatter bounded by --- and contain a Markdown body.",
                "Rewrite the file with YAML frontmatter containing name and description, then add the required Markdown sections.",
            ));
            return ValidationReport {
                ok: false,
                errors,
                warnings,
                rules: authoring_rules(),
            };
        }
    };

    validate_frontmatter(&document, &mut errors);
    validate_sections(&document, &mut errors);
    validate_size_and_granularity(&document, &mut errors, &mut warnings);

    ValidationReport {
        ok: errors.is_empty(),
        errors,
        warnings,
        rules: authoring_rules(),
    }
}

pub fn normalize_skill_path(path: &str) -> Result<String, ValidationIssue> {
    let normalized = normalize_scope_path(path)?;
    let segments: Vec<String> = normalized.split('/').map(str::to_string).collect();
    if segments.len() < 4 {
        return Err(issue(
            "path_too_shallow",
            Some(normalized),
            "Skill path must contain at least role, area, subject, and skill-name.".to_string(),
            "Skill paths must follow skills/<role>/<area>/<subject>/[optional-context...]/<skill-name>/SKILL.md.",
            "Move the skill under a path such as programmer/api/response/data-minimization.",
        ));
    }

    Ok(normalized)
}

pub fn normalize_scope_path(path: &str) -> Result<String, ValidationIssue> {
    let original = path.trim();
    if original.is_empty() {
        return Err(issue(
            "invalid_path",
            None,
            "Scope path is empty.".to_string(),
            "Scope paths must be slash-separated path prefixes under skills/.",
            "Provide a scope such as investor, investor/analyze, or programmer/api/response.",
        ));
    }

    if original.contains('\\') {
        return Err(issue(
            "invalid_path_separator",
            Some(original.to_string()),
            "Path contains backslashes.".to_string(),
            "Skill and scope paths must use forward slashes.",
            "Replace backslashes with forward slashes.",
        ));
    }

    let mut normalized = original.trim_matches('/').to_string();
    if let Some(rest) = normalized.strip_prefix("skills/") {
        normalized = rest.to_string();
    }
    if let Some(rest) = normalized.strip_suffix("/SKILL.md") {
        normalized = rest.to_string();
    }

    let segments: Vec<String> = normalized.split('/').map(str::to_string).collect();

    for segment in &segments {
        if segment.is_empty() || segment == "." || segment == ".." {
            return Err(issue(
                "invalid_path_segment",
                Some(normalized.clone()),
                format!("Invalid path segment: {segment}"),
                "Path segments must not be empty, '.', or '..'.",
                "Use stable lowercase path segments such as api, response, or data-minimization.",
            ));
        }

        if !segment
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
        {
            return Err(issue(
                "invalid_path_segment_chars",
                Some(normalized.clone()),
                format!("Path segment contains unsupported characters: {segment}"),
                "Path segments may contain only ASCII letters, digits, hyphen, and underscore.",
                "Rename the segment using ASCII letters, digits, hyphen, or underscore.",
            ));
        }
    }

    Ok(normalized)
}

fn validate_frontmatter(document: &SkillDocument, errors: &mut Vec<ValidationIssue>) {
    if document.frontmatter.name.trim().is_empty() {
        errors.push(issue(
            "missing_name",
            Some(document.path.clone()),
            "Frontmatter field `name` is empty.".to_string(),
            "Every SKILL.md frontmatter must include a stable non-empty name.",
            "Set name to a stable identifier such as market_analysis.md.",
        ));
    }

    let description = document.frontmatter.description.trim();
    if description.is_empty() {
        errors.push(issue(
            "missing_description",
            Some(document.path.clone()),
            "Frontmatter field `description` is empty.".to_string(),
            "Every SKILL.md frontmatter must include trigger-oriented description text.",
            "Write a description starting with when the agent should use the skill.",
        ));
    } else if !description_looks_trigger_oriented(description) {
        errors.push(issue(
            "weak_description",
            Some(document.path.clone()),
            "Description does not look trigger-oriented enough for recommendation.".to_string(),
            "description must explain when the agent should use the skill, not just label the topic.",
            "Rewrite description as a trigger sentence, for example: Use when the agent is designing an API response and must separate public fields, private fields, authorization boundaries, and error behavior.",
        ));
    }
}

fn validate_sections(document: &SkillDocument, errors: &mut Vec<ValidationIssue>) {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for section in &document.sections {
        for required in REQUIRED_HEADINGS {
            if section.heading == required {
                *counts.entry(required).or_default() += 1;
            } else if looks_like_required_variant(&section.heading, required) {
                errors.push(issue(
                    "non_exact_required_heading",
                    Some(document.path.clone()),
                    format!(
                        "Section heading `## {}` looks like `{}` but is not exact.",
                        section.heading, required
                    ),
                    "Required section headings must be spelled exactly: ## Core Rules, ## Common Traps, ## Self-Check.",
                    format!("Rename `## {}` to `## {required}`.", section.heading),
                ));
            }
        }
    }

    for required in REQUIRED_HEADINGS {
        match counts.get(required).copied().unwrap_or_default() {
            0 => errors.push(issue(
                "missing_required_section",
                Some(document.path.clone()),
                format!("Missing required section: ## {required}"),
                "Every SKILL.md must include exact headings: ## Core Rules, ## Common Traps, ## Self-Check.",
                format!("Add a ## {required} section with substantive guidance."),
            )),
            1 => {}
            count => errors.push(issue(
                "duplicate_required_section",
                Some(document.path.clone()),
                format!("Required section appears {count} times: ## {required}"),
                "Each required section heading must appear exactly once.",
                format!("Merge duplicate ## {required} sections into one section."),
            )),
        }
    }
}

fn validate_size_and_granularity(
    document: &SkillDocument,
    errors: &mut Vec<ValidationIssue>,
    warnings: &mut Vec<ValidationIssue>,
) {
    let words = word_count(&document.body);
    if words < MIN_SUBSTANTIVE_WORDS {
        errors.push(issue(
            "skill_too_short",
            Some(document.path.clone()),
            format!(
                "Skill body has {words} substantive words; minimum is {MIN_SUBSTANTIVE_WORDS}."
            ),
            "A standalone skill should be a substantive guidance document, normally A4 2-5 pages.",
            "Expand the skill with real rules, traps, edge cases, tradeoffs, examples, and self-checks, or merge it into a broader skill.",
        ));
    }

    let checklist_lines = document
        .body
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with("- [ ]")
                || trimmed.starts_with("- [x]")
                || trimmed.starts_with("* [ ]")
        })
        .count();
    if words < MIN_SUBSTANTIVE_WORDS && checklist_lines >= 3 {
        errors.push(issue(
            "tiny_checklist_fragment",
            Some(document.path.clone()),
            "Skill appears to be a tiny checklist fragment.".to_string(),
            "Tiny checklist fragments should be merged into a broader skill instead of created as standalone skills.",
            "Move these checklist items into the Self-Check section of a broader related skill.",
        ));
    }

    if words > LARGE_SKILL_WORDS {
        warnings.push(issue(
            "skill_may_be_too_large",
            Some(document.path.clone()),
            format!("Skill body has {words} words and may be hard to read as one document."),
            "Oversized skills should be split by distinct work intent.",
            "Consider splitting the skill if it covers multiple distinct work intents.",
        ));
    }
}

fn description_looks_trigger_oriented(description: &str) -> bool {
    let lower = description.to_ascii_lowercase();
    word_count(description) >= 12
        && (lower.contains("use when")
            || lower.contains("when the agent")
            || lower.contains("before")
            || lower.contains("while")
            || lower.contains("where "))
}

fn looks_like_required_variant(heading: &str, required: &str) -> bool {
    if heading == required {
        return false;
    }

    let normalized_heading = normalize_heading_for_comparison(heading);
    let normalized_required = normalize_heading_for_comparison(required);

    normalized_heading == normalized_required
        || normalized_heading.starts_with(&normalized_required)
        || (required == "Self-Check" && normalized_heading == "checklist")
}

fn normalize_heading_for_comparison(heading: &str) -> String {
    heading
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn issue(
    code: &'static str,
    path: Option<String>,
    message: String,
    rule: &'static str,
    fix: impl Into<String>,
) -> ValidationIssue {
    ValidationIssue {
        code,
        path,
        message,
        rule,
        fix: fix.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_skill_path, validate_skill_source, MIN_SUBSTANTIVE_WORDS};

    #[test]
    fn normalizes_supported_path_shapes() {
        assert_eq!(
            normalize_skill_path("skills/investor/analyze/market/market-analysis/SKILL.md")
                .unwrap(),
            "investor/analyze/market/market-analysis"
        );
        assert_eq!(
            normalize_skill_path("investor/analyze/market/market-analysis").unwrap(),
            "investor/analyze/market/market-analysis"
        );
    }

    #[test]
    fn rejects_shallow_paths() {
        let err = normalize_skill_path("investor/market").unwrap_err();
        assert_eq!(err.code, "path_too_shallow");
    }

    #[test]
    fn market_analysis_skill_is_valid() {
        let source = include_str!("../../skills/investor/analyze/market/market-analysis/SKILL.md");
        let report = validate_skill_source("investor/analyze/market/market-analysis", source);
        assert!(
            report.ok,
            "expected market-analysis to validate, got {:#?}",
            report.errors
        );
    }

    #[test]
    fn rejects_non_exact_self_check_heading() {
        let source = format!(
            "---
name: api_response.md
description: Use when the agent is designing an API response and must consider public data, private data, authorization boundaries, and error behavior.
---

# API Response

## Core Rules

{}

## Common Traps

{}

## Self Check

- [ ] Check one
",
            "word ".repeat(MIN_SUBSTANTIVE_WORDS / 2),
            "word ".repeat(MIN_SUBSTANTIVE_WORDS / 2),
        );

        let report = validate_skill_source("programmer/api/response/data-minimization", &source);
        assert!(!report.ok);
        assert!(report
            .errors
            .iter()
            .any(|issue| issue.code == "non_exact_required_heading"));
        assert!(report
            .errors
            .iter()
            .any(|issue| issue.code == "missing_required_section"));
    }

    #[test]
    fn rejects_tiny_checklist_fragments() {
        let source = "---
name: tiny.md
description: Use when the agent is checking API response fields before exposing data to a caller.
---

# Tiny

## Core Rules

Keep fields safe.

## Common Traps

Do not leak private fields.

## Self-Check

- [ ] One
- [ ] Two
- [ ] Three
";
        let report = validate_skill_source("programmer/api/response/tiny", source);
        assert!(!report.ok);
        assert!(report
            .errors
            .iter()
            .any(|issue| issue.code == "skill_too_short"));
        assert!(report
            .errors
            .iter()
            .any(|issue| issue.code == "tiny_checklist_fragment"));
    }
}
