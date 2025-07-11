use anyhow::Result;
use lakeshire_core::screenshot::ScreenGrabber;
use std::time::Duration;

pub fn bench_command(count: u32) -> Result<()> {
    let mut grabber = ScreenGrabber::default();

    println!("Benchmarking {} screenshots", count);
    let durations: Result<Vec<Duration>> = (0..count)
        .map(|_| {
            let t0 = std::time::Instant::now();
            let _ = grabber.get_screenshot()?;
            let t1 = std::time::Instant::now();
            let duration = t1.duration_since(t0);
            Ok(duration)
        })
        .collect();
    let avg_duration = durations?.iter().sum::<Duration>() / count;
    println!("Average time taken:: {:?}", avg_duration);

    Ok(())
}
