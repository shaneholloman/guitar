use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

pub struct Spinner {
    pub char_state: Arc<Mutex<char>>,
    pub running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Spinner {
    pub fn new() -> Self {
        Self { char_state: Arc::new(Mutex::new('|')), running: Arc::new(AtomicBool::new(false)), handle: None }
    }

    pub fn start(&mut self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(true, Ordering::SeqCst);
        let spinner = Arc::clone(&self.char_state);
        let running = Arc::clone(&self.running);

        self.handle = Some(thread::spawn(move || {
            let frames = ['⠋', '⠙', '⠸', '⠴', '⠦', '⠇'];
            let mut i = 0;
            while running.load(Ordering::SeqCst) {
                {
                    let mut c = spinner.lock().unwrap();
                    *c = frames[i];
                }
                i = (i + 1) % frames.len();
                thread::sleep(Duration::from_millis(200));
            }
        }));
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            // Join the worker so the spinner cannot outlive the App state it reports on.
            handle.join().unwrap();
        }
    }

    pub fn get_char(&self) -> char {
        *self.char_state.lock().unwrap()
    }

    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
}
