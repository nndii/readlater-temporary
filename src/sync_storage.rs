use stdweb::private::TODO;
use stdweb::unstable::TryInto;
use stdweb::{js, Reference};

use crate::browser::browser;

pub struct AddonStorageService {
    pub storage: SyncStorage,
}

impl AddonStorageService {
    pub fn new() -> Self {
        let storage = browser().sync_storage();
        AddonStorageService { storage }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Storage")]
pub struct SyncStorage(Reference);

impl SyncStorage {
    pub fn get(&self, key: &str) -> Option<Vec<String>> {
        js!(
            return @{self}.get(@{key});
        )
        .try_into()
        .ok()
    }

    pub fn set(&self, key: &str, value: Vec<String>) -> Result<(), TODO> {
        js!( @(no_return)
            @{self}.set(@{key}, @{value});
        );

        Ok(())
    }

    pub fn remove(&self, key: &str) {
        js!( @(no_return)
            @{self}.remove(@{key});
        );
    }
}
