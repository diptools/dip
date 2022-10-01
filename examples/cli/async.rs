use dip::prelude::*;
use serde::Deserialize;
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
    fn new(tx: mpsc::Sender<Response>) -> Self {
        Self {
            runner: Runtime::new().unwrap(),
            tx,
        }
    }
}

fn fetch(task_pool: Res<AsyncTaskPool<Response>>) {
    let tx = task_pool.tx.clone();

    task_pool.runner.spawn(async move {
        let res = reqwest::get("https://httpbin.org/ip")
            .await
            .unwrap()
            .json::<IpResponse>()
            .await
            .unwrap();

        tx.send(Response::IpResponse(res)).await.unwrap();
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
