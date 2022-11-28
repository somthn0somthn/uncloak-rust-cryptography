use vigenere_cipher::*;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn vigenere_benchmark(c: &mut Criterion) {
    let plaintext = black_box(
        "attackatdawn"
    );
    let key = black_box(
        "lemon"
    );

    c.bench_function(
        "vigenere cipher encrypt decrypt",
        |b| b.iter(|| encrypt(plaintext, key)));

}


criterion_group!(benches, vigenere_benchmark);
criterion_main!(benches);