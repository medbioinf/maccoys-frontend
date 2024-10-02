// std imports
use std::{fmt::Display, ops::Deref};

// 3rd party imports
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::{FaBars, FaXmark};
use dioxus_free_icons::Icon;

// internal imports
use crate::routes::Routes;

/// Menu state (open or close), used to toggle the menu
///
#[derive(Debug, Clone, PartialEq)]
enum MenuState {
    Open,
    Close,
}

impl Display for MenuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuState::Open => write!(f, "menu-open"),
            MenuState::Close => write!(f, "menu-close"),
        }
    }
}

/// Layout with two panes. One for the menu and one for the main content
pub fn TwoPanes() -> Element {
    let mut menu_state = use_signal(|| MenuState::Close);

    let close_menu_fn = move |_| menu_state.set(MenuState::Close);
    let toggle_menu_fn = move |_| {
        let new_state = match menu_state.read().deref() {
            MenuState::Open => MenuState::Close,
            MenuState::Close => MenuState::Open,
        };
        menu_state.set(new_state);
    };

    rsx! {
        div { class: "layout-two-panes container-fluid {menu_state}",
            aside { class: "pane-menu col-12 col-lg-2 position-sticky d-flex flex-column vh-100",
                header { class: "d-flex align-items-center p-3",

                    Link {
                        to: Routes::Start {},
                        class: "navbar-brand d-flex align-items-center",
                        onclick: close_menu_fn,
                        // img { src: "/images/logo.png", alt: "MaCcoyS logo" }
                        span { "MaCcoyS" }
                    }
                    button {
                        r#type: "button",
                        class: "btn btn-sm btn-outline-primary d-lg-none",
                        onclick: toggle_menu_fn,
                        if *menu_state.read().deref() == MenuState::Close {
                            Icon { icon: FaBars }
                        } else {
                            Icon { icon: FaXmark }
                        }
                    }
                }
                div { class: "application-menu d-flex flex-column flex-fill",
                    nav { class: "internal-pages flex-fill px-3",
                        div { class: "separator", "Explore the database" }
                        ul { class: "navbar-nav mx-3",
                            li { class: "nav-item",
                                Link { to: Routes::Start {}, onclick: close_menu_fn, class: "nav-link",
                                    i { class: "fa-solid fa-magnifying-glass me-2" }
                                    "Example link to start"
                                }
                            }
                        }
                        div { class: "separator", "Tools" }
                    }
                    nav { class: "external-pages p-3", "External links coming soon" }
                }
            }
            div { class: "pane-content col-12 col-lg-10 p-3", Outlet::<Routes> {} }
        }
    }
}
