use std::borrow::BorrowMut;
use std::sync::{RwLock, Arc};
use super::Network;

struct InternalStorage {
    network_history: Vec<Network>,
    latest: usize,
}

#[derive(Clone)]
pub struct Storage {
    _internal: Protected<InternalStorage>
}

pub(crate) type Protected<T> = Arc<RwLock<T>>;

impl InternalStorage {
    pub fn latest_network(&self) -> Network {
        self.network_history[self.latest].clone()
    }
}

impl Storage {
    pub fn create(network: Network) -> Self {
        Storage {
            _internal: Arc::new(RwLock::new(
                InternalStorage {
                    network_history: vec![network],
                    latest: 0,
                }
            ))
        }
    }

    pub fn add_network(self, network: Network) {
        let Ok(mut storage) = self._internal.write() else {
            panic!("Tried to read a broken Network history");
        };
        storage.network_history.push(network);
        storage.latest += 1;
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

