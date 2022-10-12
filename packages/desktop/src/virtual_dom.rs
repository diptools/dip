#![allow(non_snake_case)]

use crate::context::UiContext;
use dioxus::{
    fermi::AtomRoot,
    hooks::{UnboundedReceiver, UnboundedSender},
};
use dioxus_core::{Component, SchedulerMsg, ScopeId, VirtualDom as DioxusVirtualDom};
use dip_core::ui_state::UiStateHandler;
use std::{
    fmt::Debug,
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tokio::{select, sync::mpsc::Receiver};

pub struct VirtualDom<UiState: 'static, UiAction, AsyncAction> {
    virtual_dom: DioxusVirtualDom,
    edit_queue: Arc<Mutex<Vec<String>>>,
    ui_state_rx: Receiver<UiState>,
    scheduler_tx: UnboundedSender<SchedulerMsg>,
    ui_action_type: PhantomData<UiAction>,
    async_action_type: PhantomData<AsyncAction>,
}

impl<UiState, UiAction, AsyncAction> VirtualDom<UiState, UiAction, AsyncAction>
where
    UiState: UiStateHandler,
    UiAction: 'static + Clone + Debug,
    AsyncAction: 'static + Clone + Debug,
{
    pub fn new<RootProps>(
        Root: Component<RootProps>,
        root_props: RootProps,
        edit_queue: Arc<Mutex<Vec<String>>>,
        (scheduler_tx, scheduler_rx): (
            UnboundedSender<SchedulerMsg>,
            UnboundedReceiver<SchedulerMsg>,
        ),
        ui_state_rx: Receiver<UiState>,
    ) -> Self
    where
        RootProps: 'static,
    {
        let virtual_dom = DioxusVirtualDom::new_with_props_and_scheduler(
            Root,
            root_props,
            (scheduler_tx.clone(), scheduler_rx),
        );

        Self {
            virtual_dom,
            edit_queue,
            ui_state_rx,
            scheduler_tx,
            ui_action_type: PhantomData,
            async_action_type: PhantomData,
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
                // 2) when Ui state is changed
                state = self.ui_state_rx.recv() => {
                    if let Some(state) = state {
                        log::trace!("UiState");
                        let root = self.atom_root();
                        state.handler(root.clone());

                        self.scheduler_tx.start_send(SchedulerMsg::NewTask(ScopeId(0))).unwrap();
                    }
                }
            }
        }
    }

    pub fn provide_ui_context(&self, context: UiContext<UiAction, AsyncAction>)
    where
        UiAction: Clone + Debug,
        AsyncAction: Clone + Debug,
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
        let ui_context: UiContext<UiAction, AsyncAction> =
            self.virtual_dom.base_scope().consume_context().unwrap();
        ui_context.rerender();
    }
}
