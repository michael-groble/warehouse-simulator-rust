use crate::{ItemPicks, SimulationTime};
use std::cell::RefCell;
use std::rc::Rc;

pub trait LineMember: Stateful {
    fn process_pick_ticket<'a>(
        &mut self,
        receive_at: SimulationTime,
        pick_ticket: &ItemPicks<'a>,
        contents: &mut ItemPicks<'a>,
    ) -> SimulationTime;

    fn set_next_line_member(&mut self, next_in_line: &Rc<RefCell<dyn LineMember>>) {
        self.state_mut().set_next_line_member(next_in_line);
    }

    fn elapsed_time(&self) -> SimulationTime {
        self.state().now
    }

    /// Time spent idle (blocked or waiting)
    fn idle_time(&self) -> SimulationTime {
        self.state().idle_duration
    }
}

pub struct State {
    now: SimulationTime,
    blocked_until: SimulationTime,
    idle_duration: SimulationTime,
    next_in_line: Option<Rc<RefCell<dyn LineMember>>>,
}

impl State {
    pub fn new() -> State {
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
    pub fn process_pick_ticket<'a>(
        &mut self,
        receive_at: SimulationTime,
        pick_ticket: &ItemPicks<'a>,
        updated_contents: &mut ItemPicks<'a>,
        work_duration: SimulationTime,
    ) -> SimulationTime {
        self.wait_idle_until(receive_at);
        self.work_for_duration(work_duration);
        self.wait_until_unblocked();
        self.pass_down_line(pick_ticket, updated_contents);
        self.now
    }

    pub fn set_next_line_member(&mut self, next_in_line: &Rc<RefCell<dyn LineMember>>) {
        self.next_in_line = Some(next_in_line.clone());
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
            self.blocked_until =
                next.borrow_mut()
                    .process_pick_ticket(self.now, pick_ticket, contents);
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

pub trait Stateful {
    fn state(&self) -> &State;
    fn state_mut(&mut self) -> &mut State;
}
