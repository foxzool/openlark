#!/usr/bin/env bash
set -euo pipefail

echo "🚀 OpenLark Workspace Publish Script"
echo "===================================="
echo ""

SLEEP_DURATION="${SLEEP_DURATION:-60}"
DRY_RUN="${DRY_RUN:-false}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

publish_crate() {
    local crate=$1
    local delay=${2:-$SLEEP_DURATION}
    local output

    echo -e "${YELLOW}Publishing ${crate}...${NC}"

    if [ "$DRY_RUN" = "true" ]; then
        cargo publish -p "$crate" --registry crates-io --dry-run
    elif output=$(cargo publish -p "$crate" --registry crates-io 2>&1); then
        printf '%s\n' "$output"
        echo -e "${GREEN}✅ ${crate} published successfully${NC}"
    elif grep -q "is already uploaded" <<<"$output"; then
        printf '%s\n' "$output"
        echo -e "${YELLOW}⚠️ ${crate} exact version already exists; continuing retry${NC}"
    else
        printf '%s\n' "$output" >&2
        echo -e "${RED}❌ Failed to publish ${crate}${NC}"
        return 1
    fi

    if [ "$DRY_RUN" != "true" ] && [ "$delay" -gt 0 ]; then
        echo "Waiting ${delay}s for index propagation..."
        sleep "$delay"
    fi
    echo ""
}

PUBLISH_ORDER=(
    # Layer 1: Core
    "openlark-core"

    # Layer 2: Business crates
    "openlark-auth"
    "openlark-security"
    "openlark-communication"
    "openlark-cardkit"
    "openlark-webhook"
    "openlark-docs"
    "openlark-hr"
    "openlark-ai"
    "openlark-application"
    "openlark-platform"
    "openlark-meeting"
    "openlark-helpdesk"
    "openlark-mail"
    "openlark-bot"
    "openlark-workflow"
    "openlark-analytics"
    "openlark-user"

    # Layer 3: Client
    "openlark-client"

    # Layer 4: Root crate
    "openlark"
)

for crate in "${PUBLISH_ORDER[@]}"; do
    case "$crate" in
        openlark-core | openlark-client)
            publish_crate "$crate" 30
            ;;
        openlark)
            publish_crate "$crate" 0
            ;;
        *)
            publish_crate "$crate" "$SLEEP_DURATION"
            ;;
    esac
done

echo -e "${GREEN}====================================${NC}"
echo -e "${GREEN}🎉 All crates published successfully!${NC}"
echo -e "${GREEN}====================================${NC}"
