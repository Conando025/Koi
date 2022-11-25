#![allow(unused)]
mod config;
mod game;
mod mcts;
mod network;
mod node;
mod storage;

use config::*;
use game::*;
use network::*;
use node::*;
use rand::distributions::{Distribution, WeightedIndex};
use rand::{thread_rng, Rng};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::spawn;
use storage::*;

pub struct AlphaZero<G: Game> {
    config: Config,
    network: SharableNetwork<G>,
}

impl<G: Game + Sync + 'static> AlphaZero<G> {
    pub fn empty(config: Config) -> Self {
        AlphaZero {
            config,
            network: SharableNetwork::new(),
        }
    }

    pub fn train(&mut self, terminator: Receiver<()>) {
        let storage = Storage::create(self.network.clone());
        let (tx, rx) = channel::<G>();
        for i in 0..self.config.actor_count {
            let tx = tx.clone();
            let storage = storage.clone();
            let config = self.config.clone();
            spawn(move || {
                Self::generate_self_play(config, storage.clone(), tx.clone())
            });
        }
        drop(tx);

        let mut game_buffer: Vec<G> = Vec::with_capacity(self.config.window_size);
        for game in rx {
            if let Ok(_) = terminator.try_recv() {
                break;
            }
            game_buffer.push(game);
            if game_buffer.len() > self.config.batch_size {
                let mut move_sum = 0;
                let move_lengths = game_buffer.iter().map(|g| {
                    move_sum += 1;
                    g.len()
                });
                let mut dist = WeightedIndex::new(move_lengths).unwrap();
                let mut target_data = Vec::with_capacity(self.config.batch_size);
                let mut rng = thread_rng();
                for _ in 0..self.config.batch_size {
                    let g = &game_buffer[dist.sample(&mut rng)];
                    let i = rng.gen_range(0..g.len());
                    target_data.push((g.make_image(i), g.make_target(i)))
                }
                self.train_network(storage.clone(), target_data);
            }
        }

        self.network = storage.create_guard().get();
    }

    fn generate_self_play(config: Config, storage: Storage<G>, tx: Sender<G>) {
        let mut index = 0;
        let mut network = storage.latest_network();
        loop {
            if let Some((n,i)) = storage.latest_network_cached(index) {
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

    fn train_network(&self, storage: Storage<G>, batch_data: Vec<(G::Image, Target)>) {
        let guard = storage.create_guard();
        let sharabke_network = guard.get();
        let mut network = sharabke_network.to_network();
        network.train(self.config);
        guard.set(sharabke_network);
    }
}
