# rsomics-vcf-index

Build a coordinate index (`.csi` / `.tbi`) for a bgzip-compressed VCF/BCF — a
Rust port of `bcftools index` (and tabix-style indexing). Enables fast
region-restricted queries on large variant files.

## Install

```sh
cargo install rsomics-vcf-index
```

## Usage

```sh
rsomics-vcf-index input.vcf.gz          # writes input.vcf.gz.csi
rsomics-vcf-index -t input.vcf.gz       # writes input.vcf.gz.tbi (tabix)
```

| flag | meaning | default |
|---|---|---|
| `-t, --tbi` | emit a `.tbi` (tabix) index instead of `.csi` | csi |
| `-f, --force` | overwrite an existing index | off |

## Origin

Independent Rust reimplementation of `bcftools index` based on the public
VCF/BCF + CSI/TBI index format specifications and black-box testing against the
`bcftools`/`tabix` binaries. No GPL/MIT upstream source was used as reference.

License: MIT OR Apache-2.0.
Upstream credit: [bcftools / htslib](https://www.htslib.org/) (MIT/Expat).
