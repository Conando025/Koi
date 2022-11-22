use super::{Game, Network};
use std::borrow::BorrowMut;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

#[derive(Clone)]
pub struct Storage<G: Game> {
    network_history: Protected<Vec<Network<G>>>,
    latest: Protected<usize>,
}

pub(crate) type Protected<T> = Arc<RwLock<T>>;

impl<G: Game> Storage<G> {
    pub fn create(network: Network<G>) -> Self {
        Storage {
            network_history: Arc::new(RwLock::new(vec![network])),
            latest: Arc::new(RwLock::new(0)),
        }
    }

    pub fn lock_network_list(&self) -> RwLockWriteGuard<Vec<Network<G>>> {
        let (Ok(mut network_history), Ok(mut latest)) = (self.network_history.write(), self.latest.write()) else {
            panic!("Tried to read a broken Network history");
        };
        *latest += 1;
        network_history
    }

    pub fn latest_network(&self) -> Network<G> {
        let (Ok(network_history), Ok(latest)) = (self.network_history.read(), self.latest.read()) else {
            panic!("Tried to read a broken Network history");
        };
        network_history[*latest].clone()
    }

    pub fn latest_network_cached(&self, current: usize) -> Option<(Network<G>, usize)> {
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
