use crate::app::hop_net;

pub struct StorkeyLearningNetwork {
    pub state: Vec<f64>,
    pub rng: rand::rngs::ThreadRng,
    weights: Vec<Vec<f64>>,
    inference_weights: Vec<Vec<f64>>,
    number_of_learned_states: f64,
    steps: usize,
    nodes_yet_to_update: Vec<usize>,
}

impl hop_net::Net<f64> for StorkeyLearningNetwork {
    fn get_state(&self) -> Vec<f64> {
        self.state.clone()
    }

    fn learn(&mut self, state: &[f64]) {
        self.number_of_learned_states += 1.0;
        self.storkey_learning(state);
    }

    fn step(&mut self) -> (bool, Vec<f64>) {
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

impl StorkeyLearningNetwork {
    pub fn new(size: usize, start_state: Option<&Vec<f64>>) -> StorkeyLearningNetwork {
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

        StorkeyLearningNetwork {
            state,
            rng: rand::thread_rng(),
            weights: vec![vec![0.0; size]; size],
            steps: 0,
            number_of_learned_states: 0.0,
            nodes_yet_to_update: nodes_to_update,
            inference_weights: vec![vec![0.0; size]; size],
        }
    }

    fn update_node(&mut self, i: usize) -> bool {
        let mut sum = 0.0;
        for j in 0..self.inference_weights[i].len() {
            sum += self.inference_weights[i][j] * self.state[j];
        }
        let new_val = if sum > 0.0 { 1.0 } else { -1.0 };
        if new_val != self.state[i] {
            self.state[i] = new_val;
            return true;
        }
        false
    }

    fn storkey_learning(&mut self, state: &[f64]) {
        let old_weights = self.weights.clone();
        let len = self.state.len();
        let c = self.c(state, &old_weights);

        for i in 0..len {
            for j in 0..len {
                // when state procuct is +1, if node i and j want to be on/off at the same time, and
                // new_state = state * weights will keep changing until that constraint is met
                // for -1 they want to be different
                let state_product = state[i] * state[j];

                //I have't been able to find mutch about this term
                let noise_reduction = state[i] * self.h(j, i, &state, &old_weights)
                    + state[j] * self.h(i, j, &state, &old_weights);
                let h_product =
                    self.h(i, j, &state, &old_weights) * self.h(j, i, &state, &old_weights);

                self.weights[i][j] = old_weights[i][j]
                    +/* c */ ((state_product /*+ h_product*/) - noise_reduction) / (len as f64);
            }
        }

        self.inference_weights = self.weights.clone();
        // In the paper it said that havind 0 on the diagonal improves retrival, but hinders learning, so i just store 2 copies of the weights
        for i in 0..len {
            self.inference_weights[i][i] = 0.0;
        }
    }

    //h should be the interaction of the new state with the weights matrix (aka the other stored states)
    fn h(&self, i: usize, j: usize, new_state: &[f64], old_weights: &Vec<Vec<f64>>) -> f64 {
        let mut sum = 0.0;
        for iter in 0..self.state.len() {
            if iter != j || iter != i {
                sum += old_weights[i][iter] * new_state[iter];
            }
        }
        sum
    }

    // Pseudo inverse rule
    fn c(&self, new_state: &[f64], old_weights: &Vec<Vec<f64>>) -> f64 {
        let len = self.state.len();
        let mut sum = 0.0;
        for i in 0..len {
            for j in 0..len {
                sum += new_state[i] * old_weights[i][j] * new_state[j];
            }
        }
        1.0 - sum / (len as f64)
    }
}
