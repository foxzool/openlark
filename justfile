# Format code
fmt:
  @echo "🎨 Formatting code..."
  cargo fmt --all

# Check code formatting
fmt-check:
  @echo "🔍 Checking code format..."
  cargo fmt --all -- --check

# Lint code
lint:
  @echo "🔍 Linting code (exclude benches/dev-tests)..."
  cargo clippy --workspace --all-targets --all-features -- -Dwarnings -A missing_docs

# Run tests
test:
  @echo "🧪 Running tests..."
  cargo test --workspace --all-features

# Build project
build:
  @echo "🔨 Building project..."
  cargo build --workspace --all-features

# Issue #41 guardrail
issue41-guardrail:
  @echo "🛡️ Running issue #41 guardrail..."
  python3 tools/issue41_guardrail.py

# Generate typed API coverage reports per crate and summary dashboard
api-coverage:
  @echo "📊 Generating typed API coverage reports..."
  python3 tools/validate_apis.py --all-crates
  @echo "✅ Coverage reports generated in reports/api_validation/"

# Validate typed API endpoint contracts against the official API snapshot
api-contracts:
  @echo "🔍 Validating typed API endpoint contracts..."
  python3 tools/validate_api_contracts.py --all-crates --strict endpoint

# Validate request/response fields against current official docs for one crate
api-contract-fields CRATE="openlark-ai" MAX="5":
  @echo "🔍 Validating typed API fields against current official docs..."
  python3 tools/validate_api_contracts.py --crate {{CRATE}} --fields --live-fields --max-field-apis {{MAX}} --report-dir reports/api_contract_fields

# Strict request/response field validation against current official docs for one crate
api-contract-fields-strict CRATE="openlark-ai" MAX="5":
  @echo "🔍 Strictly validating typed API fields against current official docs..."
  python3 tools/validate_api_contracts.py --crate {{CRATE}} --fields --live-fields --max-field-apis {{MAX}} --report-dir reports/api_contract_fields --strict fields

# Regenerate crates.md from mapping + CSV
update-crates-md:
  @echo "📝 Regenerating crates.md..."
  python3 tools/update_crates_md.py

# Check crates.md is synced with mapping + CSV
check-crates-md:
  @echo "🔍 Checking crates.md sync..."
  python3 tools/update_crates_md.py --check

# Build release
build-release:
  @echo "🚀 Building release..."
  cargo build --workspace --release --all-features

# Generate documentation
docs:
  @echo "📚 Generating documentation..."
  cargo doc --workspace --all-features --no-deps

# Run coverage tests (requires cargo-llvm-cov)
coverage:
  @echo "📊 Running coverage analysis..."
  cargo llvm-cov clean --workspace
  cargo llvm-cov test --all-features --workspace \
    --html --output-dir target/llvm-cov/html
  cargo llvm-cov report --lcov --output-path target/llvm-cov/lcov.info
  @echo "📋 Coverage report generated:"
  @echo "  - HTML: target/llvm-cov/html/index.html"
  @echo "  - LCOV: target/llvm-cov/lcov.info"

# Run coverage with threshold check
coverage-check:
  @echo "📊 Running coverage with threshold check..."
  just coverage
  @echo "🔍 Checking coverage threshold..."
  @cargo llvm-cov report --summary-only | tail -1 | awk '{ \
    gsub(/%/, "", $$7); \
    cov = $$7; \
    printf "📊 Coverage: %s%%\n", cov; \
    min = "${MIN_COVERAGE:-54.0}"; \
    if (cov+0 >= min+0) { \
      print "✅ Coverage " cov "% >= threshold " min "%"; \
    } else { \
      print "❌ Coverage " cov "% < threshold " min "%"; \
      exit 1; \
    } \
  }'

# Run security audit
audit:
  @echo "🔒 Running security audit..."
  @echo "Checking for security vulnerabilities..."
  cargo audit
  @echo "Checking licenses and supply chain..."
  cargo deny check
  @echo "✅ Security audit completed!"

# Update security databases
update-audit-db:
  @echo "🔄 Updating security advisory database..."
  cargo audit --update
  @echo "✅ Security database updated!"

# Test feature combinations (requires cargo-hack)
test-features:
  @echo "🧪 Testing feature combinations..."
  @echo "Testing each feature individually (excluding websocket/otel)..."
  cargo hack test --each-feature --exclude-features websocket,otel --lib
  @echo "Testing common feature combinations..."
  cargo test --no-default-features --lib
  cargo test --no-default-features --features "auth,communication" --lib
  cargo test --no-default-features --features "auth,docs-bitable" --lib
  cargo test --no-default-features --features "auth,docs-drive" --lib
  cargo test --no-default-features --features "essential" --lib
  cargo test --no-default-features --features "enterprise" --lib
  cargo test --no-default-features --features "webhook-full" --lib
  cargo test --no-default-features --features "communication,websocket" --lib
  @echo "✅ Feature matrix testing completed!"

