#!/usr/bin/env node
/**
 * SessionEnd hook: Notify aspy proxy that session ended
 *
 * Called when Claude Code session ends (quit, clear, logout, etc).
 * Archives the session in the proxy for history tracking.
 *
 * Input (stdin): JSON with session_id, reason, etc.
 * Output: None needed (session is ending anyway)
 */

import { createHash } from 'crypto';

const ASPY_API_URL = process.env.ASPY_API_URL || 'http://127.0.0.1:8080';

async function main() {
  // Read stdin
  const chunks = [];
  for await (const chunk of process.stdin) {
    chunks.push(chunk);
  }
  const input = Buffer.concat(chunks).toString('utf8');

  let sessionData;
  try {
    sessionData = JSON.parse(input);
  } catch {
    process.exit(0);
  }

  const sessionId = sessionData.session_id;
  const reason = sessionData.reason || 'other';

  if (!sessionId) {
    process.exit(0);
  }

  // Compute user_id from API key (SHA-256, first 16 chars)
  let userId = 'unknown';
  const apiKey = process.env.ANTHROPIC_API_KEY;
  const authToken = process.env.ANTHROPIC_AUTH_TOKEN;

  if (apiKey) {
    userId = createHash('sha256').update(apiKey).digest('hex').slice(0, 16);
  } else if (authToken) {
    userId = createHash('sha256').update(authToken).digest('hex').slice(0, 16);
  }

  // Send session end to proxy (fire-and-forget, very short timeout)
  try {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), 2000);

    fetch(`${ASPY_API_URL}/api/session/end`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ session_id: sessionId, user_id: userId, reason }),
      signal: controller.signal,
    }).catch(() => {});

    clearTimeout(timeout);
  } catch {
    // Silently ignore - session is ending anyway
  }

  process.exit(0);
}

main();
