use crate::line_member::State;
use crate::*;

pub struct Picker<'a> {
    state: State<'a>,
}

impl<'a> Picker<'a> {
    pub fn new() -> Picker<'a> {
        Picker {
            state: State::new(),
        }
    }

    pub fn set_next_line_member<'b: 'a>(&mut self, next_in_line: &'b mut dyn LineMember) {
        self.state.set_next_line_member(next_in_line);
    }
}

impl<'a> LineMember for Picker<'a> {
    fn process_pick_ticket(
        &mut self,
        receive_at: SimulationTime,
        pick_ticket: &ItemPicks,
        contents: &mut ItemPicks,
    ) -> SimulationTime {
        let count = contents.entry("A").or_insert(0);
        *count += 1;
        return self
            .state
            .process_pick_ticket(receive_at, pick_ticket, contents, 1.0);
    }
}
