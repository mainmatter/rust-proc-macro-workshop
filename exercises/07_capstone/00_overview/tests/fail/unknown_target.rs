use state_machine_exercise::StateMachine;

// `Purple` is not a variant of the enum, so the `#[transition(Purple)]` must be
// rejected at compile time with an error pointing straight at `Purple`.
#[derive(StateMachine)]
enum TrafficLight {
    #[initial]
    #[transition(Purple)]
    Red,
    Green,
}

fn main() {}
