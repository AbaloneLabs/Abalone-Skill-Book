import { readFile, writeFile, readdir, rm, mkdir } from "node:fs/promises";
import { existsSync } from "node:fs";
import type { Dirent } from "node:fs";
import { join, dirname } from "node:path";
import matter from "gray-matter";

/** Absolute path to the skills repository root (the `skills/` directory). */
const SKILLS_ROOT = join(import.meta.dirname, "..", "..", "skills");

const SKILL_FILENAME = "SKILL.md";

export interface SkillMeta {
  /** Repository-relative path, e.g. programmer/rust/core/ownership-and-borrowing */
  path: string;
  name: string;
  description: string;
}

export interface Skill extends SkillMeta {
  body: string;
}

/** Convert a repo-relative skill path into an absolute filesystem path. */
function skillPath(path: string): string {
  return join(SKILLS_ROOT, path, SKILL_FILENAME);
}

/** Recursively find all SKILL.md files under a given directory. */
async function findSkills(rootDir: string): Promise<string[]> {
  const results: string[] = [];
  if (!existsSync(rootDir)) return results;

  let entries: Dirent[];
  try {
    entries = await readdir(rootDir, { withFileTypes: true });
  } catch {
    return results;
  }

  for (const entry of entries) {
    if (entry.name.startsWith(".") || entry.name === "node_modules") continue;
    const fullPath = join(rootDir, entry.name);
    if (entry.isDirectory()) {
      const nested = await findSkills(fullPath);
      results.push(...nested);
    } else if (entry.name === SKILL_FILENAME) {
      results.push(fullPath);
    }
  }
  return results;
}

/** List skills, optionally filtered by a path prefix (e.g. "programmer/rust"). */
export async function listSkills(pathPrefix?: string): Promise<SkillMeta[]> {
  const searchRoot = pathPrefix ? join(SKILLS_ROOT, pathPrefix) : SKILLS_ROOT;
  const files = await findSkills(searchRoot);

  const skills: SkillMeta[] = [];
  for (const file of files) {
    const relPath = file
      .replace(SKILLS_ROOT + "/", "")
      .replace("/" + SKILL_FILENAME, "");
    try {
      const raw = await readFile(file, "utf-8");
      const { data } = matter(raw);
      skills.push({
        path: relPath,
        name: data.name ?? relPath.split("/").pop() ?? relPath,
        description: data.description ?? "",
      });
    } catch {
      // skip unreadable
    }
  }
  return skills.sort((a, b) => a.path.localeCompare(b.path));
}

/** Read a single skill's full content. */
export async function getSkill(path: string): Promise<Skill> {
  const file = skillPath(path);
  if (!existsSync(file)) {
    throw new Error(`Skill not found: ${path}`);
  }
  const raw = await readFile(file, "utf-8");
  const { data, content } = matter(raw);
  return {
    path,
    name: data.name ?? path.split("/").pop() ?? path,
    description: data.description ?? "",
    body: content,
  };
}

/** Create a new skill. */
export async function createSkill(
  path: string,
  name: string,
  description: string,
  body: string
): Promise<Skill> {
  const file = skillPath(path);
  if (existsSync(file)) {
    throw new Error(`Skill already exists: ${path}`);
  }
  await mkdir(dirname(file), { recursive: true });
  const content = matter.stringify(body, { name, description });
  await writeFile(file, content, "utf-8");
  return { path, name, description, body };
}

/** Update an existing skill's description and/or body. */
export async function updateSkill(
  path: string,
  changes: { description?: string; body?: string }
): Promise<Skill> {
  const file = skillPath(path);
  if (!existsSync(file)) {
    throw new Error(`Skill not found: ${path}`);
  }
  const raw = await readFile(file, "utf-8");
  const { data, content } = matter(raw);

  const newDescription = changes.description ?? data.description;
  const newBody = changes.body ?? content;

  const updated = matter.stringify(newBody, {
    name: data.name,
    description: newDescription,
  });
  await writeFile(file, updated, "utf-8");
  return {
    path,
    name: data.name,
    description: newDescription,
    body: newBody,
  };
}

/** Delete a skill. */
export async function deleteSkill(path: string): Promise<void> {
  const file = skillPath(path);
  if (!existsSync(file)) {
    throw new Error(`Skill not found: ${path}`);
  }
  const dir = dirname(file);
  await rm(dir, { recursive: true, force: true });
}

/** Validate that a path looks like a valid skill path. */
export function isValidPath(path: string): boolean {
  // Must have at least role/stack/domain/skill segments, no traversal.
  if (path.includes("..")) return false;
  const parts = path.split("/").filter(Boolean);
  if (parts.length < 4) return false;
  return parts.every((p) => /^[a-zA-Z0-9_-]+$/.test(p));
}

export interface SearchResult {
  path: string;
  name: string;
  description: string;
  /** Relevance score: how many times the query matched, weighted by field. */
  score: number;
}

/**
 * Search skills by keyword across name, description, path, and body.
 *
 * Weights reflect how strongly a field indicates relevance:
 *   name (10) > description (5) > path (3) > body (1)
 *
 * Each occurrence of the query (case-insensitive) adds to the score, so a
 * skill that mentions the term multiple times ranks higher.
 */
export async function searchSkills(query: string): Promise<SearchResult[]> {
  const q = query.trim().toLowerCase();
  if (!q) return [];

  const files = await findSkills(SKILLS_ROOT);
  const results: SearchResult[] = [];

  for (const file of files) {
    const relPath = file
      .replace(SKILLS_ROOT + "/", "")
      .replace("/" + SKILL_FILENAME, "");

    let raw: string;
    try {
      raw = await readFile(file, "utf-8");
    } catch {
      continue;
    }
    const { data, content } = matter(raw);

    const name = String(data.name ?? relPath.split("/").pop() ?? "");
    const description = String(data.description ?? "");

    const score =
      countMatches(name.toLowerCase(), q) * 10 +
      countMatches(description.toLowerCase(), q) * 5 +
      countMatches(relPath.toLowerCase(), q) * 3 +
      countMatches(content.toLowerCase(), q);

    if (score > 0) {
      results.push({ path: relPath, name, description, score });
    }
  }

  return results.sort((a, b) => b.score - a.score);
}

/** Count non-overlapping case-insensitive occurrences of `needle` in `haystack`. */
function countMatches(haystack: string, needle: string): number {
  if (!needle) return 0;
  let count = 0;
  let idx = haystack.indexOf(needle);
  while (idx !== -1) {
    count++;
    idx = haystack.indexOf(needle, idx + needle.length);
  }
  return count;
}
