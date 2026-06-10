use state_machine_exercise::StateMachine;

// A state is just a name, so a variant that carries data is rejected — the error
// points at the offending variant.
#[derive(StateMachine)]
enum TrafficLight {
    #[initial]
    Red(u8),
    Green,
}

fn main() {}
