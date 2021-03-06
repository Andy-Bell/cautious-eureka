use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<super::Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    /// #FIXME Error Handling needed
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    /// Pass a function to a worker to execute
    ///
    /// f is the Function to be passed.
    ///
    /// Panic possible on sender.send returning an error
    /// #FIXME - Error Handling needed
    pub fn execute<F>(&self, f: F)
        where
        F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);

            self.sender.send(super::Message::NewJob(job)).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(super::Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

