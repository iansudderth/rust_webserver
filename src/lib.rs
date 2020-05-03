use std::thread;
use std::sync::{Arc, mpsc, Mutex};
use std::fmt;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPoolCreatedWithInvalidSize;

impl fmt::Debug for ThreadPoolCreatedWithInvalidSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ ThreadPool created with an invalid size.  file: {}, line: {} }}",
            file!(),
            line!()
        )
    }
}

pub enum ThreadPoolCreationError {
    ThreadPoolCreatedWithInvalidSize,
}

impl fmt::Display for ThreadPoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ThreadPoolCreationError::*;

        match *self {
            ThreadPoolCreatedWithInvalidSize => {
                write!(f, "ThreadPool was initialized with invalid size.")
            }
        }
    }
}

impl fmt::Debug for ThreadPoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ThreadPoolCreationError::*;

        match *self {
            ThreadPoolCreatedWithInvalidSize => write!(
                f,
                "{{ ThreadPool created with an invalid size.  file: {}, line: {} }}",
                file!(),
                line!()
            ),
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, ThreadPoolCreationError> {
        if size < 1 {
            return Err(ThreadPoolCreationError::ThreadPoolCreatedWithInvalidSize);
        }
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) =mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        Ok(ThreadPool{workers, sender})
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}


impl Drop for ThreadPool {
    fn drop(&mut self) {
            println!("Sending termination message to all workers");

            for _ in &self.workers {
                self.sender.send(Message::Terminate).unwrap();
            }

            println!("Shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job!", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} set to terminate", id);

                    break;
                }
            }



            
        });
        Worker{ id, thread:Some(thread) }
    }
}

enum Message {
    NewJob(Job),
    Terminate
}