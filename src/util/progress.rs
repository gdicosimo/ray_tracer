use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

pub struct Progress {
    count: Arc<AtomicUsize>,
    completed: Arc<AtomicBool>,
    start_time: Instant,
}

impl Progress {
    pub fn new(total: usize) -> Self {
        let count = Arc::new(AtomicUsize::new(0));
        let completed = Arc::new(AtomicBool::new(false));
        let start_time = Instant::now();

        let total_clone = total;

        thread::spawn({
            let count = Arc::clone(&count);
            let completed = Arc::clone(&completed);
            move || {
                let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
                let mut frame_idx = 0;

                while !completed.load(Ordering::Relaxed) {
                    let current = count.load(Ordering::Relaxed);
                    let progress = (current as f32 / total_clone as f32 * 100.0).ceil() as u32;

                    print!(
                        "\r\x1B[K{} \x1B[34mRendering\x1B[0m \x1B[36m{:>3}%\x1B[0m",
                        spinner_frames[frame_idx % spinner_frames.len()],
                        progress.min(100)
                    );
                    io::stdout().flush().unwrap();

                    frame_idx += 1;
                    thread::sleep(Duration::from_millis(80));
                }
            }
        });

        Self {
            count,
            completed,
            start_time,
        }
    }

    pub fn inc(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn finish(&self) {
        self.completed.store(true, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(120));

        let elapsed = self.start_time.elapsed();
        print!(
            "\r\x1B[K\x1B[32m✓\x1B[0m \x1B[34mRendering done\x1B[0m in \x1B[33m{:.2}s\x1B[0m",
            elapsed.as_secs_f64()
        );
        io::stdout().flush().unwrap();
    }
}
