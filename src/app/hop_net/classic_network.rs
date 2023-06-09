use crate::app::hop_net;
use rand::Rng;
pub struct ClassicNetworkDiscrete {
    pub state: Vec<f64>,
    pub rng: rand::rngs::ThreadRng,
    weights: Vec<Vec<f64>>,
    number_of_learned_states: f64,
    steps: usize,
    nodes_yet_to_update: Vec<usize>,
}

// The network will mostly be interacted with trough this traits
impl hop_net::Net<f64> for ClassicNetworkDiscrete {
    fn get_state(&self) -> Vec<f64> {
        self.state.clone()
    }

    fn learn(&mut self, state: &[f64]) {
        self.number_of_learned_states += 1.0;
        self.hebbian_learning(state);
    }

    fn step(&mut self) -> (bool, Vec<f64>) {
        //generat e random index from 0 to state.len()
        //let i = self.rng.gen_range(0..self.state.len());
        if self.nodes_yet_to_update.is_empty() {
            hop_net::reset_nodes_to_update(&mut self.nodes_yet_to_update, self.state.len());
        }
        let i = self.nodes_yet_to_update.pop().unwrap();
        self.steps += 1;
        let state_changed = self.update_node(i);

        (state_changed, self.state.clone())
    }

    fn get_steps(&self) -> usize {
        self.steps
    }

    fn set_state(&mut self, state: &[f64]) {
        if state.len() < 4 {
            panic!("State is too short");
        }

        if self.state.len() != state.len() {
            self.number_of_learned_states = 0.0;
            self.weights = vec![vec![0.0; state.len()]; state.len()];
            self.steps = 0;
        }
        self.state = state.to_vec();

        // The starting state has just been set, so we are 0 steps away from it
        self.steps = 0;

        // We make sure that all nodes are marked as "to update"
        hop_net::reset_nodes_to_update(&mut self.nodes_yet_to_update, self.state.len());
    }

    fn reset_weights(&mut self) {
        self.weights = vec![vec![0.0; self.state.len()]; self.state.len()];
        self.number_of_learned_states = 0.0;
    }

    fn get_weights(&self) -> Vec<Vec<f64>> {
        self.weights.clone()
    }
}
// In this case it gives a false allarm, the suggestion is not applicable
#[allow(clippy::unnecessary_unwrap)]
impl ClassicNetworkDiscrete {
    pub fn new(size: usize, start_state: Option<&Vec<f64>>) -> ClassicNetworkDiscrete {
        let state = if start_state.is_none() {
            vec![-1.0; size]
        } else {
            let start_s = start_state.unwrap();
            if start_s.len() != size {
                panic!("Size and start size lenght are differnt");
            }
            start_s.clone()
        };

        let mut nodes_to_update = Vec::with_capacity(size);
        hop_net::reset_nodes_to_update(&mut nodes_to_update, size);

        ClassicNetworkDiscrete {
            state,
            rng: rand::thread_rng(),
            weights: vec![vec![0.0; size]; size],
            steps: 0,
            number_of_learned_states: 0.0,
            nodes_yet_to_update: nodes_to_update,
        }
    }

    pub fn init(&mut self, state: Option<&Vec<f64>>) {
        if let Some(s) = state {
            self.state = s.clone();
        } else {
            for i in 0..self.state.len() {
                self.state[i] = if self.rng.gen_range(0..=1) == 1 {
                    1.0
                } else {
                    -1.0
                };
            }
        }
        // self.setWeightsToRandom();
        self.steps = 0;
    }

    fn hebbian_learning(&mut self, state_to_learn: &[f64]) {
        for i in 0..self.weights.len() {
            for j in 0..self.weights[i].len() {
                if i == j {
                    self.weights[i][j] = 0.0;
                } else {
                    // self.weights[i][j] +=(1.0 / self.number_of_learned_states) *(state_to_learn[i] * state_to_learn[j]) + ((self.number_of_learned_states - 1.0) /self.number_of_learned_states) self.weights[i][j];
                    self.weights[i][j] += state_to_learn[i] * state_to_learn[j];
                }
            }
        }
    }

    // This function summs the weight coming into a node, and then takes the sign of the sum
    // This is the standard upadate rule for the classic hopfield networks
    fn update_node(&mut self, i: usize) -> bool {
        let mut sum = 0.0;
        for j in 0..self.weights[i].len() {
            sum += self.weights[i][j] * self.state[j];
        }
        let new_val = if sum > 0.0 { 1.0 } else { -1.0 };
        if new_val != self.state[i] {
            self.state[i] = new_val;
            return true;
        }
        false
    }

    // Getters
}

impl std::fmt::Display for ClassicNetworkDiscrete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "state:\n{}", hop_net::state_vec_to_string(&self.state))
    }
}
