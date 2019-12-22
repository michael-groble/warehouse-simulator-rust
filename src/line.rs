extern crate serde;
extern crate serde_json;

use crate::{checker, picker, ItemPicks, LineMember, SimulationTime};
use serde_json::Value;
use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::io::{BufReader, ErrorKind};
use std::rc::Rc;

pub struct Line {
    members: Vec<Rc<RefCell<dyn LineMember>>>,
    now: SimulationTime,
}

pub enum Parameter {
    Picker(picker::Parameters),
    Checker(checker::Parameters),
}

pub struct Parameters {
    pub members: Vec<Parameter>,
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }
}

pub struct Times {
    pub elapsed: SimulationTime,
    pub idle: SimulationTime,
}

/// A line connects a number of members and connects them so they can pass pick tickets between them.
///
/// # Examples
/// ```
/// # use warehouse_simulator::*;
/// let mut parameters = line::Parameters::new();
/// parameters.members.push(line::Parameter::Picker( picker::Parameters {
///     pickable_items: vec!["A".to_string()].into_iter().collect(),
///     seconds_per_pick_ticket: 1.0,
///     seconds_per_item: 1.0,
///     seconds_per_quantity: 1.0
/// }));
/// parameters.members.push(line::Parameter::Picker( picker::Parameters {
///     pickable_items: vec!["B".to_string()].into_iter().collect(),
///     seconds_per_pick_ticket: 1.0,
///     seconds_per_item: 1.0,
///     seconds_per_quantity: 1.0
/// }));
/// let mut line = Line::new(parameters);
/// let mut pick_ticket = ItemPicks::new();
/// pick_ticket.insert("A", 1);
/// pick_ticket.insert("B", 2);
/// line.process_pick_ticket(&pick_ticket);
/// line.process_pick_ticket(&pick_ticket);
/// line.process_pick_ticket(&pick_ticket);
/// let times = line.times();
/// // time      1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
/// // picker 1: a a a|b b b -|c c c -|
/// // picker 2: - - - a a a a|b b b b|c c c c
/// assert_eq!(times[0].elapsed, 11.0);
/// assert_eq!(times[0].idle,     2.0);
/// assert_eq!(times[1].elapsed, 15.0);
/// assert_eq!(times[1].idle,     3.0);
/// ```
impl Line {
    /// Create parameters from a json file
    ///
    /// # Examples
    /// ```
    ///  # use warehouse_simulator::*;
    ///  # use warehouse_simulator::line::Parameter;
    ///  # use std::collections::HashSet;
    ///  let line_parameters = Line::parameters_from_file("tests/fixtures/simple_line.json").unwrap();
    ///  assert_eq!(line_parameters.members.len(), 3);
    ///
    ///  match &line_parameters.members[0] {
    ///     Parameter::Picker(parameters) => {
    ///         let expected : HashSet<String> = vec!["A".to_string(), "B".to_string()].into_iter().collect();
    ///         let diff : HashSet<_> = parameters.pickable_items.symmetric_difference(&expected).collect();
    ///         assert_eq!(diff.is_empty(), true);
    ///         assert_eq!(parameters.seconds_per_pick_ticket, 0.0);
    ///         assert_eq!(parameters.seconds_per_item, 0.0);
    ///         assert_eq!(parameters.seconds_per_quantity, 0.0);
    ///     }
    ///     _ => { panic!("member is not expected type")}
    /// }
    ///  match &line_parameters.members[1] {
    ///     Parameter::Picker(parameters) => {
    ///         let expected : HashSet<String> = vec!["C".to_string(), "D".to_string()].into_iter().collect();
    ///         let diff : HashSet<_> = parameters.pickable_items.symmetric_difference(&expected).collect();
    ///         assert_eq!(diff.is_empty(), true);
    ///         assert_eq!(parameters.seconds_per_pick_ticket, 0.0);
    ///         assert_eq!(parameters.seconds_per_item, 0.0);
    ///         assert_eq!(parameters.seconds_per_quantity, 0.0);
    ///     }
    ///     _ => { panic!("member is not expected type")}
    /// }
    ///  match &line_parameters.members[2] {
    ///     Parameter::Checker(parameters) => {
    ///         assert_eq!(parameters.check_probability, 0.25);
    ///         assert_eq!(parameters.seconds_per_pick_ticket, 2.0);
    ///         assert_eq!(parameters.seconds_per_item, 0.0);
    ///         assert_eq!(parameters.seconds_per_quantity, 0.0);
    ///     }
    ///     _ => { panic!("member is not expected type")}
    /// }
    /// ```
    pub fn parameters_from_file(filename: &str) -> io::Result<Parameters> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let v: Value = serde_json::from_reader(reader)?;
        let mut line_parameters = Parameters {
            members: Vec::new(),
        };
        for config in v["members"].as_array().unwrap() {
            match config["type"].as_str().unwrap() {
                "Picker" => {
                    let p = config.get("parameters").unwrap();
                    let parameters: picker::Parameters = serde_json::from_value(p.clone())?;
                    line_parameters.members.push(Parameter::Picker(parameters));
                }
                "Checker" => {
                    let p = config.get("parameters").unwrap();
                    let parameters: checker::Parameters = serde_json::from_value(p.clone())?;
                    line_parameters.members.push(Parameter::Checker(parameters));
                }
                member_type => {
                    let error = io::Error::new(
                        ErrorKind::InvalidData,
                        format!("{} is not a valid type", member_type),
                    );
                    return Err(error);
                }
            }
        }
        Ok(line_parameters)
    }

    pub fn new(parameters: Parameters) -> Self {
        let mut members: Vec<Rc<RefCell<dyn LineMember>>> =
            Vec::with_capacity(parameters.members.len());

        for member_parameters in parameters.members.into_iter() {
            match member_parameters {
                Parameter::Checker(parameters) => {
                    let member = checker::Checker::new(parameters);
                    members.push(Rc::new(RefCell::new(member)));
                }
                Parameter::Picker(parameters) => {
                    let member: picker::Picker = picker::Picker::new(parameters);
                    members.push(Rc::new(RefCell::new(member)));
                }
            };
        }

        for i in 1..members.len() {
            let previous = &members[i - 1];
            let current = &members[i];
            previous.borrow_mut().set_next_line_member(current);
        }
        Self { members, now: 0.0 }
    }

    pub fn process_pick_ticket(&mut self, pick_ticket: &ItemPicks) {
        let first = &self.members[0];
        self.now =
            first
                .borrow_mut()
                .process_pick_ticket(self.now, pick_ticket, &mut ItemPicks::new());
    }

    pub fn times(&self) -> Vec<Times> {
        self.members
            .iter()
            .map(|member| {
                let m = member.borrow();
                Times {
                    elapsed: m.elapsed_time(),
                    idle: m.idle_time(),
                }
            })
            .collect()
    }
}
