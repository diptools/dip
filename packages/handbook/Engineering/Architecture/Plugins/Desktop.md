
## Channels
```rust
let (vdom_scheduler_tx, vdom_scheduler_rx) = futures_channel::mpsc::unbounded::<SchedulerMsg>();
let (ui_state_tx, ui_state_rx) = tokio::sync::mpsc::channel::<UiState>(8);
let (ui_action_tx, ui_action_rx) = tokio::sync::mpsc::channel::<UiAction>(8);
let proxy = event_loop.create_proxy();
```

```mermaid
sequenceDiagram
    participant WebView
    participant Window
    participant VirtualDom
    participant EventLoop
    participant Systems

    WebView ->> Window: IpcMessage
    Window ->> VirtualDom: vdom_scheduler_tx.send()
    VirtualDom ->> EventLoop: proxy.send()
    EventLoop ->> Systems: app.update()

    Systems ->> EventLoop: ui_state.send()
    EventLoop ->> VirtualDom: ui_state_tx.send()
    VirtualDom ->> Window: dioxus_window.rerender()
    Window ->> WebView: webviwe.evaluate_script()
```

## Render cycle

```mermaid
sequenceDiagram
    participant VirtualDom
    participant EventLoop
    participant Systems
    
    VirtualDom ->> VirtualDom: Wait For Work
    Note left of VirtualDom: UiState
    Note left of VirtualDom: rerender();
    VirtualDom ->> EventLoop: Event::UserEvent(WindowEvent::Rerender)
    EventLoop ->> EventLoop: NewEvents(Init)
    EventLoop ->> EventLoop: MainEventsCleared
    EventLoop ->> Systems: app.update()

    EventLoop ->> EventLoop: RedrawRequested
    EventLoop ->> EventLoop: RedrawEventsCleared
```

### When user clicks screen

```mermaid
sequenceDiagram
    participant Window
    participant EventLoop
    participant Systems

    Note left of Window: User Click
    Window ->> EventLoop: Event::DeviceEvent
    Note left of EventLoop: User Click
    EventLoop ->> Systems: app.update()
```

### UiAction

```mermaid
sequenceDiagram
    participant WebView
    participant Window
    participant Plugin
    participant EventLoop
    participant Systems
    participant VirtualDom

    Window ->> Plugin: window.send(action)
    Plugin ->> EventLoop: proxy.send_event(UiEvent::UiAction(action));
    EventLoop ->> EventLoop: MainEventsCleared
    EventLoop ->> Systems: app.update()
    Note right of Systems: apply_globao_state_command
    Systems ->> VirtualDom: ui_state.try_send(state);
    Note right of VirtualDom: apply_edits()
    Note right of VirtualDom: rerender()
    VirtualDom ->> EventLoop: Event::UserEvent(WindowEvent::Rerender)
    EventLoop ->> Window: dioxus_window.rerender()
    Window ->> WebView: webviwe.evaluate_script()

    EventLoop ->> EventLoop: RedrawRequested
    EventLoop ->> EventLoop: RedrawEventsCleared
```