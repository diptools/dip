use std::future::Future;
use tokio::{runtime::Runtime, sync::mpsc};

pub struct AsyncActionPool<Action> {
    runner: Runtime,
    tx: mpsc::Sender<Action>,
}

impl<Action> AsyncActionPool<Action> {
    pub fn new(tx: mpsc::Sender<Action>) -> Self {
        Self {
            runner: Runtime::new().unwrap(),
            tx,
        }
    }

    pub fn send<F>(&self, future: F)
    where
        F: Future<Output = Action> + Send + 'static,
        F::Output: Send + 'static + std::fmt::Debug,
    {
        let tx = self.tx.clone();

        self.runner.spawn(async move {
            let task = future.await;
            tx.send(task).await.unwrap();
        });
    }
}

pub type NoAsyncAction = ();
