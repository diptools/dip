#![allow(non_snake_case)]

use crate::context::UiContext;
use bevy_dioxus_core::global_state::GlobalStateHandler;
use dioxus_core::{Component, SchedulerMsg, ScopeId, VirtualDom as DioxusVirtualDom};
use dioxus_hooks::{UnboundedReceiver, UnboundedSender};
use fermi::AtomRoot;
use futures_intrusive::channel::shared::Receiver;
use std::{
    fmt::Debug,
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tokio::select;

pub struct VirtualDom<GlobalState: 'static, CoreCommand> {
    virtual_dom: DioxusVirtualDom,
    edit_queue: Arc<Mutex<Vec<String>>>,
    global_state_rx: Receiver<GlobalState>,
    scheduler_tx: UnboundedSender<SchedulerMsg>,
    core_cmd_type: PhantomData<CoreCommand>,
}

impl<GlobalState, CoreCommand> VirtualDom<GlobalState, CoreCommand>
where
    GlobalState: GlobalStateHandler,
    CoreCommand: 'static + Clone + Debug,
{
    pub fn new<Props>(
        Root: Component<Props>,
        props: Props,
        edit_queue: Arc<Mutex<Vec<String>>>,
        (scheduler_tx, scheduler_rx): (
            UnboundedSender<SchedulerMsg>,
            UnboundedReceiver<SchedulerMsg>,
        ),
        global_state_rx: Receiver<GlobalState>,
    ) -> Self
    where
        Props: 'static,
    {
        let virtual_dom = DioxusVirtualDom::new_with_props_and_scheduler(
            Root,
            props,
            (scheduler_tx.clone(), scheduler_rx),
        );

        Self {
            virtual_dom,
            edit_queue,
            global_state_rx,
            scheduler_tx,
            core_cmd_type: PhantomData,
        }
    }

    pub async fn run(&mut self) {
        // apply initial edit
        let initial_muts = self.virtual_dom.rebuild();
        self.edit_queue
            .lock()
            .unwrap()
            .push(serde_json::to_string(&initial_muts.edits).unwrap());
        self.rerender();

        loop {
            // wait for either
            select! {
                // 1) pull for work
                () = self.virtual_dom.wait_for_work() => {
                    log::trace!("New task");
                    self.apply_edits();

                    if !self.edit_queue.lock().unwrap().is_empty() {
                        self.rerender();
                    }
                }
                // 2) when global state is changed or injected window.document event is emitted
                state = self.global_state_rx.receive() => {
                    if let Some(state) = state {
                        log::trace!("GlobalState");
                        let root = self.atom_root();
                        state.handler(root.clone());

                        self.scheduler_tx.start_send(SchedulerMsg::NewTask(ScopeId(0))).unwrap();
                    }
                }
            }
        }
    }

    pub fn provide_ui_context(&self, context: UiContext<CoreCommand>)
    where
        CoreCommand: Clone + Debug,
    {
        self.virtual_dom.base_scope().provide_context(context);
    }

    fn atom_root(&self) -> Rc<AtomRoot> {
        let cx = self.virtual_dom.base_scope();
        match cx.consume_context::<Rc<AtomRoot>>() {
            Some(root) => root,
            None => cx.provide_root_context(Rc::new(AtomRoot::new(cx.schedule_update_any()))),
        }
    }

    fn apply_edits(&mut self) {
        let muts = self.virtual_dom.work_with_deadline(|| false);
        for edit in muts {
            self.edit_queue
                .lock()
                .unwrap()
                .push(serde_json::to_string(&edit.edits).unwrap());
        }
    }

    fn rerender(&self) {
        let ui_context: UiContext<CoreCommand> =
            self.virtual_dom.base_scope().consume_context().unwrap();
        ui_context.rerender();
    }
}
