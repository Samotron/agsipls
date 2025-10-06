#!/bin/bash
# Integration test script for AGSi Rust implementation

set -e

echo "🧪 Running AGSi Integration Tests"
echo "=================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Test function
run_test() {
    local test_name="$1"
    local command="$2"
    TESTS_RUN=$((TESTS_RUN + 1))
    
    echo -n "  Testing: $test_name ... "
    
    if eval "$command" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

# Build check
echo "📦 Building unified binary..."
cargo build --release --quiet
echo ""

# Binary info
echo "📊 Binary information:"
BINARY_SIZE=$(ls -lh target/release/agsi | awk '{print $5}')
echo "  Size: $BINARY_SIZE"
echo "  Location: target/release/agsi"
echo ""

# Unit tests
echo "🔬 Running unit tests..."
cargo test --quiet
echo ""

# CLI tests
echo "🖥️  Testing CLI commands..."

run_test "Create document" \
    "./target/release/agsi create document --id TEST001 --output /tmp/test-doc.agsi.json"

run_test "Validate document" \
    "./target/release/agsi validate /tmp/test-doc.agsi.json"

run_test "Info command" \
    "./target/release/agsi info /tmp/test-doc.agsi.json"

run_test "Create with example" \
    "cargo run --package agsi-core --example create_model --quiet"

run_test "Extract materials" \
    "./target/release/agsi extract /tmp/example-ground-model.agsi.json --output /tmp/materials.json"

run_test "Info with materials" \
    "./target/release/agsi info /tmp/example-ground-model.agsi.json --materials"

echo ""

# MCP server tests
echo "🔌 Testing MCP server..."

run_test "MCP tools/list" \
    "echo '{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/list\"}' | timeout 2 ./target/release/agsi mcp 2>/dev/null | jq -e '.result.tools | length == 4'"

run_test "MCP agsi_get_info" \
    "echo '{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/call\",\"params\":{\"name\":\"agsi_get_info\",\"arguments\":{\"file_path\":\"/tmp/example-ground-model.agsi.json\"}}}' | timeout 2 ./target/release/agsi mcp 2>/dev/null | jq -e '.result.project'"

run_test "MCP agsi_validate" \
    "echo '{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/call\",\"params\":{\"name\":\"agsi_validate\",\"arguments\":{\"file_path\":\"/tmp/test-doc.agsi.json\"}}}' | timeout 2 ./target/release/agsi mcp 2>/dev/null | jq -e '.result.valid == true'"

echo ""

# LSP server test
echo "🔧 Testing LSP server..."

run_test "LSP server starts" \
    "timeout 1 ./target/release/agsi lsp 2>&1 | grep -q 'LSP Server'"

echo ""

# Library tests
echo "📚 Testing library integration..."

run_test "Example compiles and runs" \
    "cargo run --package agsi-core --example create_model --quiet"

echo ""

# Results
echo "=================================="
echo "📊 Test Results"
echo "=================================="
echo "  Total tests:  $TESTS_RUN"
echo -e "  Passed:       ${GREEN}$TESTS_PASSED${NC}"
if [ $TESTS_FAILED -gt 0 ]; then
    echo -e "  Failed:       ${RED}$TESTS_FAILED${NC}"
else
    echo -e "  Failed:       $TESTS_FAILED"
fi
echo ""
echo "📦 Unified Binary: target/release/agsi ($BINARY_SIZE)"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All tests passed!${NC}"
    echo "🎉 Single binary includes: CLI + MCP + LSP"
    exit 0
else
    echo -e "${RED}❌ Some tests failed${NC}"
    exit 1
fi
