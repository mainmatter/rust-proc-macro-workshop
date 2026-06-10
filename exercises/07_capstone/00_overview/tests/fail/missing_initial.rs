use state_machine_exercise::StateMachine;

// Every machine needs a starting state: with no `#[initial]` the derive is
// rejected.
#[derive(StateMachine)]
enum TrafficLight {
    Red,
    Green,
}

fn main() {}
