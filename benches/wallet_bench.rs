use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sui_wallet_demo::SuiWallet;

fn benchmark_sui_wallet_generation(c: &mut Criterion) {
    c.bench_function("sui_wallet_generation", |b| {
        b.iter(|| {
            let wallet = SuiWallet::generate().unwrap();
            black_box(wallet);
        })
    });
}

criterion_group!(benches, benchmark_sui_wallet_generation);
criterion_main!(benches);
