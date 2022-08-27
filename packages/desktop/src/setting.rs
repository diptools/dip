//! Resources for configuring usage of the `dioxus (tao/wry)` library.

use bevy::utils::Duration;
use std::{
    fmt::{self, Debug},
    path::PathBuf,
};

use wry::{
    application::window::Window,
    http::{Request as HttpRequest, Response as HttpResponse},
    webview::FileDropEvent,
    Result as WryResult,
};

/// A resource for configuring usage of the `dioxous (tao/wry)` library.
pub struct DioxusSettings<Props> {
    /// Configures how the tao event loop updates while the window is focused.
    pub focused_mode: UpdateMode,
    /// Configures how the tao event loop updates while the window is *not* focused.
    pub unfocused_mode: UpdateMode,

    /// Configures how to handle file drops
    pub file_drop_handler: Option<Box<dyn Fn(&Window, FileDropEvent) -> bool>>,
    /// Custom file loading protocols with pairs of scheme uri string and a handling
    /// closure
    pub protocols: Vec<WryProtocol>,
    // pub pre_rendered: Option<String>,
    // pub event_handler: Option<Box<DynEventHandlerFn>>,
    /// This provide access to DevTools on menu bar
    pub disable_context_menu: bool,
    /// The directory from which assets will be searched in release mode
    pub resource_dir: Option<PathBuf>,
    /// Stores custom JavaScript code to be execute onload
    pub custom_head: Option<String>,
    /// Stores custom index.html to be used instead of the default Dioxus one
    pub custom_index: Option<String>,
    /// Props for Root component
    pub props: Option<Props>,

    /// Enable keyboard event. This will also emit other keyboard related events such as KeyboardInput and ReceivedCaracter.
    pub keyboard_event: bool,
}

type WryProtocol = (
    String,
    Box<dyn Fn(&HttpRequest) -> WryResult<HttpResponse> + 'static>,
);

// type DynEventHandlerFn = dyn Fn(&mut EventLoop<()>, &mut WebView);

impl<Props> Debug for DioxusSettings<Props> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DioxusWindows")
            .field("focused_mode", &self.focused_mode)
            .field("unfocused_mode", &self.unfocused_mode)
            .finish()
    }
}

