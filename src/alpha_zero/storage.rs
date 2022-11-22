use std::borrow::BorrowMut;
use std::sync::{RwLock, Arc, RwLockWriteGuard};
use super::Network;

#[derive(Clone)]
pub struct Storage {
    network_history: Protected<Vec<Network>>,
    latest: Protected<usize>,
}

pub(crate) type Protected<T> = Arc<RwLock<T>>;

impl Storage {
    pub fn create(network: Network) -> Self {
        Storage {
            network_history: Arc::new(RwLock::new(
                    network_history: vec![network]
            )),
            latest: Arc::new(RwLock::new(0))
        }
    }

    pub fn lock_network_list(self) -> RwLockWriteGuard<Vec<Network>>{
        let Ok((mut network_history, mut latest)) = (self.network_history.write(), self.latest.write()) else {
            panic!("Tried to read a broken Network history");
        };
        latest += 1;
        network_history
    }

    pub fn latest_network(&self) -> Network {
        let Ok(mut storage) = self._internal.read() else {
            panic!("Tried to read a broken Network history");
        };
        storage.latest_network()
    }

    pub fn latest_network_cached(&self, current: usize) -> Option<(Network, usize)> {
        let Ok(mut storage) = self._internal.read() else {
            panic!("Tried to read a broken Network history");
        };
        if current == storage.latest {
            None
        } else {
            Some((storage.latest_network(), storage.latest))
        }
    }
}

