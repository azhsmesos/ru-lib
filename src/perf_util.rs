use chrono::{DateTime, FixedOffset, Utc};
use perf_monitor::cpu::{processor_numbers, ProcessStat, ThreadStat};
use perf_monitor::fd::fd_count_cur;
use perf_monitor::io::get_process_io_stats;
use perf_monitor::mem::get_process_memory_info;
use tabled::{Tabled, Table};

pub fn default() {
    let beijing_time_zone = FixedOffset::east_opt(8 * 3600).unwrap();
    let now = Utc::now();
    let beijing_time = now.with_timezone(&beijing_time_zone);

    let core_num = processor_numbers().unwrap();
    let mut stat_p = ProcessStat::cur().unwrap();
    let mut stat_t = ThreadStat::cur().unwrap();
    let _ = (0..1_000).into_iter().sum::<i128>();

    let usage_p = stat_p.cpu().unwrap() * 100f64;
    let usage_t = stat_t.cpu().unwrap() * 100f64;
    // mem
    let mem_info = get_process_memory_info().unwrap();
    // fd
    let fd_num = fd_count_cur().unwrap();
    // io
    let io_stat = get_process_io_stats().unwrap();

    let cpu_info = format!("core Number: {}\nprocess usage: {:.2}%\ncurrent thread usage: {:.2}%", core_num, usage_p, usage_t);
    let mem_info = format!("memory used: {} bytes({} mb)\nvirtural memory used: {} bytes", mem_info.resident_set_size,
                           mem_info.resident_set_size / 1024, mem_info.virtual_memory_size);
    let fd_info = format!("fd number: {}", fd_num);
    let io_info = format!("io-in: {} bytes({} mb),\nio-out: {} bytes({} mb)", io_stat.read_bytes, io_stat.read_bytes / 1024,
                          io_stat.write_bytes, io_stat.write_bytes / 1024);


    let infos = vec![
        PerfInfo {
            name: "CPU",
            desc: cpu_info,
            perf_time: beijing_time,
        },
        PerfInfo {
            name: "MEM",
            desc: mem_info,
            perf_time: beijing_time,
        },
        PerfInfo {
            name: "FD",
            desc: fd_info,
            perf_time: beijing_time,
        },
        PerfInfo {
            name: "IO",
            desc: io_info,
            perf_time: beijing_time,
        }
    ];

    let table = Table::new(infos);

    tracing::info!("\n{}\n", table);
}

#[derive(Tabled)]
struct PerfInfo {
    name: &'static str,
    desc: String,
    perf_time: DateTime<FixedOffset>,
}