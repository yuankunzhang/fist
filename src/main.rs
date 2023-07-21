use fist::Fist;

fn main() {
    let fist = Fist::new();
    let files = fist.run();
    for file in files {
        println!("{}", file.to_string());
    }
    // let me = procfs::process::Process::myself().unwrap();
    // let me_stat = me.stat().unwrap();
    // let tps = procfs::ticks_per_second();

    // println!("{: >5} {: <8} {: >8} {}", "PID", "TTY", "TIME", "CMD");

    // let tty = format!("pty/{}", me_stat.tty_nr().1);
    // for prc in procfs::process::all_processes().unwrap() {
    //     let prc = prc.unwrap();
    //     let stat = prc.stat().unwrap();
    //     if stat.tty_nr == me_stat.tty_nr {
    //         // total_time is in seconds
    //         let total_time = (stat.utime + stat.stime) as f32 / (tps as f32);
    //         println!(
    //             "{: >5} {: <8} {: >8} {}",
    //             stat.pid, tty, total_time, stat.comm
    //         );
    //     }
    // }
}
