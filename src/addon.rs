use yew::{Component, ComponentLink, Renderable, ShouldRender, Html, html};
use rand::{Rng, thread_rng};

use crate::browser::{browser, Browser};
use crate::sync_storage::AddonStorageService;

const KEY: &'static str = "readlater_storage";

pub struct Model {
    storage: AddonStorageService,
    browser: Browser,
}

pub enum Msg {
    Pop,
    Push,
}

impl Model {
    fn pop(&self) {
        if let Some(array) = self.storage.storage.get(KEY) {
            let mut entries = array;

            let slice: &mut [String] = &mut entries;
            thread_rng().shuffle(slice);

            self.browser.create_tab(&entries.pop().unwrap());
            self.storage.storage.set(KEY, entries);
        }
    }

    fn get_current_url(&self) -> String {
        let tab = self.browser.active_tab();
        tab.url()
    }

    fn push(&self) {
        let url = self.get_current_url();
        let mut entries = self.storage.storage.get(KEY).unwrap();
        entries.push(url);
        self.storage.storage.set(KEY, entries);
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut storage = AddonStorageService::new();
        let mut browser = browser();
        Model { storage, browser }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Push => {
                self.push();
                false
            },
            Msg::Pop => {
                self.pop();
                false
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                <button onclick=|_| Msg::Pop,>{ "Pop" }</button>
                <button onclick=|_| Msg::Push,>{ "Push" }</button>
                </nav>
            </div>
        }
    }
}
