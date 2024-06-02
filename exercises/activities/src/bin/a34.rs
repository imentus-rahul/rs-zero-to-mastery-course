// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
#[derive(Debug)]
struct Luggage<State> {
    tracking_id: String,
    state: State, // generic T,U,V == State
}

impl<State> Luggage<State> {
    fn transition<NextState>(self, state: NextState) -> Luggage<NextState> {
        Luggage {
            tracking_id: self.tracking_id,
            state: state,
        }
    }
}

#[derive(Debug)]
struct CheckIn;
#[derive(Debug)]
struct OnLoading;
#[derive(Debug)]
struct Offloading;
#[derive(Debug)]
struct AwaitingPickup;
#[derive(Debug)]
struct EndCustody;

impl Luggage<CheckIn> {
    fn new(tracking_id: &str) -> Self {
        Self {
            tracking_id: tracking_id.to_string(),
            state: CheckIn,
        }
    }

    fn transit_from_check_in(self) -> Luggage<OnLoading> {
        println!("🚀 ~ file: a34.rs ~ line 42 ~ fn transit_from_check_in ~ self {:?}", self);
        self.transition(OnLoading)
    }
}

impl Luggage<OnLoading> {
    // fn new(tracking_id: &str) -> Self{
    //     Self { tracking_id: tracking_id.to_string(), state: OnLoading }
    // }

    fn transit_from_on_loading(self) -> Luggage<Offloading> {
        println!("🚀 ~ file: a34.rs ~ line 47 ~ fn transit_from_on_loading ~ self {:?}", self);
        self.transition(Offloading)
    }
}

impl Luggage<Offloading> {
    // fn new(tracking_id: &str) -> Self{
    //     Self { tracking_id: tracking_id.to_string(), state: Offloading }
    // }

    fn transit_from_off_loading(self) -> Luggage<AwaitingPickup> {
        println!("🚀 ~ file: a34.rs ~ line 63 ~ fn transit_from_off_loading ~ self {:?}", self);
        self.transition(AwaitingPickup)
    }
}

impl Luggage<AwaitingPickup> {
    // fn new(tracking_id: &str) -> Self{
    //     Self { tracking_id: tracking_id.to_string(), state: AwaitingPickup }
    // }

    fn transit_from_awaiting_pickup(self) -> Luggage<EndCustody> {
        println!("🚀 ~ file: a34.rs ~ line 74 ~ fn transit_from_awaiting_pickup ~ self {:?}", self);
        self.transition(EndCustody)
    }
}

impl Luggage<EndCustody> {
    // fn new(tracking_id: &str) -> Self{
    //     Self { tracking_id: tracking_id.to_string(), state: EndCustody }
    // }

    fn transit_from_end_custody(self) -> Luggage<EndCustody> {
        println!("🚀 ~ file: a34.rs ~ line 85 ~ fn transit_from_end_custody ~ self {:?}", self);
        self.transition(EndCustody)
    }
}
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

fn main() {
    let tracking_id = "some_tracking_id";
    let luggage = Luggage::new(tracking_id);
    luggage
        .transit_from_check_in()
        .transit_from_on_loading()
        .transit_from_off_loading()
        .transit_from_awaiting_pickup()
        .transit_from_end_custody();
}
