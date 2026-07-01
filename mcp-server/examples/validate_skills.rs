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

    for file in &targets {
        total += 1;
        let rel = file.strip_prefix(&skills_root).unwrap_or(file);
        let logical = rel
            .to_string_lossy()
            .trim_end_matches("/SKILL.md")
            .trim_start_matches("skills/")
            .to_string();

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
