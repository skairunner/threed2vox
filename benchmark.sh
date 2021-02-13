cargo build --release --bin bench_teapot --features="sequential"
hyperfine --warmup 1 --min-runs=5 ./target/release/bench_teapot
