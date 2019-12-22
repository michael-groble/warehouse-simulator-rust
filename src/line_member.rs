use crate::{ItemPicks, SimulationTime};

pub trait LineMember<'a> {
    fn process_pick_ticket<'b>(
        &mut self,
        receive_at: SimulationTime,
        pick_ticket: &ItemPicks<'b>,
        contents: &mut ItemPicks<'b>,
    ) -> SimulationTime;

    fn set_next_line_member(&mut self, next_in_line: &'a mut dyn LineMember<'a>);
}

pub struct State<'a> {
    now: SimulationTime,
    blocked_until: SimulationTime,
    idle_duration: SimulationTime,
    next_in_line: Option<&'a mut dyn LineMember<'a>>,
}

impl<'a> State<'a> {
    pub fn new() -> State<'a> {
        State {
            now: 0.0,
            blocked_until: 0.0,
            idle_duration: 0.0,
            next_in_line: None,
        }
    }

    /// Performs the expected state update given how much work time is required and what the updated contents
    /// are to pass down the line.
    ///
    /// Returns the current value of [`State::elapsed_time`]
    ///
    /// # Examples
    /// ```
    /// # use warehouse_simulator::*;
    /// # use warehouse_simulator::line_member::State;
    /// let mut state = State::new();
    ///
    /// let duration = state.process_pick_ticket(1.0, &ItemPicks::new(), &mut ItemPicks::new(), 1.0);
    /// assert_eq!(duration, 2.0);
    /// assert_eq!(state.elapsed_time(), 2.0);
    /// assert_eq!(state.idle_time(), 1.0);
    /// ```
    pub fn process_pick_ticket<'b>(
        &mut self,
        receive_at: SimulationTime,
        pick_ticket: &ItemPicks<'b>,
        updated_contents: &mut ItemPicks<'b>,
        work_duration: SimulationTime,
    ) -> SimulationTime {
        self.wait_idle_until(receive_at);
        self.work_for_duration(work_duration);
        self.wait_until_unblocked();
        self.pass_down_line(pick_ticket, updated_contents);
        self.now
    }

    pub fn set_next_line_member(&mut self, next_in_line: &'a mut dyn LineMember<'a>) {
        self.next_in_line = Some(next_in_line);
        self.blocked_until = 0.0;
    }

    /// Total time elapsed until the member can accept another work item
    ///
    /// Note they may currently be blocked until after this time, but that won't show up
    /// until a new pick ticket is processed.
    pub fn elapsed_time(&self) -> SimulationTime {
        self.now
    }

    /// Time spent idle (blocked or waiting)
    pub fn idle_time(&self) -> SimulationTime {
        self.idle_duration
    }

    fn pass_down_line<'b>(&mut self, pick_ticket: &ItemPicks<'b>, contents: &mut ItemPicks<'b>) {
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
