#![allow(unused)]
mod config;
mod node;
mod game;
mod network;
mod storage;

use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread::spawn;
use config::*;
use node::*;
use network::*;
use game::*;
use storage::*;

pub struct AlphaZero {
    config: Config,
    network: Network,
    iteration: usize,
}

impl AlphaZero {
    pub fn empty(config: Config) -> Self {
        AlphaZero {
            config,
            network: Network{},
            iteration: 0,
        }
    }

    pub fn set_training_steps(&mut self, steps: usize) -> Result<(), Err()>{
        if steps < self.iteration {
            Err("You cant have less steps that already done!")
        } else {
            self.config.training_steps = steps;
            Ok(())
        }

    }

    pub fn train<G: Game>(&mut self, terminator: Receiver<()>) {
        let storage = Storage::create(self.network.clone());
        let (tx, rx) = channel::<G::Action>();
        for i in 0..self.config.actor_count {
            let (config, storage, tx) = (self.config.clone(), storage.clone(), tx.clone());
            spawn(|| generate_self_play::<G>(config, storage, tx));
        }
        drop(tx);

        let mut action_list = Vec::with_capacity(100);
        for action in rx {
            if let Ok(_) = terminator.try_recv() {
                break;
            }
            action_list.push(action);
        }

        let network = storage.latest_network();
        self.network = network;
    }
}

fn generate_self_play<G: Game>(config: Config, storage: Storage, tx: Sender<G::Action>) {
    unimplemented!()
}