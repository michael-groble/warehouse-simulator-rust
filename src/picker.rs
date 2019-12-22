extern crate serde;
use crate::line_member::State;
use crate::*;
use std::collections::HashSet;

#[derive(Default, serde::Deserialize)]
pub struct Parameters {
    #[serde(default)]
    pub pickable_items: HashSet<String>,
    #[serde(default)]
    pub seconds_per_pick_ticket: SimulationTime,
    #[serde(default)]
    pub seconds_per_item: SimulationTime,
    #[serde(default)]
    pub seconds_per_quantity: SimulationTime,
}

pub struct Picker<'a> {
    state: State<'a>,
    parameters: Parameters,
}

impl<'a> Picker<'a> {
    pub fn new(parameters: Parameters) -> Self {
        Self {
            state: State::new(),
            parameters,
        }
    }

    fn pick_duration_and_update_contents<'b>(
        &self,
        pick_ticket: &ItemPicks<'b>,
        contents: &mut ItemPicks<'b>,
    ) -> SimulationTime {
        let picks: ItemPicks = pick_ticket
            .iter()
            .filter(|(&k, _v)| self.parameters.pickable_items.contains(k))
            .map(|(&k, &v)| (k, v))
            .collect();
        let item_count = picks.len();
        let pick_count: PickQuantity = picks.iter().map(|(_k, &v)| v).sum();
        let duration = self.parameters.seconds_per_pick_ticket
            + item_count as SimulationTime * self.parameters.seconds_per_item
            + pick_count as SimulationTime * self.parameters.seconds_per_quantity;

        for (k, v) in picks {
            contents.insert(k, v);
        }

        duration
    }
}

impl<'a> LineMember<'a> for Picker<'a> {
    /// Processes the pick ticket based on configured parameters
    ///
    /// # Examples
    /// ```
    /// # use warehouse_simulator::*;
    ///
    /// let mut p1 = Picker::new(picker::Parameters {
    ///    pickable_items: vec!["A".to_string()].into_iter().collect(),
    ///    seconds_per_item: 1.0,
    ///    ..Default::default()
    ///  });
    /// let mut pick_ticket = ItemPicks::new();
    /// pick_ticket.insert("A", 1);
    /// pick_ticket.insert("B", 2);
    /// let mut contents = ItemPicks::new();
    /// let duration = p1.process_pick_ticket(0.0, &pick_ticket, &mut contents);
    /// assert_eq!(duration, 1.0);
    /// assert_eq!(contents["A"], 1);
    /// assert_eq!(contents.contains_key("B"), false);
    /// ```
    fn process_pick_ticket<'b>(
        &mut self,
        receive_at: SimulationTime,
        pick_ticket: &ItemPicks<'b>,
        contents: &mut ItemPicks<'b>,
    ) -> SimulationTime {
        let duration = self.pick_duration_and_update_contents(pick_ticket, contents);
        self.state
            .process_pick_ticket(receive_at, pick_ticket, contents, duration)
    }

    fn set_next_line_member(&mut self, next_in_line: &'a mut dyn LineMember<'a>) {
        self.state.set_next_line_member(next_in_line);
    }
}
