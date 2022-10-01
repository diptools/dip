use dip::prelude::*;
use std::collections::HashMap;
use tokio::{runtime::Runtime, sync::mpsc};

fn main() {
    App::new()
        .set_runner(|mut app| {
            let (tx, mut rx) = mpsc::channel::<FetchResult>(8);
            let task_pool = AsyncTaskPool::new(tx.clone());

            app.world.insert_resource(task_pool);
            app.update();

            let mut event = app.world.get_resource_mut::<Events<FetchResult>>().unwrap();
            Runtime::new().unwrap().block_on(async move {
                while let Some(res) = rx.recv().await {
                    event.send(res);

                    break;
                }
            });

            app.update();
        })
        .add_event::<FetchResult>()
        .add_startup_system(fetch)
        .add_system(log_fetch_result)
        .run();
}

pub struct AsyncTaskPool {
    runner: Runtime,
    tx: mpsc::Sender<FetchResult>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct FetchResult {
    value: HashMap<String, String>,
}

impl AsyncTaskPool {
    fn new(tx: mpsc::Sender<FetchResult>) -> Self {
        Self {
            runner: Runtime::new().unwrap(),
            tx,
        }
    }
}

fn fetch(task_pool: Res<AsyncTaskPool>) {
    let tx = task_pool.tx.clone();

    task_pool.runner.spawn(async move {
        let res = reqwest::get("https://httpbin.org/ip")
            .await
            .unwrap()
            .json::<HashMap<String, String>>()
            .await
            .unwrap();

        tx.send(FetchResult { value: res }).await.unwrap();
    });
}

fn log_fetch_result(mut events: EventReader<FetchResult>) {
    for res in events.iter() {
        println!("{res:#?}");
    }
}
