
#[derive(Debug, Clone)]
enum TrafficLightState {
    Red,
    Yellow,
    Green,
}

#[derive(Debug, Clone)]
enum TrafficLightEvent {
    SwitchToRed,
    SwitchToYellow,
    SwitchToGreen,
}

#[derive(Debug, Clone)]
struct TrafficLight {
    state: TrafficLightState,
}

impl TrafficLight {
    fn new(initial_state: TrafficLightState) -> TrafficLight {
        TrafficLight {
            state: initial_state,
        }
    }

    fn transition(&mut self, event: TrafficLightEvent) {
        match (self.state.clone(), event.clone()) {
            (TrafficLightState::Red, TrafficLightEvent::SwitchToGreen) => {
                self.state = TrafficLightState::Green;
            }
            (TrafficLightState::Green, TrafficLightEvent::SwitchToYellow) => {
                self.state = TrafficLightState::Yellow;
            }
            (TrafficLightState::Yellow, TrafficLightEvent::SwitchToRed) => {
                self.state = TrafficLightState::Red;
            }
            _ => {
                println!("Invalid transition from {:?} to {:?}", self.state, event);
            }
        }
    }
}

fn main() {
    let mut traffic_light = TrafficLight::new(TrafficLightState::Red);
    println!("Initial state: {:?}", traffic_light.state);

    traffic_light.transition(TrafficLightEvent::SwitchToGreen);
    println!("State after switching to green: {:?}", traffic_light.state);

    traffic_light.transition(TrafficLightEvent::SwitchToYellow);
    println!("State after switching to yellow: {:?}", traffic_light.state);

    traffic_light.transition(TrafficLightEvent::SwitchToRed);
    println!("State after switching to red: {:?}", traffic_light.state);

    traffic_light.transition(TrafficLightEvent::SwitchToRed);
    println!("State after switching to red: {:?}", traffic_light.state);
}



