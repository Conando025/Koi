#[derive(Clone, Copy)]
pub struct Config {
    pub(super) actor_count: usize,
    pub(super) move_samples: usize,
    pub(super) max_move_count: usize,
    pub(super) dirichlet_alpha: f64, //0.3 for chess, 0.03 for Go and 0.15 for shogi.
    pub(super) exploration_fraction: f64,
    pub(super) c_base: usize, //for UCB formula
    pub(super) c_init: f64,   //for UCB formula
    pub(super) training_steps: usize,
    pub(super) checkpoint_interval: usize,
    pub(super) window_size: usize,
    pub(super) batch_size: usize,
    pub(super) weight_decay: f64,
    pub(super) momentum: f64,
    pub(super) learning_rate_schedule: (usize, f64),
}
