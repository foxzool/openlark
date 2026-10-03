#!/usr/bin/env bash

set -euo pipefail

echo "Checking root README aligned example..."
cargo check --example readme_quick_start --no-default-features --features "auth,docs-bitable"

echo "Checking root architecture overview example..."
cargo check --example docs_helpers --no-default-features --features "auth,docs-bitable"

echo "Checking client setup example..."
cargo check --example client_setup --no-default-features --features "auth,communication"

echo "Checking communication workflow example..."
cargo check --example communication_workflows --no-default-features --features "auth,communication,workflow"

echo "Checking docs workflow example..."
cargo check --example docs_workflows --no-default-features --features "auth,docs-bitable"

echo "Checking workflow example..."
cargo check --example workflow_api_example --no-default-features --features "workflow"

echo "Checking websocket example..."
cargo check --example websocket_echo_bot --no-default-features --features "communication,websocket"

echo "Checking HTTP event inbound example..."
cargo check --example http_event_inbound --no-default-features --features "event-http"

echo "Checking websocket typed handler example..."
cargo check --example websocket_typed_handler --no-default-features --features "communication,websocket"

echo "Checking OAuth user token example..."
cargo check --example oauth_user_token --no-default-features --features "auth,communication"

echo "Checking openlark-client README examples..."
cargo check -p openlark-client --example client_readme_examples --no-default-features --features "communication,docs"

echo "Checking openlark-docs README examples..."
cargo check -p openlark-docs --example docs_readme_examples --no-default-features --features "ccm,bitable,baike"
