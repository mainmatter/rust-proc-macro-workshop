use state_machine_exercise::StateMachine;

// A state machine's states are an enum's variants, so deriving it on a struct
// must be rejected.
#[derive(StateMachine)]
struct TrafficLight {
    red: bool,
}

fn main() {}
