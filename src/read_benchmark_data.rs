use std::env;
use std::io;
use spqlios_arith_gpu::read_benchmark_data;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <benchmark_data.bin>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    let data = read_benchmark_data(filename)?;
    
    println!("Parameters:");
    println!("  n = {}", data.n);
    println!("  size = {}", data.size);
    println!("  a_sl = {}", data.a_sl);
    println!("  b_sl = {}", data.b_sl);
    
    println!("\nArray A:");
    for (i, row) in data.array_a.iter().enumerate() {
        println!("  Vector {}: {:?}", i, row);
    }
    
    println!("\nArray B:");
    for (i, row) in data.array_b.iter().enumerate() {
        println!("  Vector {}: {:?}", i, row);
    }
    
    let expected_results = data.compute_expected_results();
    println!("\nExpected results (A + B):");
    for (i, result) in expected_results.iter().enumerate() {
        println!("  Vector {}: {:?}", i, result);
    }
    
    Ok(())
}

