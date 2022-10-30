
### Criteria

- How to declare markdown in ui component?
	- via ui component, macro or custom element ?

### Dependencies

- [Dioxus](https://github.com/DioxusLabs/dioxus)
- [zoni/obsidian-export](https://github.com/zoni/obsidian-export): convert [[🛒 Glossary#Obsidian|Obsidian]] markdown to CommonMark
- [raphlinus/pulldown-cmark](https://github.com/raphlinus/pulldown-cmark): Markdown parser

### Resources

- [Html (and SVG) Namespace for Dioxus](https://github.com/DioxusLabs/dioxus/blob/master/packages/html/README.md#html-and-svg-namespace-for-dioxus)
- [custom element](https://github.com/DioxusLabs/dioxus/blob/c97ca7dff651dd1e14edb3fc70d517715796d666/docs/guide/src/en/__unused/advanced-guides/custom-renderer.md#custom-raw-elements)

### Mdx Component

**Usage**

```rust
use dip::prelude::*;

fn Page(cx: Scope) -> Element {
	cx.render(rsx! {
		h1 { "MDX example" }

		Mdx {
			src: "hello.mdx"
		}
	})
}
```

**Implementation**

```rust
struct MdxProps {
	src: String
}

fn Mdx(cx: Scope<MdxProps>) -> Element {
	let mdx = use_read(&cx, MDX);
	let window = use_window::<UiAction, NoAsyncAction>();
	let id = cx.scope_id();

	use_effect(&cx, (), |_| {
		let src = cx.props.src.clone();
		window.send(UiAction::fetch_markdown(id, src)):
	});

	cx.render(mdx.render(id))
}

pub struct MdxPlugin;

impl Plugin for MdxPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<MarkdownComponent>()
			.add_plugin(ActionPlugin);
	}
}

#[ui_state]
impl UiState {
	mdx: MarkdownComponent,
}

struct FetchMarkdown {
	id: ScopeId,
	src: String,
}

#[ui_action]
impl UiAction {
	pub fn fetch_markdown(id, src) -> FetchMarkdown {
		FetchMarkdown {
			id,
			src,
		}
	}
}

// Resources

#[derive(Clone, Default)]
pub struct MarkdownComponent {
	mds: HashMap,
}

impl MarkdownComponent {
	pub fn render(&self, id: Uuid) -> LazyNodes {
		LazyNodes::new(|f| {
			let md = mds.get(id).expect("Failed to get markdown");
		})
	}
}
```