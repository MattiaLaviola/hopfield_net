use rand::Rng;
use rand::prelude::SliceRandom;
pub struct ClassicNetworkDiscrete{
    pub state : Vec<f64>,
    pub rng : rand::rngs::ThreadRng,
    weights : Vec<Vec<f64>>,
    numberOfLearnedStates : f64,
    steps: usize,
    nodes_yet_to_update: Vec<usize>
}

impl ClassicNetworkDiscrete {
    pub fn new(size : usize) -> ClassicNetworkDiscrete{
        let mut vect = Vec::with_capacity(size);
        ClassicNetworkDiscrete::reset_nodes_to_update(&mut vect, size);

        ClassicNetworkDiscrete{
            state : vec![1.0; size],
            rng : rand::thread_rng(),
            weights : vec![vec![0.4; size]; size],
            steps : 0,
            numberOfLearnedStates : 0.0,
            nodes_yet_to_update: vect
        }
    }


    pub fn init(&mut self, state : Option<&Vec<f64>>){
        if state.is_some() {
            self.state = state.unwrap().clone();
        } else{
            for i in 0..self.state.len(){
                self.state[i] = if self.rng.gen_range(0..=1) == 1 {1.0} else {-1.0};
            }
        }
        // self.setWeightsToRandom();
        self.steps = 0;
    }

    pub fn get_steps(&self) -> usize{
        self.steps
    }

    pub fn learn(&mut self, state_to_learn: &Vec<f64>){
        self.numberOfLearnedStates += 1.0;
        self.hebbian_learning(state_to_learn);
       
    }

    fn hebbian_learning(&mut self, state_to_learn: &Vec<f64>){
        if self.numberOfLearnedStates == 1.0{
            for i in 0..self.weights.len(){
                for j in 0..self.weights[i].len(){
                    if i == j {
                        self.weights[i][j] = 0.0;
                    }
                    else{
                        self.weights[i][j] = state_to_learn[i] * state_to_learn[j];
                    }
                }
            }
        }else{
            for i in 0..self.weights.len(){
                for j in 0..self.weights[i].len(){
                    if i == j {
                        self.weights[i][j] = 0.0;
                    }
                    else{
                        self.weights[i][j] += (1.0/self.numberOfLearnedStates)*(state_to_learn[i] * state_to_learn[j]) + (self.numberOfLearnedStates - 1.0)/self.numberOfLearnedStates * self.weights[i][j];
                       //self.weights[i][j] += state_to_learn[i] * state_to_learn[j] - self.weights[i][j];
                    }
                }
            }
        }

       
    }

    fn storkey_learning(&mut self, state_to_learn: &Vec<isize>){

    }

    pub fn next_step(&mut self){
        //generat e random index from 0 to state.len()
        //let i = self.rng.gen_range(0..self.state.len());
        if self.nodes_yet_to_update.len() <= 0{
            ClassicNetworkDiscrete::reset_nodes_to_update(&mut self.nodes_yet_to_update, self.state.len())
        }
        let i = self.nodes_yet_to_update.pop().unwrap();
        self.update_node(i);
        self.steps += 1;
    }

    fn update_node(&mut self, i : usize){
        let mut sum = 0.0;
        for j in 0..self.weights[i].len(){
            sum += self.weights[i][j] * self.state[j];
        }
        self.state[i] = if sum > 0.0 {1.0} else {-1.0};
    }

    fn set_weights_to_random(&mut self){
        for i in 0..self.weights.len(){
            for j in 0..self.weights[i].len(){
                self.weights[i][j] = self.rng.gen_range(-5.0..=5.0);
            }
        }

    }

    fn reset_nodes_to_update(container: &mut Vec<usize>, lenght : usize){
        // If the containere isn't already empty, we empty it
        while container.len() != 0{
            container.pop();
        }

        // The container is populated with the indices of the nodes
        for i in 0..lenght{
            container.push(i);
        }

        container.shuffle(&mut rand::thread_rng());
    }


}

impl std::fmt::Display for ClassicNetworkDiscrete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //chek to see if the state lenght is greater then 0
        if self.state.len() <= 0{
            return write!(f, "state: the network has size 0");
        }

        let mut state = String::new();

        // Extract the closest integer square root
        let sqrt = (self.state.len() as f64).sqrt().round() as usize;
       
        //chesk if the lenght of the state is the square of a number
        if sqrt.pow(2) == self.state.len() {
            for i in 0..sqrt{
                for j in 0..sqrt {
                    if self.state[(i*sqrt)+j] == 1.0{
                        state.push_str("◼");
                    }else{
                    state.push_str("◻");
                    }
                }
                state.push_str("\n");
            }
        } else {
            for i in 0..self.state.len(){
                if self.state[i] == 1.0{
                    state.push_str("◼");
                }else{
                state.push_str("◻");
                }
            }
        }


       
        write!(f, "state:\n{}",state)
    }
}