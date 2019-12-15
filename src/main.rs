use std::collections::HashMap;

type SimulationTime = f64;
type ItemPicks<'a> = HashMap<&'a str, i32>;

pub struct State<'a> {
    now: SimulationTime,
    blocked_until: SimulationTime,
    idle_duration: SimulationTime,
    next_in_line: Option<&'a mut dyn LineMember>,
}

impl<'a> State<'a> {
    fn new() -> State<'a> {
        State {
            now: 0.0,
            blocked_until: 0.0,
            idle_duration: 0.0,
            next_in_line: None,
        }
    }

    fn process_pick_ticket(
        &mut self,
        receive_at: SimulationTime,
        pick_ticket: &ItemPicks,
        updated_contents: &mut ItemPicks,
        work_duration: SimulationTime,
    ) -> SimulationTime {
        self.wait_idle_until(receive_at);
        self.work_for_duration(work_duration);
        self.wait_until_unblocked();
        self.pass_down_line(pick_ticket, updated_contents);
        return self.now;
    }

    fn set_next_line_member<'b: 'a>(&mut self, next_in_line: &'b mut dyn LineMember) {
        self.next_in_line = Some(next_in_line);
        self.blocked_until = 0.0;
    }

    fn pass_down_line(&mut self, pick_ticket: &ItemPicks, contents: &mut ItemPicks) {
        if let Some(next) = &mut self.next_in_line {
            self.blocked_until = next.process_pick_ticket(self.now, pick_ticket, contents);
        }
    }

    fn wait_idle_until(&mut self, time: SimulationTime) {
        let duration = time - self.now;
        if duration > 0.0 {
            self.now = time;
            self.idle_duration += duration;
        }
    }

    fn wait_until_unblocked(&mut self) {
        self.wait_idle_until(self.blocked_until);
    }

    fn work_for_duration(&mut self, duration: SimulationTime) {
        self.now += duration;
    }
}

pub trait LineMember {
    fn process_pick_ticket(
        &mut self,
        receive_at: SimulationTime,
        pick_ticket: &ItemPicks,
        contents: &mut ItemPicks,
    ) -> SimulationTime;
}

pub struct Picker<'a> {
    state: State<'a>,
}

impl<'a> Picker<'a> {
    fn new() -> Picker<'a> {
        Picker {
            state: State::new(),
        }
    }
}

impl LineMember for Picker<'_> {
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

fn main() {
    let mut p1 = Picker::new();
    let mut p2 = Picker::new();
    p1.state.set_next_line_member(&mut p2);
    let mut pick_ticket = ItemPicks::new();
    pick_ticket.insert("A", 1);
    pick_ticket.insert("B", 2);
    let mut contents = ItemPicks::new();
    let result = p1.process_pick_ticket(0.0, &pick_ticket, &mut contents);
    println!("{}, {}", result, contents["A"]);
}
