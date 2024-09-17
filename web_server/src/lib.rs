use std::thread;
use std::sync::{mpsc, Arc, Mutex};

struct Worker {
    id: usize,
    // we use an Option so that when we shut down a thread we can
    // take the value out of the Some variant and replace it with None
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || {
            loop {
                let msg = receiver.lock().unwrap().recv();
                match msg {
                    Ok(job) => {
                        println!("Worker {id} got a job, executing...");
                        job();
                    },
                    Err(_) => {
                        println!("Worker {id} disconnected, shutting down...");
                        break
                    }
                }
            }
        });
        Self {
            id,
            handle: Some(handle),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    // we use an option so the sender can be dropped while the
    // ThreadPool is still in scope
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send>;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// # Parameters
    /// size - number of threads in the pool
    /// 
    /// # Panics
    /// If size is 0
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        // each Worker needs ownership of the receiver
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Self {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // dropping the sender closes the channel - calls to recv will now err
        drop(self.sender.take());
        for worker in self.workers.iter_mut() {
            println!("Shutting down worker {}", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            };
        }
    }
}