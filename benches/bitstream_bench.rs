use criterion::{criterion_group, criterion_main, Criterion};
use h264_422_10bit::H264Decoder;

fn bench_empty_stream(c: &mut Criterion) {
    c.bench_function("decode_empty_annex_b", |b| {
        let mut decoder = H264Decoder::new();
        let data: &[u8] = &[];
        b.iter(|| {
            let _ = decoder.push_annex_b(data).unwrap();
        });
    });
}

criterion_group!(benches, bench_empty_stream);
criterion_main!(benches);
