#!/usr/bin/env python3
"""Lightweight mock server for ElevenLabs API integration tests.

Serves mock responses matching the OpenAPI spec for endpoints exercised by
``crates/elevenlabs-sdk/tests/integration_test.rs``.

Usage:
    python3 scripts/mock-server.py [--port PORT]
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from http.server import HTTPServer, BaseHTTPRequestHandler

# ── Mock response data ───────────────────────────────────────────────────────

MODELS = json.dumps(
    [
        {
            "model_id": "eleven_multilingual_v2",
            "name": "Eleven Multilingual v2",
            "can_be_finetuned": True,
            "can_do_text_to_speech": True,
            "can_do_voice_conversion": True,
            "can_use_style": True,
            "can_use_speaker_boost": True,
            "serves_pro_voices": False,
            "token_cost_factor": 1.0,
            "description": "Mock model for testing",
            "requires_alpha_access": False,
            "max_characters_request_free_user": 2500,
            "max_characters_request_subscribed_user": 5000,
            "maximum_text_length_per_request": 1000000,
            "languages": [{"language_id": "en", "name": "English"}],
            "model_rates": {"character_cost_multiplier": 1.0},
            "concurrency_group": "standard",
        }
    ]
)

VOICES = json.dumps(
    {
        "voices": [
            {
                "voice_id": "21m00Tcm4TlvDq8ikWAM",
                "name": "Rachel",
                "category": "premade",
                "labels": {},
                "available_for_tiers": [],
                "high_quality_base_model_ids": [],
            }
        ]
    }
)

VOICE_SETTINGS = json.dumps(
    {
        "stability": 0.5,
        "similarity_boost": 0.75,
        "style": 0.0,
        "use_speaker_boost": True,
        "speed": 1.0,
    }
)

USER = json.dumps(
    {
        "user_id": "test-user-id",
        "subscription": {
            "tier": "creator",
            "character_count": 5000,
            "character_limit": 100000,
            "can_extend_character_limit": True,
            "allowed_to_extend_character_limit": True,
            "voice_slots_used": 3,
            "professional_voice_slots_used": 0,
            "voice_limit": 30,
            "voice_add_edit_counter": 5,
            "professional_voice_limit": 1,
            "can_extend_voice_limit": True,
            "can_use_instant_voice_cloning": True,
            "can_use_professional_voice_cloning": True,
            "status": "active",
        },
        "is_new_user": False,
        "can_use_delayed_payment_methods": False,
        "is_onboarding_completed": True,
        "is_onboarding_checklist_completed": True,
        "created_at": 1700000000,
    }
)

SUBSCRIPTION = json.dumps(
    {
        "tier": "creator",
        "character_count": 5000,
        "character_limit": 100000,
        "can_extend_character_limit": True,
        "allowed_to_extend_character_limit": True,
        "voice_slots_used": 3,
        "professional_voice_slots_used": 0,
        "voice_limit": 30,
        "voice_add_edit_counter": 5,
        "professional_voice_limit": 1,
        "can_extend_voice_limit": True,
        "can_use_instant_voice_cloning": True,
        "can_use_professional_voice_cloning": True,
    }
)

HISTORY = json.dumps(
    {
        "history": [
            {
                "history_item_id": "mock-item-1",
                "date_unix": 1714650306,
                "character_count_change_from": 100,
                "character_count_change_to": 150,
                "content_type": "audio/mpeg",
                "state": "created",
            }
        ],
        "has_more": False,
    }
)

VOICE_GEN_PARAMS = json.dumps(
    {
        "genders": [{"name": "Female", "code": "female"}],
        "accents": [{"name": "American", "code": "american"}],
        "ages": [{"name": "Young", "code": "young"}],
        "minimum_characters": 100,
        "maximum_characters": 1000,
        "minimum_accent_strength": 0.3,
        "maximum_accent_strength": 2.0,
    }
)

PROJECTS = json.dumps(
    {
        "projects": [
            {
                "project_id": "mock-project-1",
                "name": "Test Project",
                "create_date_unix": 1700000000,
                "default_title_voice_id": "v1",
                "default_paragraph_voice_id": "v2",
                "default_model_id": "eleven_multilingual_v2",
                "can_be_downloaded": False,
                "volume_normalization": True,
                "state": "default",
                "access_level": "owner",
                "quality_check_on": True,
                "quality_check_on_when_bulk_convert": True,
            }
        ]
    }
)

DUBBING = json.dumps(
    {
        "dubs": [
            {
                "dubbing_id": "mock-dub-1",
                "name": "Test Dub",
                "status": "dubbed",
                "target_languages": ["es"],
                "editable": False,
                "created_at": "2024-01-01T00:00:00Z",
            }
        ],
        "has_more": False,
    }
)

AGENTS = json.dumps(
    {
        "agents": [
            {
                "agent_id": "mock-agent-1",
                "name": "Test Agent",
                "tags": [],
                "created_at_unix_secs": 1700000000,
                "access_info": {
                    "is_creator": True,
                    "creator_name": "Test",
                    "creator_email": "test@test.com",
                    "role": "admin",
                },
            }
        ],
        "has_more": False,
    }
)

SERVICE_ACCOUNTS = json.dumps(
    {
        "service-accounts": [
            {
                "service_account_user_id": "mock-sa-1",
                "name": "Test SA",
                "api-keys": [],
            }
        ]
    }
)

# Minimal valid audio bytes (not real audio, but enough for a Bytes response)
TTS_AUDIO = b"\xff\xfb\x90\x00" + b"\x00" * 128

# ── Route table ──────────────────────────────────────────────────────────────

# Each entry: (method, compiled regex) -> (status, content_type, body)
ROUTES: list[tuple[str, re.Pattern[str], int, str, bytes]] = [
    ("GET", re.compile(r"^/v1/models$"), 200, "application/json", MODELS.encode()),
    ("GET", re.compile(r"^/v1/voices$"), 200, "application/json", VOICES.encode()),
    (
        "GET",
        re.compile(r"^/v1/voices/settings/default$"),
        200,
        "application/json",
        VOICE_SETTINGS.encode(),
    ),
    ("GET", re.compile(r"^/v1/user$"), 200, "application/json", USER.encode()),
    (
        "GET",
        re.compile(r"^/v1/user/subscription$"),
        200,
        "application/json",
        SUBSCRIPTION.encode(),
    ),
    ("GET", re.compile(r"^/v1/history$"), 200, "application/json", HISTORY.encode()),
    (
        "POST",
        re.compile(r"^/v1/text-to-speech/[^/]+$"),
        200,
        "audio/mpeg",
        TTS_AUDIO,
    ),
    (
        "GET",
        re.compile(r"^/v1/voice-generation/generate-voice/parameters$"),
        200,
        "application/json",
        VOICE_GEN_PARAMS.encode(),
    ),
    (
        "GET",
        re.compile(r"^/v1/studio/projects$"),
        200,
        "application/json",
        PROJECTS.encode(),
    ),
    ("GET", re.compile(r"^/v1/dubbing$"), 200, "application/json", DUBBING.encode()),
    (
        "GET",
        re.compile(r"^/v1/convai/agents$"),
        200,
        "application/json",
        AGENTS.encode(),
    ),
    (
        "GET",
        re.compile(r"^/v1/service-accounts$"),
        200,
        "application/json",
        SERVICE_ACCOUNTS.encode(),
    ),
]


# ── Handler ──────────────────────────────────────────────────────────────────


class MockHandler(BaseHTTPRequestHandler):
    """Handles mock API requests."""

    def _route(self, method: str) -> None:
        # Strip query string for matching
        path = self.path.split("?")[0]
        for route_method, pattern, status, content_type, body in ROUTES:
            if route_method == method and pattern.match(path):
                self.send_response(status)
                self.send_header("Content-Type", content_type)
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                self.wfile.write(body)
                return
        # Not found
        self.send_response(404)
        self.send_header("Content-Type", "application/json")
        msg = json.dumps({"error": f"Not found: {method} {path}"}).encode()
        self.send_header("Content-Length", str(len(msg)))
        self.end_headers()
        self.wfile.write(msg)

    def do_GET(self) -> None:  # noqa: N802
        self._route("GET")

    def do_POST(self) -> None:  # noqa: N802
        # Consume request body to avoid broken pipe
        content_length = int(self.headers.get("Content-Length", 0))
        if content_length > 0:
            self.rfile.read(content_length)
        self._route("POST")

    def log_message(self, format: str, *args: object) -> None:
        """Suppress default stderr logging; print one-liners to stdout."""
        print(f"[mock] {args[0]}", flush=True)


# ── Main ─────────────────────────────────────────────────────────────────────


def main() -> None:
    parser = argparse.ArgumentParser(description="ElevenLabs mock API server")
    parser.add_argument("--port", type=int, default=4010, help="Port to listen on")
    parser.add_argument("--host", type=str, default="127.0.0.1", help="Host to bind to")
    args = parser.parse_args()

    server = HTTPServer((args.host, args.port), MockHandler)
    print(f"Mock server listening on http://{args.host}:{args.port}", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down mock server.", flush=True)
        server.server_close()
        sys.exit(0)


if __name__ == "__main__":
    main()
