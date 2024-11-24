use std::thread::sleep;
use std::time::Duration;

use eyre::Report;

use graphite_client::LocalGraphite;

mod memory;
mod psi;
mod cpu;
mod load;

fn timestamp() -> u64 {
  use std::time::{SystemTime, UNIX_EPOCH};
  let now = SystemTime::now();
  let elapsed = now.duration_since(UNIX_EPOCH).unwrap();
  elapsed.as_secs()
}

struct State {
  last_cpu_info: Vec<u64>,
  last_cpu_total: f32,
  cpu_count: f32,
}

fn one_run(
  graphite: &mut LocalGraphite,
  host: &str,
  state: &mut State,
) -> Result<(), Report> {
  let usage = memory::get_memory_usage()?;
  let psi = psi::get_psi_info()?;
  let cpu_info = cpu::get_cpu_time()?;
  let load = load::get_system_load()?;
  let t = timestamp();

  let mut msgs = Vec::new();

  msgs.push(format!("stats.{host}.memory.used {} {t}", usage.used));
  msgs.push(format!("stats.{host}.memory.avail {} {t}", usage.avail));
  msgs.push(format!("stats.{host}.memory.cached {} {t}", usage.cached));
  msgs.push(format!("stats.{host}.memory.buffered {} {t}", usage.buffered));
  msgs.push(format!("stats.{host}.memory.slab_recl {} {t}", usage.slab_recl));
  msgs.push(format!("stats.{host}.memory.slab_unrecl {} {t}", usage.slab_unrecl));
  if let Some(swap) = usage.swap_free {
    msgs.push(format!("stats.{host}.swap_free {swap} {t}"));
  }

  msgs.push(format!("stats.{host}.psi.cpu {} {t}", psi.cpu));
  msgs.push(format!("stats.{host}.psi.memory {} {t}", psi.memory));
  msgs.push(format!("stats.{host}.psi.io {} {t}", psi.io));
  msgs.push(format!("stats.{host}.psi.irq {} {t}", psi.irq));

  {
    let mut cpu_msgs = Vec::with_capacity(cpu_info.len());
    let mut skip = false;

    let total = cpu_info.iter().sum::<u64>() as f32;
    for (i, name) in cpu::FIELD_NAMES.iter().enumerate() {
      let value = (cpu_info[i] - state.last_cpu_info[i]) as f32
        * 100.0 * state.cpu_count
        / (total - state.last_cpu_total);
      if value < 0.0 {
        // iowait can decrease after resume from suspend
        skip = true;
        break;
      }
      cpu_msgs.push(format!("stats.{host}.cpu.{name} {value} {t}"));
    }
    if !skip {
      msgs.extend(cpu_msgs);
    }
    state.last_cpu_info = cpu_info;
    state.last_cpu_total = total;
  }

  msgs.push(format!("stats.{host}.load.shortterm {} {t}", load.shortterm));
  msgs.push(format!("stats.{host}.load.midterm {} {t}", load.midterm));
  msgs.push(format!("stats.{host}.load.longterm {} {t}", load.longterm));

  graphite.send_stats(&msgs);

  Ok(())
}

fn main() {
  let mut graphite = LocalGraphite::new_localhost().unwrap();
  let host = uname::Info::new().unwrap().nodename.replace('.', "_");

  let cpu_info = cpu::get_cpu_time().unwrap();
  let total = cpu_info.iter().sum::<u64>() as f32;
  let mut state = State {
    last_cpu_info: cpu_info,
    last_cpu_total: total,
    cpu_count: num_cpus::get() as f32,
  };

  loop {
    sleep(Duration::from_secs(9));
    if let Err(e) = one_run(&mut graphite, &host, &mut state) {
      eprintln!("Error: {e:?}");
    }
  }
}
