#![recursion_limit = "256"]

mod material {
    pub mod topbar;
}

mod components {
    pub mod main_page;
}

use wasm_bindgen::prelude::*;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{prelude::*, Switch};
use yew_router::switch::{AllowMissing, Permissive};

use material::topbar::*;
use components::main_page::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Topbar title="What 2 by where?"/>
                <div class="mdc-top-app-bar--fixed-adjust">
                    <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Index => html! { <MaingPageModel /> },
                                // AppRoute::A(AllowMissing(route)) => html!{<AModel route = route />},
                                // AppRoute::B(route) => {
                                //     let route: b_component::Props = route.into();
                                //     html!{<BModel with route/>}
                                // },
                                // AppRoute::C => html!{<CModel />},
                                AppRoute::E(string) => html!{format!("hello {}", string)},
                                AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                            }
                        })
                        redirect = Router::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route)))
                        })
                    />
                </div>
            </div>
        }
    }
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/"]
    Index,
    // #[to = "/a{*:inner}"]
    // A(AllowMissing<ARoute>),
    // #[to = "/b{*:inner}"]
    // B(BRoute),
    // #[to = "/c"]
    // C,
    #[to = "/e/{string}"]
    E(String),
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

#[derive(Debug, Switch, PartialEq, Clone, Copy)]
#[to = "/c"]
pub struct ARoute;

#[wasm_bindgen(start)]
pub fn render() {
    yew::start_app::<Model>();
}