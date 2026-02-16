#!/usr/bin/env bash
# scripts/prism-test.sh
# Manages mock server lifecycle for SDK integration tests.
# Usage: just sdk-test-integration   OR   ./scripts/prism-test.sh

set -euo pipefail

MOCK_PORT=4010
MOCK_HOST=127.0.0.1
MOCK_URL="http://${MOCK_HOST}:${MOCK_PORT}"
MAX_WAIT=10

# ── Start mock server ────────────────────────────────────────
echo "Starting mock server on ${MOCK_URL} ..."
python3 scripts/mock-server.py --port "${MOCK_PORT}" --host "${MOCK_HOST}" &
MOCK_PID=$!

# Ensure server is killed on exit regardless of outcome
cleanup() {
  if kill -0 "${MOCK_PID}" 2>/dev/null; then
      echo "Stopping mock server (PID ${MOCK_PID})..."
      kill "${MOCK_PID}" 2>/dev/null || true
      wait "${MOCK_PID}" 2>/dev/null || true
  fi
}
trap cleanup EXIT

# ── Health check ─────────────────────────────────────────────
echo "Waiting for mock server to become healthy (max ${MAX_WAIT}s)..."
elapsed=0
while ! curl -sf -o /dev/null "${MOCK_URL}/v1/models" 2>/dev/null; do
  if ! kill -0 "${MOCK_PID}" 2>/dev/null; then
    echo "ERROR: Mock server process exited unexpectedly"
    exit 1
  fi
  if [ "${elapsed}" -ge "${MAX_WAIT}" ]; then
    echo "ERROR: Mock server did not become healthy within ${MAX_WAIT}s"
    exit 1
  fi
  sleep 1
  elapsed=$((elapsed + 1))
done
echo "Mock server is healthy after ${elapsed}s."

# ── Run tests ────────────────────────────────────────────────
echo "Running integration tests against mock server at ${MOCK_URL} ..."
cargo test -p elevenlabs-sdk --test integration_test -- --ignored --nocapture
TEST_EXIT=$?

# cleanup runs via trap automatically
exit "${TEST_EXIT}"