# Quick feature combination test (most common combinations)
test-features-quick:
  @echo "🧪 Quick feature combination testing..."
  @echo "Testing minimal build..."
  cargo test --no-default-features --lib
  @echo "Testing essential business combo..."
  cargo test --no-default-features --features "essential" --lib
  @echo "Testing enterprise combo..."
  cargo test --no-default-features --features "enterprise" --lib
  @echo "Testing webhook combo..."
  cargo test --no-default-features --features "webhook-full" --lib
  @echo "✅ Quick feature testing completed!"

# Install development tools
install-dev-tools:
  @echo "🛠️ Installing development tools..."
  cargo install cargo-llvm-cov cargo-audit cargo-deny cargo-hack
  @echo "✅ Development tools installed!"

# Run all pre-release checks including coverage and security
check-all: fmt-check lint test coverage-check audit build-release docs
  @echo "✅ All checks passed!"

# Release a new version
release VERSION:
  @echo "🚀 Starting release process for version {{VERSION}}"
  
  # Check if on main branch
  @if [ "$(git branch --show-current)" != "main" ]; then \
    echo "⚠️  Warning: You are not on the main branch (current: $(git branch --show-current))"; \
    read -p "Continue anyway? (y/N): " -n 1 -r; \
    echo; \
    if [[ ! $$REPLY =~ ^[Yy]$$ ]]; then \
      echo "ℹ️  Aborting release"; \
      exit 1; \
    fi; \
  fi
  
  # Check if working directory is clean
  @if ! git diff-index --quiet HEAD --; then \
    echo "❌ Working directory is not clean. Please commit or stash your changes."; \
    git status --porcelain; \
    exit 1; \
  fi
  
  # Pull latest changes
  @echo "🔄 Pulling latest changes..."
  git pull origin main
  
  # Check if tag already exists
  @if git tag -l | grep -q "^v{{VERSION}}$$"; then \
    echo "❌ Tag v{{VERSION}} already exists"; \
    exit 1; \
  fi
  
  # Verify version in Cargo.toml
  @CARGO_VERSION=$$(grep '^version = ' Cargo.toml | cut -d'"' -f2); \
  if [ "$$CARGO_VERSION" != "{{VERSION}}" ]; then \
    echo "❌ Version mismatch: Cargo.toml has $$CARGO_VERSION, but you specified {{VERSION}}"; \
    echo "ℹ️  Please update Cargo.toml first"; \
    exit 1; \
  fi
  
  # Run pre-release checks
  @echo "🔍 Running pre-release checks..."
  just check-all
  
  # Check changelog
  @if ! grep -q "## \[{{VERSION}}\]" CHANGELOG.md; then \
    echo "⚠️  No changelog entry found for version {{VERSION}}"; \
    echo "ℹ️  Please update CHANGELOG.md before releasing"; \
    read -p "Continue anyway? (y/N): " -n 1 -r; \
    echo; \
    if [[ ! $$REPLY =~ ^[Yy]$$ ]]; then \
      echo "ℹ️  Aborting release"; \
      exit 1; \
    fi; \
  fi
  
  # Create and push tag
  @echo "🏷️  Creating tag v{{VERSION}}..."
  git tag -a "v{{VERSION}}" -m "Release version {{VERSION}}"
  
  @echo "📤 Pushing tag to origin..."
  git push origin "v{{VERSION}}"
  
  @echo "✅ Tag v{{VERSION}} has been created and pushed!"
  @echo "ℹ️  GitHub Actions will now:"
  @echo "  1. Run validation tests"
  @echo "  2. Create a GitHub release"
  @echo "  3. Publish to crates.io"
  @echo ""
  @echo "ℹ️  Monitor progress at: https://github.com/foxzool/openlark/actions"
  @echo ""
  @echo "🚀 Release process initiated successfully!"


# Show available commands
help:
  @echo "📋 Available commands:"
  @echo "  fmt          - Format code"
  @echo "  fmt-check    - Check code formatting"
  @echo "  lint         - Lint code with clippy"
  @echo "  test         - Run tests"
  @echo "  test-features - Test all feature combinations (slow)"
  @echo "  test-features-quick - Test common feature combinations"
  @echo "  build        - Build project"
  @echo "  build-release - Build release version"
  @echo "  docs         - Generate documentation"
  @echo "  coverage     - Run coverage analysis"
  @echo "  coverage-check - Run coverage with threshold check"
  @echo "  audit        - Run security audit"
  @echo "  api-coverage - Generate typed API coverage reports (per crate + summary)"
  @echo "  api-contracts - Validate typed API endpoint contracts"
  @echo "  api-contract-fields - Validate request/response fields against live official docs"
  @echo "  api-contract-fields-strict - Strict request/response field validation"
  @echo "  update-audit-db - Update security advisory database"
  @echo "  install-dev-tools - Install development tools"
  @echo "  check-all    - Run all pre-release checks (includes coverage & security)"
  @echo "  release VERSION - Release a new version (e.g., just release 0.4.0)"
  @echo "  help         - Show this help message"
