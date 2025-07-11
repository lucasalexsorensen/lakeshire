mod components;
mod faker;
mod global;
mod pages;
use dioxus::prelude::*;
use global::GlobalState;

// use global::GAME_STATE;

use lakeshire_core::serialization::protos::GameState;
use pages::{DebugPage, HomePage};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut global_state = use_context_provider(|| GlobalState {
        game_state: SyncSignal::new_maybe_sync(None::<GameState>),
    });

    use_hook(|| {
        let mut faker = faker::GameStateFaker::default();
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(16));
            global_state.game_state.set(Some(faker.generate()));
        });
    });

    let mut active_window_title = use_signal_sync(|| None::<String>);
    use_hook(|| {
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_millis(250));
            let old_val = active_window_title();
            let window =
                active_win_pos_rs::get_active_window().expect("Error while getting active window");

            let new_val = Some(window.title);

            if new_val != old_val {
                println!("new_val: {:?}", &new_val);
                active_window_title.set(new_val);
            }
        });
    });

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
