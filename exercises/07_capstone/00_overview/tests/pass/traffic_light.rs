use state_machine_exercise::StateMachine;

// A classic three-state cycle: Red -> Green -> Yellow -> Red.
#[derive(StateMachine, Debug, PartialEq)]
enum TrafficLight {
    #[initial]
    #[transition(Green)]
    Red,
    #[transition(Yellow)]
    Green,
    #[transition(Red)]
    Yellow,
}

fn main() {
    // The `#[initial]` state.
    assert_eq!(TrafficLight::initial(), TrafficLight::Red);

    // Declared edges are allowed; everything else is not.
    assert!(TrafficLight::Red.can_transition_to(&TrafficLight::Green));
    assert!(!TrafficLight::Red.can_transition_to(&TrafficLight::Yellow));
    assert!(!TrafficLight::Red.can_transition_to(&TrafficLight::Red));

    // A legal move hands back the new state.
    assert_eq!(
        TrafficLight::Red.transition_to(TrafficLight::Green).unwrap(),
        TrafficLight::Green,
    );

    // An illegal move reports a useful error built from the generated `name()`s.
    let err = TrafficLight::Red
        .transition_to(TrafficLight::Yellow)
        .unwrap_err();
    assert_eq!(err.to_string(), "invalid transition from Red to Yellow");
}
