pub mod gpu;
pub mod shader;

use std::fs::File;
use std::io::{self, Read, BufReader};

#[derive(Debug)]
pub struct BenchmarkData {
    pub n: u64,
    pub size: u64,
    pub a_sl: u64,
    pub b_sl: u64,
    pub array_a: Vec<Vec<i64>>,
    pub array_b: Vec<Vec<i64>>,
}

impl BenchmarkData {
    pub fn compute_expected_results(&self) -> Vec<Vec<i64>> {
        let mut results = Vec::new();
        for i in 0..self.size {
            let mut result = Vec::new();
            for j in 0..self.n {
                let sum = self.array_a[i as usize][j as usize] + self.array_b[i as usize][j as usize];
                result.push(sum);
            }
            results.push(result);
        }
        results
    }
}

pub fn read_benchmark_data(filename: &str) -> io::Result<BenchmarkData> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    
    // Read parameters
    let n = read_u64(&mut reader)?;
    let size = read_u64(&mut reader)?;
    let a_sl = read_u64(&mut reader)?;
    let b_sl = read_u64(&mut reader)?;
    
    // Read input array A
    let mut array_a = Vec::new();
    for _ in 0..size {
        let mut row = Vec::new();
        for _ in 0..n {
            let val = read_i64(&mut reader)?;
            row.push(val);
        }
        array_a.push(row);
    }
    
    // Read input array B
    let mut array_b = Vec::new();
    for _ in 0..size {
        let mut row = Vec::new();
        for _ in 0..n {
            let val = read_i64(&mut reader)?;
            row.push(val);
        }
        array_b.push(row);
    }
    
    Ok(BenchmarkData {
        n,
        size,
        a_sl,
        b_sl,
        array_a,
        array_b,
    })
}

fn read_u64<R: Read>(reader: &mut R) -> io::Result<u64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

fn read_i64<R: Read>(reader: &mut R) -> io::Result<i64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(i64::from_le_bytes(buf))
}
