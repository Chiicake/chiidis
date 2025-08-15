use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>,
}

impl ThreadPool{
    pub fn new(thread_num: u8) -> ThreadPool{
        let mut workers: Vec<Worker> = Vec::new();
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..thread_num{
            workers.push(Worker::new(i, receiver.clone()));
        }

        ThreadPool{ workers,sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}


struct Worker{
    id: u8,
    thread: thread::JoinHandle<()>,
}

impl Worker{
    fn new(id: u8, receiver :Arc<Mutex<Receiver<Task>>>) -> Worker{
        let th = thread::spawn(move ||{
            loop{
                let job = receiver.lock().unwrap().recv().unwrap();
                job();
            }
        });
        Worker{id, thread: th}
    }
}

type Task = Box<dyn FnOnce() + Send + 'static>;
