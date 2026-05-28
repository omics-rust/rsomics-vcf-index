use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

fn bench_vcf_index(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-vcf-index");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let src_vcf_gz = manifest.join("tests/golden/small.vcf.gz");
    let dir = tempdir().unwrap();
    let vcf_copy = dir.path().join("small.vcf.gz");
    std::fs::copy(&src_vcf_gz, &vcf_copy).unwrap();
    c.bench_function("rsomics-vcf-index golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .arg(vcf_copy.to_str().unwrap())
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_vcf_index);
criterion_main!(benches);
