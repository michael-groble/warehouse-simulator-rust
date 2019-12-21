extern crate rand;

use crate::line_member::State;
use crate::*;
use rand::Rng;

pub struct Parameters {
    pub check_probability: f32,
    pub seconds_per_pick_ticket: SimulationTime,
    pub seconds_per_item: SimulationTime,
    pub seconds_per_quantity: SimulationTime,
}

impl Default for Parameters {
    fn default() -> Self  {
        Self {
            check_probability: 1.0,
            seconds_per_pick_ticket: 0.0,
            seconds_per_item: 0.0,
            seconds_per_quantity: 0.0
        }
    }
}

pub struct Checker<'a> {
    state: State<'a>,
    parameters: Parameters,
}

impl<'a> Checker<'a> {
    pub fn new(parameters: Parameters) -> Checker<'a> {
        Checker {
            state: State::new(),
            parameters
        }
    }

    fn check_duration(
        &self,
        _pick_ticket: &ItemPicks,
        contents: &ItemPicks,
    ) -> SimulationTime {
        let p : f32 = rand::thread_rng().gen();
        println!("{}", p);
        if  p > self.parameters.check_probability {
            return 0.0
        }
        else {
            let item_count = contents.len();
            println!("{}", item_count);
            let pick_count: PickQuantity = contents.iter().map(|(_k, &v)| v).sum();
            let duration = self.parameters.seconds_per_pick_ticket
                + item_count as SimulationTime * self.parameters.seconds_per_item
                + pick_count as SimulationTime * self.parameters.seconds_per_quantity;

            return duration;
        }
    }
}

impl<'a> LineMember<'a> for Checker<'a> {

    /// Processes the pick ticket based on configured parameters
    ///
    /// # Examples
    /// ```
    /// # use warehouse_simulator::*;
    ///
    /// let mut p1 = Checker::new(checker::Parameters {
    ///    seconds_per_item: 1.0,
    ///    ..Default::default()
    ///  });
    /// let mut pick_ticket = ItemPicks::new();
    /// let mut contents = ItemPicks::new();
    /// contents.insert("A", 1);
    /// contents.insert("B", 2);
    /// let duration = p1.process_pick_ticket(0.0, &pick_ticket, &mut contents);
    /// assert_eq!(duration, 2.0);
    /// ```
    fn process_pick_ticket<'b>(
        &mut self,
        receive_at: SimulationTime,
        pick_ticket: &ItemPicks<'b>,
        contents: &mut ItemPicks<'b>,
    ) -> SimulationTime {
        let duration = self.check_duration(pick_ticket, contents);
        return self
            .state
            .process_pick_ticket(receive_at, pick_ticket, contents, duration);
    }

    fn set_next_line_member(&mut self, next_in_line: &'a mut dyn LineMember<'a>) {
        self.state.set_next_line_member(next_in_line);
    }
}