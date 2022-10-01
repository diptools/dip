use dip::prelude::*;
use serde::Deserialize;
use std::future::Future;
use tokio::{runtime::Runtime, sync::mpsc};

fn main() {
    App::new()
        .set_runner(|mut app| {
            let (tx, mut rx) = mpsc::channel::<Response>(8);
            let task_pool = AsyncTaskPool::new(tx.clone());

            app.world.insert_resource(task_pool);
            app.update();

            let mut event = app.world.get_resource_mut::<Events<Response>>().unwrap();
            Runtime::new().unwrap().block_on(async move {
                while let Some(res) = rx.recv().await {
                    event.send(res);

                    break;
                }
            });

            app.update();
        })
        .add_event::<Response>()
        .add_startup_system(fetch)
        .add_system(log_fetch_result)
        .run();
}

pub struct AsyncTaskPool<Response> {
    runner: Runtime,
    tx: mpsc::Sender<Response>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
struct IpResponse {
    origin: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Response {
    IpResponse(IpResponse),
}

impl<Response> AsyncTaskPool<Response> {
    pub fn new(tx: mpsc::Sender<Response>) -> Self {
        Self {
            runner: Runtime::new().unwrap(),
            tx,
        }
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = Response> + Send + 'static,
        F::Output: Send + 'static + std::fmt::Debug,
    {
        let tx = self.tx.clone();

        self.runner.spawn(async move {
            let res = future.await;
            tx.send(res).await.unwrap();
        });
    }
}

fn fetch(task_pool: Res<AsyncTaskPool<Response>>) {
    task_pool.spawn(async move {
        let res = reqwest::get("https://httpbin.org/ip")
            .await
            .unwrap()
            .json::<IpResponse>()
            .await
            .unwrap();

        Response::IpResponse(res)
    });
}

fn log_fetch_result(mut events: EventReader<Response>) {
    for res in events.iter() {
        match res {
            Response::IpResponse(res) => {
                println!("{res:#?}");
            }
        }
    }
}
