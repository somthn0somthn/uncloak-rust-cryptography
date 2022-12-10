use sha512n::{question1, question3, question3a};
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use sha2::{Sha256, Sha512, Digest};
use sha3::Sha3_256;
use rand::Rng;
use generic_array::{GenericArray, typenum::{U1, U2, U3, U4}};
use blake3::*;


/* fn bench(c: &mut Criterion) {
    

    c.bench_function(
        "question3",
        |b| {
            let mut hash = Sha3_256::new();
            b.iter(|| question3(black_box(&mut hash,)))
        }
    );
} */

fn bench(c: &mut Criterion) {    

    c.bench_function(
        "question3",
        |b| {
            let mut hash = blake3::Hasher::new();
            b.iter(|| question3a(black_box(&mut hash)))
        }
    );
}


criterion_group!(benches, bench);
criterion_main!(benches);