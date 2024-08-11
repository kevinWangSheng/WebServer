use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::thread::{JoinHandle, Thread};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool{
    threads:Vec<Worker>,
    sender: mpsc::Sender<Message>
}
enum Message {
    NewJob(Job),
    Terminate,
}
struct Worker{
    id:usize,
    thread:Option<thread::JoinHandle<()>>
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0);
        let (sender,recv) = mpsc::channel();
        let receive = Arc::new(Mutex::new(recv));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i,Arc::clone(&receive)));
        }
        ThreadPool{
            threads:workers,
            sender:sender
        }
    }
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }

    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static{
        thread::spawn(f)
    }

}



impl Worker{
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Message>>>)->Worker{
        // let thread = thread::spawn(move ||{
        //     while let Ok(job) = receiver.lock().unwrap().recv(){
        //         println!("Worker {} got a job; executing.", id);
        //         job();
        //     }
        // });

        let thread = thread::spawn(move || {
            loop {
                let message= receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });
        Worker{
            id,
            thread:Some(thread),
        }
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        for _ in self.threads.iter_mut(){
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);
            // if let Some(thread) = worker.thread.take() {
            //     thread.join().unwrap();
            // }

            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}