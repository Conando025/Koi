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
            network_history: Arc::new(RwLock::new(vec![network])),
            latest: Arc::new(RwLock::new(0))
        }
    }

    pub fn lock_network_list(&self) -> RwLockWriteGuard<Vec<Network>> {
        let (Ok(mut network_history), Ok(mut latest)) = (self.network_history.write(), self.latest.write()) else {
            panic!("Tried to read a broken Network history");
        };
        *latest += 1;
        network_history
    }

    pub fn latest_network(&self) -> Network {
        let (Ok(network_history), Ok(latest)) = (self.network_history.read(), self.latest.read()) else {
            panic!("Tried to read a broken Network history");
        };
        network_history[*latest].clone()
    }

    pub fn latest_network_cached(&self, current: usize) -> Option<(Network, usize)> {
        let Ok(latest) =  self.latest.read() else {
            panic!("Tried to read a broken Network history");
        };
        if current == *latest {
            None
        } else {
            let Ok(network_history) = self.network_history.read() else {
                panic!("Tried to read a broken Network history");
            };

            Some((network_history[*latest].clone(), *latest))
        }
    }
}

