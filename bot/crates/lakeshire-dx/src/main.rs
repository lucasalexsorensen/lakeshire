mod components;
mod faker;
mod global;
mod pages;
mod tasks;
use dioxus::{
    desktop::{use_global_shortcut, use_window, HotKeyState},
    prelude::*,
};
use global::GlobalState;

use lakeshire_core::serialization::protos::GameState;
use pages::{DebugPage, HomePage};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let global_state = use_context_provider(|| GlobalState {
        game_state: SyncSignal::new_maybe_sync(None::<GameState>),
    });

    use_hook(|| {
        tasks::start_scanner_task(global_state);
    });

    let active_window_title = use_signal_sync(|| None::<String>);
    use_hook(|| {
        tasks::start_active_window_monitor_task(active_window_title);
    });

    setup_shortcuts();

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "bg-gray-800 w-screen h-screen flex flex-col items-center",
            Router::<Route> {}
        }
    }
}

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        HomePage {},
        #[route("/debug")]
        DebugPage {}
}

#[component]
fn NavBar() -> Element {
    let current_route = use_route::<Route>();
    let is_home = matches!(current_route, Route::HomePage {});
    let is_debug = matches!(current_route, Route::DebugPage {});
    rsx! {
        div {
            role: "tablist",
            class: "tabs tabs-box justify-center w-fit",
            Link {
                role: "tab",
                class: format!("tab {}", if is_home { "tab-active" } else { "" }),
                to: "/",
                "Home"
            }
            Link {
                role: "tab",
                class: format!("tab {}", if is_debug { "tab-active" } else { "" }),
                to: "/debug",
                "Debug"
            }
        }
        div {
            class: "p-2",
            Outlet::<Route> {}
        }
    }
}

fn setup_shortcuts() {
    _ = use_global_shortcut("ctrl+l", move |state| {
        let window = use_window();
        if state != HotKeyState::Pressed {
            return;
        }
        // window.set_visible(!window.is_visible());
        window.devtool();
    });
}
