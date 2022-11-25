#![allow(unused)]
mod config;
mod game;
mod mcts;
mod network;
mod node;
mod replay_buffer;
mod storage;

use config::*;
use game::*;
use network::*;
use node::*;
use replay_buffer::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread::spawn;
use storage::*;

pub struct AlphaZero<G: Game> {
    config: Config,
    network: ShareableNetwork<G>,
}

impl<G: Game + Sync + 'static> AlphaZero<G> {
    pub fn empty(config: Config) -> Self {
        AlphaZero {
            config,
            network: ShareableNetwork::new(),
        }
    }

    pub fn train(&mut self, terminator: Receiver<()>) {
        let storage = Storage::create(self.network.clone());
        let (tx, rx) = channel::<G>();
        for i in 0..self.config.actor_count {
            let tx = tx.clone();
            let storage = storage.clone();
            let config = self.config.clone();
            spawn(move || Self::generate_self_play(config, storage.clone(), tx.clone()));
        }
        drop(tx);

        let game_buffer: Protected<ReplayBuffer<G>> =
            Arc::new(RwLock::new(ReplayBuffer::new(self.config)));
        let thread_game_buffer = game_buffer.clone();
        spawn(move || {
            const TINY_BUFFER_SIZE: usize = 10;
            let mut tiny_buffer = Vec::with_capacity(TINY_BUFFER_SIZE);
            for game in rx {
                if let Ok(_) = terminator.try_recv() {
                    break;
                }
                if tiny_buffer.len() < TINY_BUFFER_SIZE {
                    tiny_buffer.push(game);
                } else {
                    let mut core = (*thread_game_buffer)
                        .write()
                        .expect("The buffer was poisened");
                    (*core).save_games(&mut tiny_buffer);
                    tiny_buffer = Vec::with_capacity(TINY_BUFFER_SIZE);
                }
            }
        });

        self.train_network(storage.clone(), game_buffer);

        self.network = storage.latest_shareablenetwork();
    }

    fn generate_self_play(config: Config, storage: Storage<G>, tx: Sender<G>) {
        let mut index = 0;
        let mut network = storage.latest_network();
        loop {
            if let Some((n, i)) = storage.latest_network_cached(index) {
                index = i;
                network = n;
            };
            let game = Self::play_game(config, &*network);
            tx.send(game);
        }
    }

    fn play_game(config: Config, network: &Network<G>) -> G {
        let mut game = G::create(None);
        while !game.terminal() && game.len() < config.max_move_count {
            let (Some(action), root) = mcts::run(config, &game, network) else {
                break
            };
            game.apply(action);
            game.store_search_statistics(root);
        }
        game
    }

    fn train_network(&self, storage: Storage<G>, replay_buffer: Protected<ReplayBuffer<G>>) {
        let shareable_network = storage.latest_shareablenetwork();
        let mut network = shareable_network.to_network();

        for i in 0..self.config.training_steps {
            if i % self.config.checkpoint_interval == 0 {
                storage.add_new_network(shareable_network.clone());
            }
            let buffer = replay_buffer.read().expect("Buffer was poisened");
            let _batch = (*buffer).sample_batch();
            network.train(self.config);
        }
        storage.add_new_network(shareable_network);
    }
}
