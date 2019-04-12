use stdweb::unstable::TryInto;
use stdweb::Reference;

#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Tab")]
pub struct Tab(Reference);

impl Tab {
    pub fn url(&self) -> String {
        js!( return @{self}.url; ).try_into().unwrap()
    }
}
