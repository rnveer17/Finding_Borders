use std::time::Instant;
use std::hint::black_box;
use finding_borders::{naive_border_check, kmp_border_compute};
use std::fs;

fn main() {
    let test_dir = "tests/inout";
    let mut test_cases = Vec::new();

    if let Ok(entries) = fs::read_dir(test_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("in") {
                let output_file = path.with_extension("out");
                if output_file.exists() {
                    test_cases.push(path);
                }
            }
        }
    }

    if test_cases.is_empty() {
        println!("No test cases found. Run `cargo build` first.");
        return;
    }

    test_cases.sort();

    println!("{:>10} | {:>15} | {:>15} | {:>15}",
             "n", "Naive (ms)", "KMP (ms)", "Diff (ms)");
    println!("{:-<10}-+-{:-<15}-+-{:-<15}-+-{:-<15}", "", "", "", "");

    for input_file in &test_cases {
        let content = fs::read_to_string(input_file).unwrap();
        let s = content.trim();
        let n = s.len();

        if n == 0 {
            continue;
        }

        // Naive (only for small n to avoid timeouts)
        let naive_ms = if n <= 10000 {
            let start = Instant::now();
            let _ = black_box(naive_border_check(s));
            start.elapsed().as_secs_f64() * 1000.0
        } else {
            f64::NAN
        };

        let start = Instant::now();
        let _ = black_box(kmp_border_compute(s));
        let kmp_ms = start.elapsed().as_secs_f64() * 1000.0;

        let diff = kmp_ms - naive_ms;

        println!(
            "{:>10} | {:>15.2} | {:>15.2} | {:>15.2}",
            n,
            if naive_ms.is_nan() { f64::NAN } else { naive_ms },
            kmp_ms,
            diff
        );
    }
}