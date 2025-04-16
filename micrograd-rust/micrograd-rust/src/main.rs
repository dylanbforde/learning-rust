
mod neural_network;
mod helper_functions;
mod data;

use neural_network::NeuralNetwork;

fn main() {
    let d = data::get_data().unwrap();

    let inputs = d.training_inputs;
    let outputs = d.training_outputs;
    let test_inputs = d.test_inputs;

    let mut neural_net = NeuralNetwork::new();

}
