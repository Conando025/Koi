use super::{Game, SharableNetwork};
use crate::alpha_zero::network::Network;
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

#[derive(Clone)]
pub struct Storage<G: Game> {
    network_history: Protected<Vec<SharableNetwork<G>>>,
    latest_natwork: Protected<Arc<Network<G>>>,
    latest_index: Protected<usize>,
}

pub(crate) type Protected<T> = Arc<RwLock<T>>;

impl<G: Game> Storage<G> {
    pub fn create(network: SharableNetwork<G>) -> Self {
        Storage {
            network_history: Arc::new(RwLock::new(vec![network.clone()])),
            latest_natwork: Arc::new(RwLock::new(Arc::new(network.to_network()))),
            latest_index: Arc::new(RwLock::new(0)),
        }
    }

    pub fn create_guard(&self) -> Guard<G> {
        Guard { storage: &self }
    }

    pub fn latest_network(&self) -> Arc<Network<G>> {
        let Ok(latest_natwork) = self.latest_natwork.read() else {
            panic!("Tried to read a broken Network history");
        };
        latest_natwork.clone()
    }

    pub fn latest_network_cached(&self, current: usize) -> Option<(Arc<Network<G>>, usize)> {
        let Ok(latest) =  self.latest_index.read() else {
            panic!("Tried to read a broken Network history");
        };
        if current == *latest {
            None
        } else {
            let Ok(latest_natwork) = self.latest_natwork.read() else {
                panic!("Tried to read a broken Network history");
            };

            Some((latest_natwork.clone(), *latest))
        }
    }
}

pub struct Guard<'a, G: Game> {
    storage: &'a Storage<G>,
}

impl<'a, G: Game> Guard<'a, G> {
    pub fn get(&self) -> SharableNetwork<G> {
        let (Ok(latest), Ok(mut network_history)) =  (self.storage.latest_index.read(), self.storage.network_history.read()) else {
            panic!("Tried to read a broken Network history");
        };
        return network_history[*latest].clone();
    }

    pub fn set(self, network: SharableNetwork<G>) {
        let (Ok(mut latest), Ok(mut network_history), Ok(mut latest_natwork)) =  (self.storage.latest_index.write(), self.storage.network_history.write(), self.storage.latest_natwork.write()) else {
            panic!("Tried to read a broken Network history");
        };
        (*network_history).push(network);
        *latest += 1;
        *latest_natwork = Arc::new(network_history[*latest].to_network());
    }
}

impl<'a, G: Game> Drop for Guard<'a, G> {
    fn drop(&mut self) {
        let Ok(mut latest) = self.storage.latest_index.write() else {
            panic!("Tried to read a broken Network history");
        };
        *latest += 1;
    }
}
