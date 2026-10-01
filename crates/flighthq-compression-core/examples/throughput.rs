// Measures decode throughput on real streams, so the case for a Rust mirror is a number rather than an
// assertion. Usage:
//
//     cargo run --release -p flighthq-compression-core --example throughput -- <iterations> <deflate|lzma> <file>…
//
// where each file is `<declared-uncompressed-length>:<compressed bytes>`. `agents/compression-mirror.md`
// records how to generate them and what the current figures are.
//
// Deliberately not a committed benchmark harness: neither algorithm has an encoder in this crate yet, so
// the inputs come from outside, and a fixture big enough to measure does not belong in the repository.

type Decoder = fn(&[u8], usize, flighthq_compression_core::Framing) -> Option<Vec<u8>>;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let iterations: u32 = arguments
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(10);
    let algorithm = arguments.next().unwrap_or_else(|| "deflate".to_owned());
    let (decode, framing): (Decoder, _) = match algorithm.as_str() {
        "deflate" => (
            flighthq_compression_core::decompress_deflate as Decoder,
            flighthq_compression_core::Framing::Rfc1950,
        ),
        // LZMA carries no wrapper, so its framing is Raw by definition.
        "lzma" => (
            flighthq_compression_core::decompress_lzma as Decoder,
            flighthq_compression_core::Framing::Raw,
        ),
        other => panic!("unknown algorithm {other}; expected deflate or lzma"),
    };

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

        let first = decode(compressed, declared, framing).expect("the payload must decode");
        assert_eq!(
            first.len(),
            declared,
            "{path}: decoded length disagrees with the declared length"
        );

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            assert!(decode(compressed, declared, framing).is_some());
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
