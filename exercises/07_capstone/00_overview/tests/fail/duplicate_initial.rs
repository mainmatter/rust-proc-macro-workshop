use state_machine_exercise::StateMachine;

// A machine has exactly one starting state, so two `#[initial]` variants are
// rejected — the error points at the second one.
#[derive(StateMachine)]
enum TrafficLight {
    #[initial]
    Red,
    #[initial]
    Green,
}

fn main() {}
