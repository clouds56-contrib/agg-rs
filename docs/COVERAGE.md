# Code Coverage Documentation

This document describes the code coverage setup and tools available for the agg-rs project.

## Overview

The project uses Rust's built-in LLVM-based code coverage through `cargo-llvm-cov`, which provides accurate source-based coverage instrumentation. Current coverage is approximately **80.65%** line coverage.

## Quick Start

### Prerequisites

1. Install the coverage tools:
   ```bash
   cargo install cargo-llvm-cov
   rustup component add llvm-tools-preview
   ```

### Generate Coverage Reports

Use the provided convenience script:

```bash
# Generate HTML coverage report (default)
./scripts/coverage.sh

# Generate and open HTML report in browser
./scripts/coverage.sh --open

# Generate LCOV format for external tools
./scripts/coverage.sh --format lcov

# Generate text summary
./scripts/coverage.sh --format text

# Generate JSON format
./scripts/coverage.sh --format json
```

Or use the Makefile targets:

```bash
make coverage          # Text summary
make coverage-html     # HTML report
make coverage-open     # HTML report and open in browser
```

## CI/CD Integration

The project includes a GitHub Actions workflow (`.github/workflows/coverage.yml`) that:

1. Runs on every push and pull request to `master`
2. Generates LCOV coverage data
3. Uploads results to [Codecov](https://codecov.io/gh/clouds56-contrib/agg-rs)

Coverage badges are displayed in the README.md file.

## Coverage Configuration

Coverage settings can be configured in `Cargo.toml` under the `[package.metadata.coverage]` section. Currently, this is used to document coverage-related metadata.

## Manual Coverage Commands

For advanced usage, you can use `cargo-llvm-cov` directly:

```bash
# Generate text summary
cargo llvm-cov --all-targets --all-features --workspace

# Generate HTML report
cargo llvm-cov --all-targets --all-features --workspace --html --output-dir target/coverage

# Generate LCOV format
cargo llvm-cov --all-targets --all-features --workspace --lcov --output-path coverage.info

# Clean coverage data
cargo llvm-cov clean
```

## File Exclusions

Coverage artifacts are excluded from version control via `.gitignore`:

- `coverage.info`
- `lcov.info`
- `/coverage-html`
- `*.profraw`
- `*.profdata`
- `/target/coverage`

## Understanding Coverage Reports

### Text Summary
The text summary shows:
- **Regions**: Code regions that can be covered
- **Functions**: Function coverage statistics
- **Lines**: Line coverage statistics
- **Branches**: Branch coverage (when available)

### HTML Report
The HTML report provides:
- Interactive file-by-file coverage visualization
- Color-coded line coverage (green = covered, red = uncovered)
- Function and region coverage details
- Summary statistics

### LCOV Format
LCOV format is used for:
- Integration with external tools (IDEs, coverage services)
- Historical coverage tracking
- Coverage diff analysis

## Coverage Goals

Current coverage targets:
- **Line Coverage**: 80%+ (currently 80.65% ✅)
- **Function Coverage**: 80%+ (currently 80.64% ✅)

Areas with lower coverage that could benefit from additional testing:
- `sources/text.rs` (38.37% line coverage)
- `color/color_value.rs` (40.69% line coverage)
- `pixels/alpha_mask.rs` (60.00% line coverage)

## Troubleshooting

### Common Issues

1. **`cargo-llvm-cov` not found**
   ```bash
   cargo install cargo-llvm-cov
   ```

2. **LLVM tools missing**
   ```bash
   rustup component add llvm-tools-preview
   ```

3. **Permission denied on scripts**
   ```bash
   chmod +x scripts/coverage.sh
   ```

4. **Coverage data inconsistent**
   ```bash
   cargo llvm-cov clean
   ./scripts/coverage.sh
   ```

### Performance Notes

- Coverage builds are slower than regular builds due to instrumentation
- HTML generation takes longer than text/LCOV formats
- Use `cargo llvm-cov clean` to ensure fresh coverage data

## Contributing

When adding new code:
1. Write tests that exercise the new functionality
2. Run coverage locally to verify your tests are effective
3. Aim to maintain or improve overall coverage percentage
4. Consider edge cases and error paths in your tests

The coverage reports help identify untested code paths and ensure comprehensive testing of the graphics rendering engine.