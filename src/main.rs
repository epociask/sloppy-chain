use api;
use state::NodeState;

fn main() {
    // TODO - change this to an Arc once multiple state dependent routines are introduced
    let node_state: NodeState = NodeState::new();

    api::main();
    
}
