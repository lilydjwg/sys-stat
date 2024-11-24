use std::fs::File;
use std::io::{BufReader, BufRead};

use eyre::Result;

pub struct MemoryUsage {
  pub used: u64,
  pub buffered: u64,
  pub cached: u64,
  pub slab_recl: u64,
  pub slab_unrecl: u64,
  pub avail: u64,
  pub swap_free: Option<u64>,
}

fn parse_memory_value(line: &str) -> Result<u64> {
  let n = line.split_whitespace().nth(1).unwrap().parse()?;
  Ok(n)
}

pub fn get_memory_usage() -> Result<MemoryUsage> {
  let f = File::open("/proc/meminfo")?;
  let r = BufReader::new(f);

  let mut mem_total = 0;
  let mut mem_free = 0;
  let mut avail = 0;
  let mut buffered = 0;
  let mut cached = 0;
  let mut slab_recl = 0;
  let mut slab_unrecl = 0;

  let mut swap_total = 0;
  let mut swap_free = 0;

  for line in r.lines() {
    let line = line?;
    if line.starts_with("MemTotal:") {
      mem_total = parse_memory_value(&line)?;
    } else if line.starts_with("MemFree:") {
      mem_free = parse_memory_value(&line)?;
    } else if line.starts_with("MemAvailable:") {
      avail = parse_memory_value(&line)?;
    } else if line.starts_with("Buffers:") {
      buffered = parse_memory_value(&line)?;
    } else if line.starts_with("Cached:") {
      cached = parse_memory_value(&line)?;
    } else if line.starts_with("SReclaimable:") {
      slab_recl = parse_memory_value(&line)?;
    } else if line.starts_with("SUnreclaim:") {
      slab_unrecl = parse_memory_value(&line)?;

    } else if line.starts_with("SwapTotal:") {
      swap_total = parse_memory_value(&line)?;
    } else if line.starts_with("SwapFree:") {
      swap_free = parse_memory_value(&line)?;
    }
  }

  let used = mem_total - mem_free - buffered - cached - slab_recl - slab_unrecl;

  Ok(MemoryUsage {
    buffered,
    cached,
    slab_recl,
    slab_unrecl,
    used,
    avail,
    swap_free: if swap_total > 0 { Some(swap_free) } else { None },
  })
}

