Real-World Integration
Consumes feeds from: Nasdaq, CBOE, EUREX

Outputs normalized events to C++ risk engine via shared memory ring buffer

Supports tick-by-tick backtesting with deterministic replay

Integrates with monitoring/alerting for exchange disconnects

Technical Highlights for Hiring
Rust Expertise: Demonstrates mastery of ownership, lifetimes, concurrency primitives

Low-Latency Design: Ring buffers, lock-free algorithms, CPU cache awareness

Production Patterns: Error handling, metrics, observability, graceful shutdown

Performance Profiling: Benchmarks, flamegraph analysis, deterministic timing

Multi-Exchange Support: Extensible feed handler architecture

Testing Strategy
Unit Tests: Parser correctness, order book state transitions

Integration Tests: Multi-feed scenario, out-of-order message handling

Benchmarks: Throughput under sustained load, latency percentiles (p50, p99, p99.9)

Chaos Engineering: Feed disconnects, corrupted messages, clock skew

Deployment Considerations
Linux kernel tuning: mlockall(), RT scheduling priority

# Build with optimizations
cargo build --release

# Run with latency tracking
RUST_LOG=info cargo run --release -- --feed exchange.cboe

# Benchmark throughput
cargo bench --bench throughput

# Run latency profiler
cargo run --release --example latency_profiler

CPU affinity pinning to isolated cores

NUMA awareness for multi-socket systems

# Build library and binary
cargo build --release

# Run the binary
./target/release/hft-market-data-engine

# Run with logging
RUST_LOG=debug cargo run --release

# Run tests
cargo test --release

# Run benchmarks
cargo bench


Real-time kernel recommended for sub-microsecond consistency
