+++
title = "Overview"
weight = 0
+++

## Channels
```rust
let (vdom_scheduler_tx, vdom_scheduler_rx) = mpsc::unbounded::<SchedulerMsg>();
let (global_state_tx, global_state_rx) = channel::<GlobalState>(8);
let (core_tx, core_rx) = channel::<CoreCommand>(8);
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

    Systems ->> EventLoop: global_state.send()
    EventLoop ->> VirtualDom: global_state_tx.send()
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
    Note left of VirtualDom: GlobalState
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

### CoreCommand

```mermaid
sequenceDiagram
    participant WebView
    participant Window
    participant Plugin
    participant EventLoop
    participant Systems
    participant VirtualDom

    Window ->> Plugin: window.send(cmd)
    Plugin ->> EventLoop: proxy.send_event(UiEvent::CoreCommand(cmd));
    EventLoop ->> EventLoop: MainEventsCleared
    EventLoop ->> Systems: app.update()
    Note right of Systems: apply_globao_state_command
    Systems ->> VirtualDom: global_state.try_send(state);
    Note right of VirtualDom: apply_edits()
    Note right of VirtualDom: rerender()
    VirtualDom ->> EventLoop: Event::UserEvent(WindowEvent::Rerender)
    EventLoop ->> Window: dioxus_window.rerender()
    Window ->> WebView: webviwe.evaluate_script()

    EventLoop ->> EventLoop: RedrawRequested
    EventLoop ->> EventLoop: RedrawEventsCleared
```
