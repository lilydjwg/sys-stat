use std::fs;
use std::io::{self, BufRead};

use eyre::Result;

fn get_one_psi(which: &str) -> Result<f32> {
  let f = fs::File::open(format!("/proc/pressure/{which}"))?;
  let mut f = io::BufReader::new(f);
  let mut line = String::new();
  f.read_line(&mut line)?;
  Ok(line.split(' ').nth(1).unwrap().split('=').nth(1).unwrap().parse()?)
}

pub struct PsiInfo {
  pub memory: f32,
  pub cpu: f32,
  pub io: f32,
  pub irq: f32,
}

pub fn get_psi_info() -> Result<PsiInfo> {
  Ok(PsiInfo {
    cpu: get_one_psi("cpu")?,
    memory: get_one_psi("memory")?,
    io: get_one_psi("io")?,
    irq: get_one_psi("irq")?,
  })
}
