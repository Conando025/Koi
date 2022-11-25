use super::{Config, Game, Target};
use rand::distributions::{Distribution, WeightedIndex};
use rand::{thread_rng, Rng};

pub struct ReplayBuffer<G> {
    data: Vec<G>,
    running_index: usize,
    size: usize,
    batch_size: usize,
}

impl<G: Game> ReplayBuffer<G> {
    pub fn new(config: Config) -> ReplayBuffer<G> {
        ReplayBuffer {
            data: Vec::with_capacity(config.window_size),
            running_index: 0,
            size: config.window_size,
            batch_size: config.batch_size,
        }
    }

    pub fn save_game(&mut self, g: G) {
        if self.data.len() < self.size {
            self.running_index += 1;
            self.data.push(g);
        } else {
            self.running_index = (self.running_index + 1) % self.size;
            self.data[self.running_index] = g;
        }
    }

    pub fn save_games(&mut self, games: &mut Vec<G>) {
        if self.data.len() + games.len() < self.size {
            self.running_index += games.len();
            self.data.append(games);
        } else {
            let buf_pointer = self.data.as_mut_ptr();
            unsafe {
                for (i, game) in games.drain(..).enumerate() {
                    *buf_pointer.add((self.running_index + i) % self.size) = game;
                }
            }
            self.running_index += games.len() % self.size;
        }
    }

    pub fn sample_batch(&self) -> Vec<(G::Image, Target)> {
        let mut move_sum = 0;
        let move_lengths = self.data.iter().map(|g| {
            move_sum += g.len();
            g.len()
        });
        let mut dist = WeightedIndex::new(move_lengths).unwrap();
        let mut target_data = Vec::with_capacity(self.batch_size);
        let mut rng = thread_rng();
        for _ in 0..self.batch_size {
            let g = &self.data[dist.sample(&mut rng)];
            let i = rng.gen_range(0..g.len());
            target_data.push((g.make_image(i), g.make_target(i)))
        }
        target_data
    }
}
