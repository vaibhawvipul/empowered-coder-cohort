#[derive(Debug, Clone)]
enum TrafficLightState {
    Red,
    Yellow,
    Green,
}
#[derive(Debug, Clone)]
enum TraffcLightEvent {
    SwitchToRed,
    SwitchToYellow,
    SwitchToGreen,
}

#[derive(Debug, Clone)]
struct TrafficLight {
    state: TrafficLightState,
}

impl TrafficLight {
    fn new(initial_state: TrafficLightState) -> Self {
        TrafficLight {
            state: initial_state,
        }
    }

    fn transition(&mut self, event: TraffcLightEvent) {
        // match the current state and event to determine the next state
        match (self.state.clone(), event.clone()) {
            (TrafficLightState::Red, TraffcLightEvent::SwitchToGreen) => {
                println!("Red -> Green");
                self.state = TrafficLightState::Green;
            }
            (TrafficLightState::Green, TraffcLightEvent::SwitchToYellow) => {
                println!("Green -> Yellow");
                self.state = TrafficLightState::Yellow;
            }
            (TrafficLightState::Yellow, TraffcLightEvent::SwitchToRed) => {
                println!("Yellow -> Red");
                self.state = TrafficLightState::Red;
            }
            _ => {
                println!("Invalid transition from {:?} with {:?}",
                         self.state, event);
            }
        }
    }
}


fn main() {
    println!("Traffic Light State Machine");

    let mut traffic_light = TrafficLight::new(TrafficLightState::Red);

    // demonstrate the state machine transitions
    traffic_light.transition(TraffcLightEvent::SwitchToGreen);
    traffic_light.transition(TraffcLightEvent::SwitchToYellow);
    traffic_light.transition(TraffcLightEvent::SwitchToRed);

    // demonstrate an invalid transition
    traffic_light.transition(TraffcLightEvent::SwitchToGreen);
    traffic_light.transition(TraffcLightEvent::SwitchToRed);
    traffic_light.transition(TraffcLightEvent::SwitchToGreen);

}
