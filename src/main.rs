extern crate warehouse_simulator;
use std::cell::RefCell;
use std::rc::Rc;
use warehouse_simulator::*;

fn main() {
    let mut p1 = Picker::new(picker::Parameters {
        pickable_items: vec!["A".to_string()].into_iter().collect(),
        seconds_per_item: 1.0,
        ..Default::default()
    });
    let p2 = Picker::new(picker::Parameters {
        pickable_items: vec!["B".to_string()].into_iter().collect(),
        seconds_per_item: 1.0,
        ..Default::default()
    });
    let r: Rc<RefCell<dyn LineMember>> = Rc::new(RefCell::new(p2));
    p1.set_next_line_member(&r);
    let mut pick_ticket = ItemPicks::new();
    pick_ticket.insert("A", 1);
    pick_ticket.insert("B", 2);
    let mut contents = ItemPicks::new();
    let result = p1.process_pick_ticket(0.0, &pick_ticket, &mut contents);
    println!("{}, {}", result, contents["A"]);
}
