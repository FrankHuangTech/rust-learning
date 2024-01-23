use std::thread;
use std::sync::{Arc, mpsc, Mutex};


pub struct ThreadPool{
    walkers: Vec<Walker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool{
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);

        let (sender,receiver) = mpsc::channel();
        let recv = Arc::new(Mutex::new(receiver));

        let mut walkers = Vec::with_capacity(size);

        for id in 0..size {
            walkers.push(Walker::new(id, Arc::clone(&recv)));
        }

        ThreadPool{walkers,sender}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}


impl Drop for ThreadPool{
    fn drop(&mut self) {
        for w in &mut self.walkers  {
            println!("Shutdown job {}",w.id);
            if let Some(thread) = w.thread.take(){
                thread.join().unwrap()
            }

        }
    }
}

struct Walker{
    id:usize,
    thread:Option<thread::JoinHandle<()>>,
}

impl Walker{
    pub fn new(id:usize,receiver: Arc<Mutex<mpsc::Receiver<Job>>>)->Walker{
        let thread = thread::spawn(move||loop {

            // let job = receiver.lock().unwrap().recv().unwrap();
            while let Ok (job)=receiver.lock().unwrap().recv(){
                println!("Job[{id}] execute....");
                job()
            }

            // let message =   receiver.lock().unwrap().recv();
            // match message {
            //     Ok(job)=>{
            //         println!("Job[{id}] execute....");
            //         job()
            //     }
            //     Err(_)=>{
            //         println!("Worker {id} disconnected; shutting down.");
            //         break;
            //     }
            // }
        });

        Walker{id,thread:Some(thread) }
    }
}