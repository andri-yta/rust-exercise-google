use std::sync::mpsc::{self, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Fork {
    id: usize
}

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: SyncSender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        let left = self.left_fork.lock().unwrap();
        let right = self.right_fork.lock().unwrap();
        println!("{} is picking up fork: {:?} & {:?}", &self.name, left.id, right.id);

        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    let (tx, rx) = mpsc::sync_channel(PHILOSOPHERS.len());

    // Create forks
    let mut forks = Vec::new();
    for i in 0..PHILOSOPHERS.len() {
        forks.push(Arc::new(Mutex::new(Fork {id: i})));
    }

    for i in 0..forks.len() {
        let mut left_fork = forks[i].clone();
        let mut right_fork = forks[(i + 1) % forks.len()].clone();

        // From solution:
        // To avoid a deadlock, we have to break the symmetry
        // somewhere. This will swap the forks without deinitializing
        // either of them.
        if i == forks.len() - 1 {
            std::mem::swap(&mut left_fork, &mut right_fork);
        }

        // Create philosophers
        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            left_fork,
            right_fork,
            thoughts: tx.clone(),
        };

        // Make them think and eat
        let _ = thread::spawn(move || {
            let thread_id = thread::current().id();
            for i in 0..1 {
                println!("----- {thread_id:?} spawn: {i} -> {}", philosopher.name);
                philosopher.think();
                philosopher.eat();
            }
            println!("{thread_id:?}: done");
        });
    }

    drop(tx);
    // Output their thoughts
    for thought in rx.iter() {
        println!("Main: got {}", thought);
    }
}
