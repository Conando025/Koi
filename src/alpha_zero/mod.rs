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
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::spawn;
use storage::*;

pub struct AlphaZero<G: Game> {
    config: Config,
    network: Network<G>,
}

impl<G: Game + Sync + 'static> AlphaZero<G> {
    pub fn empty(config: Config) -> Self {
        AlphaZero {
            config,
            network: Network::new(),
        }
    }

    pub fn train(&mut self, terminator: Receiver<()>) {
        let storage = Storage::create(self.network.clone());
        let (tx, rx) = channel::<G>();
        for i in 0..self.config.actor_count {
            let (config, storage, tx) = (self.config.clone(), storage.clone(), tx.clone());
            spawn(move || generate_self_play::<G>(config, storage, tx));
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

fn generate_self_play<G: Game>(config: Config, storage: Storage<G>, tx: Sender<G>) {
    let mut network = storage.latest_network();
    let mut current_network_index = 0;
    loop {
        if let Some((new_network, new_index)) = storage.latest_network_cached(current_network_index)
        {
            network = new_network;
            current_network_index = new_index;
        }
        let game = play_game::<G>(config, &network);
        tx.send(game);
    }
}

fn play_game<G: Game>(config: Config, network: &Network<G>) -> G {
    let mut game = G::create(None);
    while !game.terminal() && game.len() < config.max_move_count {
        let (action, root) = mcts::run(config, &game, &network);
        game.apply(action);
        game.store_search_statistics(root);
    }
    game
}
