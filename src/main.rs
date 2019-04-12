#[macro_use]
extern crate stdweb_derive;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

mod browser;
mod sync_storage;
mod tab;
mod addon;

fn main() {
    yew::start_app::<crate::addon::Model>();
}
