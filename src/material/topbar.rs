use yew::prelude::*;

pub struct Topbar {
    link: ComponentLink<Self>,
    title: String,
    // onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
    // pub onsignal: Callback<()>,
}

impl Component for Topbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Topbar {
            link,
            title: props.title,
            // onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                // self.onsignal.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        // self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        html! {
            <header class="mdc-top-app-bar">
                <div class="mdc-top-app-bar__row">
                    <section class="mdc-top-app-bar__section mdc-top-app-bar__section--align-start">
                        <button class="material-icons mdc-top-app-bar__navigation-icon mdc-icon-button"> { "menu" } </button>
                        <span class="mdc-top-app-bar__title">{ &self.title }</span>
                    </section>
                </div>
            </header>
        }
    }
}