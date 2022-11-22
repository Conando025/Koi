struct Config {
    actor_count: usize,
    move_samples: usize,
    max_move_count: usize,
    dirichlet_alpha: f64,           //0.3 for chess, 0.03 for Go and 0.15 for shogi.
    exploration_fraction: f64,
    c_base: usize,                  //for UCB formula
    c_init: f64,                    //for UCB formula
    training_steps: usize,
    checkpoint_interval: usize,
    window_size: usize,
    batch_size: usize,
    weight_decay: f64,
    momentum: f64,
    learning_rate_schedule: (usize, f64),
}