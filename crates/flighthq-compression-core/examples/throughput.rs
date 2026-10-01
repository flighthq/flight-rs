// Measures decode throughput on real zlib streams, so the case for a Rust decoder is a number rather
// than an assertion. Takes one or more files, each `<declared-uncompressed-length>:<zlib bytes>`; the
// companion generator is documented in `agents/compression-mirror.md`.
//
// Deliberately not a committed benchmark harness: there is no encoder in this crate yet, so the inputs
// have to come from outside, and a fixture big enough to measure does not belong in the repository.

fn main() {
    let mut arguments = std::env::args().skip(1);
    let iterations: u32 = arguments
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(10);

    for path in arguments {
        let bytes = std::fs::read(&path).expect("read payload");
        let separator = bytes
            .iter()
            .position(|byte| *byte == b':')
            .expect("declared length prefix");
        let declared: usize = std::str::from_utf8(&bytes[..separator])
            .expect("utf8 length")
            .parse()
            .expect("numeric length");
        let compressed = &bytes[separator + 1..];

        let first = flighthq_compression_core::decompress_deflate(
            compressed,
            declared,
            flighthq_compression_core::Framing::Rfc1950,
        )
        .expect("the payload must decode");
        assert_eq!(
            first.len(),
            declared,
            "{path}: decoded length disagrees with the declared length"
        );

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let out = flighthq_compression_core::decompress_deflate(
                compressed,
                declared,
                flighthq_compression_core::Framing::Rfc1950,
            );
            assert!(out.is_some());
        }
        let per_run = start.elapsed().as_secs_f64() / f64::from(iterations);
        let megabytes = declared as f64 / 1024.0 / 1024.0;
        println!(
            "{path}: {:.2} ms  {:.0} MB/s",
            per_run * 1000.0,
            megabytes / per_run
        );
    }
}
