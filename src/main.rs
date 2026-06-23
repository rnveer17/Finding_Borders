use finding_borders::kmp_border_compute;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let s = input.trim();

    let borders = kmp_border_compute(s);

    for b in borders {
        print!("{} ", b);
    }
    println!();
}