import { appendFile, mkdir } from "node:fs/promises";
import { join } from "node:path";
import { randomUUID } from "node:crypto";

/** Absolute path to the feedback storage directory (outside the skills tree). */
const FEEDBACK_DIR = join(import.meta.dirname, "..", "..", ".feedback");

export interface Feedback {
  id: string;
  skillPath: string;
  content: string;
  submittedAt: number;
}

/**
 * Append a feedback entry to the feedback queue.
 *
 * Feedback is stored as newline-delimited JSON in a single file per day.
 * A local model or human reviewer consumes this queue to decide what to
 * merge back into the skills.
 */
export async function submitFeedback(
  skillPath: string,
  content: string
): Promise<Feedback> {
  const feedback: Feedback = {
    id: randomUUID(),
    skillPath,
    content,
    submittedAt: Date.now(),
  };

  await mkdir(FEEDBACK_DIR, { recursive: true });

  const date = new Date().toISOString().slice(0, 10); // YYYY-MM-DD
  const file = join(FEEDBACK_DIR, `${date}.jsonl`);
  await appendFile(file, JSON.stringify(feedback) + "\n", "utf-8");

  return feedback;
}
