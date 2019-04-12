use stdweb::js;
use stdweb::unstable::TryInto;
use stdweb::Reference;

use crate::sync_storage::SyncStorage;
use crate::tab::Tab;

#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Browser")]
pub struct Browser(Reference);

pub fn browser() -> Browser {
    unsafe { js!( console.log("BROWSER:", browser); return browser; ).into_reference_unchecked() }.unwrap()
}

impl Browser {
    pub fn sync_storage(&self) -> SyncStorage {
        unsafe {
            js!(
                return @{self}.storage.sync;
            )
            .into_reference_unchecked()
            .unwrap()
        }
    }

    pub fn active_tab(&self) -> Tab {
        js!(

            console.log("ACTIVE_TABS", @{self}.tabs);
            const active_tabs = @{self}.tabs.query({
                currentWindow: true,
                active: true,
            });
            console.log("ACTIVE_TABS", active_tabs);
            return active_tabs[0];
        ).try_into().unwrap()
    }

    pub fn create_tab(&self, url: &str) {
        js! { @(no_return)
                @{self}.tabs.create({
                    url: @{url}
                })
        }
    }
}