impl<Props> DioxusSettings<Props>
where
    Props: Default,
{
    /// Configure tao with common settings for a game.
    pub fn game() -> Self {
        DioxusSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
            ..Default::default()
        }
    }

    /// Configure tao with common settings for a desktop application.
    pub fn application() -> Self {
        DioxusSettings {
            focused_mode: UpdateMode::Reactive {
                max_wait: Duration::from_secs(5),
            },
            unfocused_mode: UpdateMode::ReactiveLowPower {
                max_wait: Duration::from_secs(60),
            },

            protocols: Vec::new(),
            file_drop_handler: None,
            // pre_rendered: None,
            // event_handler: None,
            disable_context_menu: !cfg!(debug_assertions),
            resource_dir: None,
            custom_head: None,
            custom_index: None,
            props: Some(Props::default()),

            keyboard_event: false,
        }
    }

    /// Gets the configured `UpdateMode` depending on whether the window is focused or not
    pub fn update_mode(&self, focused: bool) -> &UpdateMode {
        match focused {
            true => &self.focused_mode,
            false => &self.unfocused_mode,
        }
    }

    /// set the directory from which assets will be searched in release mode
    pub fn with_resource_directory(mut self, path: impl Into<PathBuf>) -> Self {
        self.resource_dir = Some(path.into());
        self
    }

    /// Set whether or not the right-click context menu should be disabled.
    pub fn with_disable_context_menu(&mut self, disable: bool) -> &mut Self {
        self.disable_context_menu = disable;
        self
    }

    // pub fn with_prerendered(&mut self, content: String) -> &mut Self {
    //     self.pre_rendered = Some(content);
    //     self
    // }

    // pub fn with_event_handler(
    //     &mut self,
    //     handler: impl Fn(&mut EventLoop<()>, &mut WebView) + 'static,
    // ) -> &mut Self {
    //     self.event_handler = Some(Box::new(handler));
    //     self
    // }

    /// Set a handler closure to process incoming [`FileDropEvent`] of the webview.
    ///
    /// # Blocking OS Default Behavior
    /// Return `true` in the callback to block the OS' default behavior of handling a file drop.
    ///
    /// Note, that if you do block this behavior, it won't be possible to drop files on `<input type="file">` forms.
    /// Also note, that it's not possible to manually set the value of a `<input type="file">` via JavaScript for security reasons.
    pub fn with_file_drop_handler(
        &mut self,
        handler: impl Fn(&Window, FileDropEvent) -> bool + 'static,
    ) -> &mut Self {
        self.file_drop_handler = Some(Box::new(handler));
        self
    }

    /// Register custom file loading protocols with pairs of scheme uri string and a handling
    /// closure.
    ///
    /// The closure takes a url string slice, and returns a two item tuple of a
    /// vector of bytes which is the content and a mimetype string of the content.
    ///
    /// # Warning
    /// Pages loaded from custom protocol will have different Origin on different platforms. And
    /// servers which enforce CORS will need to add exact same Origin header in `Access-Control-Allow-Origin`
    /// if you wish to send requests with native `fetch` and `XmlHttpRequest` APIs. Here are the
    /// different Origin headers across platforms:
    ///
    /// - macOS: `<scheme_name>://<path>` (so it will be `wry://examples` in `custom_protocol` example)
    /// - Linux: Though it's same as macOS, there's a [bug] that Origin header in the request will be
    /// empty. So the only way to pass the server is setting `Access-Control-Allow-Origin: *`.
    /// - Windows: `https://<scheme_name>.<path>` (so it will be `https://wry.examples` in `custom_protocol` example)
    /// - Android: Custom protocol on Android is fixed to `https://tauri.wry/` due to its design and
    /// our approach to use it. On Android, We only handle the scheme name and ignore the closure. So
    /// when you load the url like `wry://assets/index.html`, it will become
    /// `https://tauri.wry/assets/index.html`. Android has `assets` and `resource` path finder to
    /// locate your files in those directories. For more information, see [Loading in-app content](https://developer.android.com/guide/webapps/load-local-content) page.
    /// - iOS: Same as macOS. To get the path of your assets, you can call [`CFBundle::resources_path`](https://docs.rs/core-foundation/latest/core_foundation/bundle/struct.CFBundle.html#method.resources_path). So url like `wry://assets/index.html` could get the html file in assets directory.
    ///
    /// [bug]: https://bugs.webkit.org/show_bug.cgi?id=229034
    pub fn with_custom_protocol<F>(&mut self, name: String, handler: F) -> &mut Self
    where
        F: Fn(&HttpRequest) -> WryResult<HttpResponse> + 'static,
    {
        self.protocols.push((name, Box::new(handler)));
        self
    }

    /// Initialize javascript code when loading new pages. When webview load a new page, this
    /// initialization code will be executed. It is guaranteed that code is executed before
    /// `window.onload`.
    pub fn with_custom_head(&mut self, head: String) -> &mut Self {
        self.custom_head = Some(head);
        self
    }

    /// Use a custom index.html instead of the default Dioxus one.
    ///
    /// Make sure your index.html is valid HTML.
    ///
    /// Dioxus injects some loader code into the closing body tag. Your document
    /// must include a body element!
    pub fn with_custom_index(&mut self, index: String) -> &mut Self {
        self.custom_index = Some(index);
        self
    }
}

impl<Props> Default for DioxusSettings<Props>
where
    Props: Default,
{
    fn default() -> Self {
        Self::application()
    }
}

/// Configure how the tao event loop should update.
#[derive(Clone, Debug)]
pub enum UpdateMode {
    /// The event loop will update continuously, running as fast as possible.
    Continuous,
    /// The event loop will only update if there is a tao event, a redraw is requested, or the
    /// maximum wait time has elapsed.
    ///
    /// ## Note
    ///
    /// Once the app has executed all bevy systems and reaches the end of the event loop, there is
    /// no way to force the app to wake and update again, unless a `tao` event (such as user
    /// input, or the window being resized) is received or the time limit is reached.
    Reactive {
        /// max wait duration
        max_wait: Duration,
    },
    /// The event loop will only update if there is a tao event from direct interaction with the
    /// window (e.g. mouseover), a redraw is requested, or the maximum wait time has elapsed.
    ///
    /// ## Note
    ///
    /// Once the app has executed all bevy systems and reaches the end of the event loop, there is
    /// no way to force the app to wake and update again, unless a `tao` event (such as user
    /// input, or the window being resized) is received or the time limit is reached.
    ///
    /// ## Differences from [`UpdateMode::Reactive`]
    ///
    /// Unlike [`UpdateMode::Reactive`], this mode will ignore tao events that aren't directly
    /// caused by interaction with the window. For example, you might want to use this mode when the
    /// window is not focused, to only re-draw your bevy app when the cursor is over the window, but
    /// not when the mouse moves somewhere else on the screen. This helps to significantly reduce
    /// power consumption by only updated the app when absolutely necessary.
    ReactiveLowPower {
        /// max wait duration
        max_wait: Duration,
    },
}
