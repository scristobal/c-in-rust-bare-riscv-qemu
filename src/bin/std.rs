use std::io::{self, BufRead, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter n m: ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    let mut parts = line.split_whitespace();
    let n: u32 = parts.next().ok_or("missing n")?.parse()?;
    let m: u32 = parts.next().ok_or("missing m")?.parse()?;

    println!("gcd({}, {}) = {}", n, m, rust_bare_riscv::some_c::gcd(n, m));
    Ok(())
}
