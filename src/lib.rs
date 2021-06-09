use std::sync::{
        mpsc,
        Arc,
        Mutex,
};
use std::thread;

pub struct Result
{
    pub runtime         :      f32,
    pub status          :      Status,
    pub output          :      String,
}

pub enum Status
{
    Success,
    Failure
}

pub struct ThreadPool
{
    workers         :       Vec<Worker>,
    sender          :       mpsc::Sender<Message>,
    pub resReceiver     :       mpsc::Receiver<Result>,
}

type Job = Box<dyn FnOnce(usize,&mpsc::Sender<Result>) + Send + Sync + 'static>;

enum Message
{
    NewJob(Job),
    Terminate,
}

impl ThreadPool
{
    pub fn new(size: usize) ->  ThreadPool
    {
        let (sender, receiver)  =   mpsc::channel();
        let receiver    =   Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        let (resSender, resReceiver) = mpsc::channel();

        for id in 0..size
        {
            workers.push(Worker::new(id,Arc::clone(&receiver),resSender.clone()));
        }

        ThreadPool
        {
            workers,
            sender,
            resReceiver
        }
    }

    pub fn execute<F>(&self, f:F)
    where F: FnOnce(usize,&mpsc::Sender<Result>) + Send + Sync + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool
{
    fn drop(&mut self)
    {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
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

unsafe impl Sync for ThreadPool{}

struct Worker
{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker
{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>, resSender: mpsc::Sender<Result>) -> Worker
    {
        let thread = thread::spawn(move ||
            loop
            {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message
                {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job", id);
                        job(id,&resSender);
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        );

        Worker
        {
            id,
            thread: Some(thread),
        }
    }
}
