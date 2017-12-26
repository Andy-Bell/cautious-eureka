use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;


pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<super::Message>>>) ->
        Worker {

            let thread = thread::spawn(move ||{
                loop {
                    let message = receiver.lock().unwrap().recv().unwrap();

                    match message {
                        super::Message::NewJob(job) => {
                            println!("Worker {} got a job; executing.", id);

                            job.call_box();
                        },
                        super::Message::Terminate => {
                            println!("Worker {} was told to terminate.", id);

                            break;
                        },
                    }
                }
            });

            Worker {
                id,
                thread: Some(thread),
            }
        }
}
