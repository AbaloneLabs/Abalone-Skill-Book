use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use abalone_mcp_server::validation::validate_skill_source;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let skills_root: PathBuf = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("skills")
    };

    let mut targets: Vec<PathBuf> = Vec::new();
    collect_skills(&skills_root, &mut targets);

    if targets.is_empty() {
        eprintln!("No SKILL.md files found under {}", skills_root.display());
        std::process::exit(1);
    }

    targets.sort();
    let mut total = 0usize;
    let mut failures = 0usize;
    // Track skill count per area (role/area) for over-fragmentation detection.
    let mut area_counts: BTreeMap<String, usize> = BTreeMap::new();

    for file in &targets {
        total += 1;
        let rel = file.strip_prefix(&skills_root).unwrap_or(file);
        let logical = rel
            .to_string_lossy()
            .trim_end_matches("/SKILL.md")
            .trim_start_matches("skills/")
            .to_string();

        // Aggregate by area path (role/area). C-plan depth is role/area/skill-name,
        // so the area is the first two segments.
        let area_key = logical
            .split('/')
            .take(2)
            .collect::<Vec<&str>>()
            .join("/");
        *area_counts.entry(area_key).or_insert(0) += 1;

        let source = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => {
                println!("FAIL  {}  (read error: {e})", logical);
                failures += 1;
                continue;
            }
        };

        let report = validate_skill_source(&logical, &source);
        if report.ok {
            let warns = report.warnings.len();
            println!(
                "OK    {}  (warnings: {warns})",
                logical
            );
            for w in &report.warnings {
                println!("        warn [{}]: {}", w.code, w.message);
            }
        } else {
            failures += 1;
            println!("FAIL  {}", logical);
            for e in &report.errors {
                println!("        error [{}]: {}", e.code, e.message);
                println!("          fix: {}", e.fix);
            }
        }
    }

    println!("\nValidated {total} skill(s), {failures} failure(s).");

    // Over-fragmentation report: areas with too few skills indicate that
    // related skills were split into separate area folders instead of being
    // grouped under a single area (anti-pattern F-2). An area with only one
    // skill is usually a sign the skill should belong to a broader area.
    let single_skill_areas: Vec<(&String, &usize)> =
        area_counts.iter().filter(|(_, &c)| c == 1).collect();
    let total_areas = area_counts.len();
    if !single_skill_areas.is_empty() {
        let pct = if total_areas > 0 {
            single_skill_areas.len() * 100 / total_areas
        } else {
            0
        };
        println!(
            "\nOver-fragmentation warning: {}/{} areas ({pct}%) have only 1 skill.",
            single_skill_areas.len(),
            total_areas
        );
        println!("        These areas likely split a concept that should group multiple skills.");
        println!("        Review whether these single-skill areas should be merged into a broader area.");
        for (area, _) in &single_skill_areas {
            println!("        1-skill area: {}", area);
        }
    }

    if failures > 0 {
        std::process::exit(1);
    }
}

fn collect_skills(dir: &PathBuf, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_skills(&path, out);
        } else if path.file_name().map(|n| n == "SKILL.md").unwrap_or(false) {
            out.push(path);
        }
    }
}
