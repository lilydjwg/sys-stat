use std::fs::File;
use std::io::{BufReader, BufRead};

use eyre::Result;

pub const FIELD_NAMES: &[&str] = &[
  "user", "nice", "system", "idle", "iowait", "irq", "softirq",
  "steal", "guest", "guest_nice",
];

pub fn get_cpu_time() -> Result<Vec<u64>> {
  let f = File::open("/proc/stat")?;
  let mut f = BufReader::new(f);
  let mut line = String::new();
  f.read_line(&mut line)?;
  let numbers = line.split_ascii_whitespace().skip(1) // cpu
    .map(|i| i.parse().unwrap())
    .collect();
  Ok(numbers)
}

