#![allow(unused)]
mod config;
mod node;
mod game;
mod network;
mod storage;
mod mcts;

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
}

impl AlphaZero {
    pub fn empty(config: Config) -> Self {
        AlphaZero {
            config,
            network: Network{},
        }
    }

    pub fn train<G: Game>(&mut self, terminator: Receiver<()>) {
        let storage = Storage::create(self.network.clone());
        let (tx, rx) = channel::<G>();
        for i in 0..self.config.actor_count {
            let (config, storage, tx) = (self.config, storage.clone(), tx.clone());
            spawn(|| generate_self_play::<G>(config, storage, tx));
        }
        drop(tx);

        let mut game_buffer: Vec<G> = Vec::with_capacity(self.config.window_size);
        for game in rx {
            if let Ok(_) = terminator.try_recv() {
                break;
            }
            game_buffer.push(game);
        }

        let network = storage.latest_network();
        self.network = network;
    }
}

fn generate_self_play<G: Game>(config: Config, storage: Storage, tx: Sender<G>) {
    let mut network = storage.latest_network();
    let mut current_network_index = 0;
    loop {
        if let (new_network, new_index) = storage.latest_network_cached(current_network_index) {
            network = new_network;
            current_network_index = new_index;
        }
        let game = play_game(config, &network);
        tx.send(game)
    }
}

fn play_game<G: Game>(config: Config, network: &Network) -> G {
    let mut game = G::create(None);
    while !game.terminal() && game.len() < config.max_move_count {
        let (action, root) = mcts::run(config, &game, &network);
        game.apply(action);
        game.store_search_statistics(root);
    }
    game
}