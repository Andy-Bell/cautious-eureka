use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use error_handler::error_handler;


pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<super::Message>>>
        ) -> Worker {

            let thread = thread::spawn(move ||{
                loop {
                    let message1 = error_handler(receiver.lock(),"there was a proplem locking the receiver: {:?}");

                    let message2 = message1.recv();
                    let message2 = match message2 {
                        Ok(message) => message,
                        Err(error) => {
                            panic!("The sending channel has closed and can not be used: {:?}", error);
                        },
                    };

                    match message2 {
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
