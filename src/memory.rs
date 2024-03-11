use std::fs::File;
use std::io::{BufReader, BufRead};

use eyre::Result;

pub struct MemoryUsage {
  pub mem_avail: u64,
  pub swap_free: Option<u64>,
}

pub fn get_memory_usage() -> Result<MemoryUsage> {
  let f = File::open("/proc/meminfo")?;
  let r = BufReader::new(f);

  let mut mem_avail = 0;
  let mut swap_total = 0;
  let mut swap_free = 0;
  for line in r.lines() {
    let line = line?;
    if line.starts_with("MemAvailable:") {
      mem_avail = line.split_whitespace().nth(1).unwrap().parse()?;
    } else if line.starts_with("SwapTotal:") {
      swap_total = line.split_whitespace().nth(1).unwrap().parse()?;
    } else if line.starts_with("SwapFree:") {
      swap_free = line.split_whitespace().nth(1).unwrap().parse()?;
    }
  }

  Ok(MemoryUsage {
    mem_avail,
    swap_free: if swap_total > 0 { Some(swap_free) } else { None },
  })
}

