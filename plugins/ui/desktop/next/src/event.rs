/// Tao events that emit from UI side
#[derive(Debug)]
pub enum UiEvent
// <UiAction: Debug, AsyncAction>
{
    /// UI events regards window manipulation
    WindowEvent(WindowEvent),
    // /// User defined UiAction coming from Ui
    // UiAction(UiAction),
    // /// KeyboardEvent which dispatched from `window.document`. Make sure to pass `keyboard_event:
    // /// true` to `DioxusSettings`.
    // KeyboardEvent(KeyboardEvent),
    // /// User defined AsyncAction
    // AsyncAction(AsyncAction),
}
