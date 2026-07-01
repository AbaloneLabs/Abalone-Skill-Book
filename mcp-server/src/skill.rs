use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

pub const REQUIRED_HEADINGS: [&str; 3] = ["Core Rules", "Common Traps", "Self-Check"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: String,
    #[serde(default, alias = "disable-model-invocation")]
    pub disable_model_invocation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillDocument {
    pub path: String,
    pub file_path: PathBuf,
    pub source: String,
    pub frontmatter: SkillFrontmatter,
    pub body: String,
    pub sections: Vec<MarkdownSection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SkillSummary {
    pub path: String,
    pub name: String,
    pub description: String,
    pub disable_model_invocation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownSection {
    pub heading: String,
    pub content: String,
    pub start_line: usize,
}

impl SkillDocument {
    pub fn parse(path: String, file_path: PathBuf, source: String) -> Result<Self> {
        let (frontmatter_source, body) = split_frontmatter(&source)?;
        let frontmatter = serde_yaml::from_str::<SkillFrontmatter>(frontmatter_source)
            .context("failed to parse SKILL.md YAML frontmatter")?;
        let body = body.to_string();
        let sections = extract_h2_sections(&body);

        Ok(Self {
            path,
            file_path,
            source,
            frontmatter,
            body,
            sections,
        })
    }

    pub fn summary(&self) -> SkillSummary {
        SkillSummary {
            path: self.path.clone(),
            name: self.frontmatter.name.clone(),
            description: self.frontmatter.description.clone(),
            disable_model_invocation: self.frontmatter.disable_model_invocation,
        }
    }

    pub fn embedding_text(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.path, self.frontmatter.name, self.frontmatter.description
        )
    }

    pub fn section(&self, heading: &str) -> Option<&MarkdownSection> {
        self.sections
            .iter()
            .find(|section| section.heading == heading)
    }
}

pub fn split_frontmatter(source: &str) -> Result<(&str, &str)> {
    let mut lines = source.lines();
    let first = lines.next().unwrap_or_default().trim();
    if first != "---" {
        bail!("SKILL.md must start with YAML frontmatter bounded by ---");
    }

    let mut frontmatter_end_byte = None;
    let mut cursor = source.find('\n').map(|idx| idx + 1).unwrap_or(source.len());

    for line in lines {
        let line_start = cursor;
        let line_end = line_start + line.len();
        let newline_len = source[line_end..]
            .chars()
            .next()
            .map(|ch| ch.len_utf8())
            .unwrap_or(0);

        if line.trim() == "---" {
            frontmatter_end_byte = Some((line_start, line_end + newline_len));
            break;
        }

        cursor = line_end + newline_len;
    }

    let Some((marker_start, body_start)) = frontmatter_end_byte else {
        bail!("SKILL.md frontmatter is missing closing --- marker");
    };

    let frontmatter_start = source.find('\n').map(|idx| idx + 1).unwrap_or(source.len());
    Ok((
        &source[frontmatter_start..marker_start],
        &source[body_start..],
    ))
}

pub fn extract_h2_sections(markdown: &str) -> Vec<MarkdownSection> {
    let mut sections = Vec::new();
    let mut current_heading: Option<String> = None;
    let mut current_start_line = 0;
    let mut current_content = Vec::new();

    for (idx, line) in markdown.lines().enumerate() {
        if let Some(heading) = h2_heading(line) {
            if let Some(previous_heading) = current_heading.replace(heading.to_string()) {
                sections.push(MarkdownSection {
                    heading: previous_heading,
                    content: current_content.join("\n").trim().to_string(),
                    start_line: current_start_line,
                });
                current_content.clear();
            }
            current_start_line = idx + 1;
        } else if current_heading.is_some() {
            current_content.push(line.to_string());
        }
    }

    if let Some(heading) = current_heading {
        sections.push(MarkdownSection {
            heading,
            content: current_content.join("\n").trim().to_string(),
            start_line: current_start_line,
        });
    }

    sections
}

fn h2_heading(line: &str) -> Option<&str> {
    let trimmed = line.trim_end();
    if let Some(rest) = trimmed.strip_prefix("## ") {
        if !rest.starts_with('#') {
            return Some(rest.trim());
        }
    }
    None
}

pub fn word_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| word.chars().any(char::is_alphanumeric))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{extract_h2_sections, split_frontmatter, SkillDocument};
    use std::path::PathBuf;

    #[test]
    fn parses_frontmatter_and_exact_h2_sections() {
        let source = include_str!("../../skills/investor/analyze/market-analysis/SKILL.md");
        let doc = SkillDocument::parse(
            "investor/analyze/market-analysis".to_string(),
            PathBuf::from("SKILL.md"),
            source.to_string(),
        )
        .expect("market analysis should parse");

        assert_eq!(doc.frontmatter.name, "market_analysis.md");
        assert!(doc.section("Core Rules").is_some());
        assert!(doc.section("Common Traps").is_some());
        assert!(doc.section("Self-Check").is_some());
    }

    #[test]
    fn split_frontmatter_requires_closing_marker() {
        let err = split_frontmatter("---\nname: broken\n# Body").unwrap_err();
        assert!(err.to_string().contains("closing"));
    }

    #[test]
    fn h2_extraction_ignores_h3_headings() {
        let sections = extract_h2_sections("## Core Rules\nx\n### Detail\ny\n## Self-Check\nz");
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].heading, "Core Rules");
        assert!(sections[0].content.contains("### Detail"));
    }
}
