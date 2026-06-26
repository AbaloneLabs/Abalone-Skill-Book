import { randomUUID } from "node:crypto";

/**
 * Session tracking with a 3-day TTL.
 *
 * An agent must read the usage guide at least once per session lifecycle.
 * After the TTL expires, the session is treated as "new" again and the
 * agent is asked to re-read the guide.
 */

const SESSION_TTL_MS = 3 * 24 * 60 * 60 * 1000; // 3 days

export interface SessionState {
  /** Random UUID identifying the session. */
  id: string;
  /** Epoch ms when the session was first created. */
  createdAt: number;
  /** Epoch ms of the last activity. */
  lastSeenAt: number;
  /** Whether the usage guide has been read in this session lifecycle. */
  hasReadGuide: boolean;
  /** Epoch ms when the guide was last read (for re-gating after TTL). */
  guideReadAt: number | null;
}

const sessions = new Map<string, SessionState>();

/** Create a fresh session. */
export function createSession(): SessionState {
  const now = Date.now();
  const session: SessionState = {
    id: randomUUID(),
    createdAt: now,
    lastSeenAt: now,
    hasReadGuide: false,
    guideReadAt: null,
  };
  sessions.set(session.id, session);
  return session;
}

/** Get a session by id, returning null if it doesn't exist or has expired. */
export function getSession(id: string): SessionState | null {
  const session = sessions.get(id);
  if (!session) return null;

  // Check TTL: if the guide was read more than 3 days ago, reset it.
  if (session.guideReadAt !== null) {
    const elapsed = Date.now() - session.guideReadAt;
    if (elapsed > SESSION_TTL_MS) {
      // TTL expired - require re-reading the guide.
      session.hasReadGuide = false;
      session.guideReadAt = null;
    }
  }

  session.lastSeenAt = Date.now();
  return session;
}

/** Mark a session as having read the usage guide. */
export function markGuideRead(id: string): void {
  const session = sessions.get(id);
  if (session) {
    session.hasReadGuide = true;
    session.guideReadAt = Date.now();
  }
}

/** Returns true if the session has read the guide within the TTL window. */
export function hasValidGuideRead(id: string): boolean {
  const session = getSession(id);
  if (!session) return false;
  return session.hasReadGuide;
}

/** The human-readable TTL duration for inclusion in messages. */
export const SESSION_TTL_DESCRIPTION = "3 days";
