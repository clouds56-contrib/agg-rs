#!/bin/bash
# Code coverage generation script for agg-rs
set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}agg-rs Code Coverage Generator${NC}"
echo "======================================"

# Function to print colored status messages
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    print_error "cargo-llvm-cov is not installed. Please install it with:"
    echo "  cargo install cargo-llvm-cov"
    echo "  rustup component add llvm-tools-preview"
    exit 1
fi

# Parse command line arguments
FORMAT="html"
OUTPUT_DIR="target/coverage"
OPEN_REPORT=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --format)
            FORMAT="$2"
            shift 2
            ;;
        --output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --open)
            OPEN_REPORT=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --format FORMAT     Coverage report format (html, lcov, json, text)"
            echo "  --output-dir DIR    Output directory for reports"
            echo "  --open              Open HTML report in browser after generation"
            echo "  --help, -h          Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                         # Generate HTML report"
            echo "  $0 --format lcov           # Generate LCOV report"
            echo "  $0 --open                  # Generate and open HTML report"
            echo "  $0 --format text           # Generate text summary"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Clean previous coverage data
print_status "Cleaning previous coverage data..."
cargo llvm-cov clean

# Generate coverage based on format
case $FORMAT in
    "html")
        print_status "Generating HTML coverage report..."
        cargo llvm-cov --all-targets --all-features --workspace --html --output-dir "$OUTPUT_DIR"
        print_status "HTML coverage report generated in: $OUTPUT_DIR"
        
        if [[ "$OPEN_REPORT" == true ]]; then
            if command -v xdg-open &> /dev/null; then
                xdg-open "$OUTPUT_DIR/index.html"
            elif command -v open &> /dev/null; then
                open "$OUTPUT_DIR/index.html"
            else
                print_status "Please open $OUTPUT_DIR/index.html in your browser"
            fi
        fi
        ;;
    "lcov")
        print_status "Generating LCOV coverage report..."
        mkdir -p "$(dirname "$OUTPUT_DIR/lcov.info")"
        cargo llvm-cov --all-targets --all-features --workspace --lcov --output-path "$OUTPUT_DIR/lcov.info"
        print_status "LCOV coverage report generated: $OUTPUT_DIR/lcov.info"
        ;;
    "json")
        print_status "Generating JSON coverage report..."
        mkdir -p "$(dirname "$OUTPUT_DIR/coverage.json")"
        cargo llvm-cov --all-targets --all-features --workspace --json --output-path "$OUTPUT_DIR/coverage.json"
        print_status "JSON coverage report generated: $OUTPUT_DIR/coverage.json"
        ;;
    "text")
        print_status "Generating text coverage summary..."
        mkdir -p "$OUTPUT_DIR"
        cargo llvm-cov --all-targets --all-features --workspace | tee "$OUTPUT_DIR/coverage_summary.txt"
        print_status "Text coverage summary saved: $OUTPUT_DIR/coverage_summary.txt"
        ;;
    *)
        print_error "Unsupported format: $FORMAT"
        print_error "Supported formats: html, lcov, json, text"
        exit 1
        ;;
esac

print_status "Coverage generation completed!"