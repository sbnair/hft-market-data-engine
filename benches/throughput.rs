use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hft_market_data_engine::feed::market_data::*;
use hft_market_data_engine::feed::fix_parser::FixParser;

fn fix_parse_benchmark(c: &mut Criterion) {
    let msg = b"35=D|55=AAPL|132=1000|34=15050|149=100|35=15100|150=100|60=1000";
    
    c.bench_function("fix_parse_100k", |b| {
        b.iter(|| {
            for _ in 0..100_000 {
                black_box(FixParser::parse_quote(black_box(msg))).ok();
            }
        })
    });
}

criterion_group!(benches, fix_parse_benchmark);
criterion_main!(benches);
