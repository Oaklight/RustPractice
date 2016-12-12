use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
#[derive(Clone, Copy, Debug)]
pub enum Action {
    Eat,
    Think,
}

pub struct Philosopher {
}

impl Philosopher {
    pub fn new() -> Philosopher {
        Philosopher{}
    }

    /// Enters a loop, wherein waits for commands from rx, takes actions,
    /// and then replies to tx.
    ///
    /// Initially all the forks are available. The caller guarantees that
    /// every Action::Eat is preceded by Action::Think, and vice versa in
    /// the messages received from rx.
    ///
    /// id: [0..n-1]
    /// forks: n shared forks.
    /// tx: After an action finishes, sends a notification (id, action) to tx.
    /// rx: Receives a command for action from rx.
    pub fn run(self, id: usize, forks: Arc<Vec<Mutex<()>>>,
               tx: Sender<(usize, Action)>, rx: Receiver<Action>) {
        let mut left_fork = id;
        let mut right_fork = (id + 1) % forks.len();
        if left_fork > right_fork {
            let temp = left_fork;
            left_fork = right_fork;
            right_fork = temp;
        }
        /*println!("{} {}", left_fork, right_fork);*/
        loop {
            match rx.recv() {
                Ok(cmd) => {
                    match cmd {
                        Action::Eat => {
                            forks[left_fork].lock().unwrap();
                            forks[right_fork].lock().unwrap();
                            tx.send((id, Action::Eat)).unwrap();
                            thread::sleep(std::time::Duration::from_millis(200));
                        },
                        Action::Think => {
                            tx.send((id, Action::Think)).unwrap();
                        },
                    }
                }
                Err(_) => {thread::sleep(std::time::Duration::from_millis(200));},
            }
        }
    }
}

