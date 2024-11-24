use eyre::{Result, bail};

pub struct SystemLoad {
  pub shortterm: f32,
  pub midterm: f32,
  pub longterm: f32,
}

pub fn get_system_load() -> Result<SystemLoad> {
  let content = std::fs::read_to_string("/proc/loadavg")?;
  let parts: Vec<&str> = content.split_whitespace().collect();

  if parts.len() < 3 {
    bail!("Unexpected format in /proc/loadavg");
  }

  let shortterm = parts[0].parse()?;
  let midterm = parts[1].parse()?;
  let longterm = parts[2].parse()?;

  Ok(SystemLoad {
    shortterm,
    midterm,
    longterm,
  })
}
