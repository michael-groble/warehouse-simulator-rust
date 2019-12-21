extern crate warehouse_simulator;
use warehouse_simulator::*;

fn main() {
    let mut p1 = Picker::new(picker::Parameters {
        pickable_items: vec!["A"].into_iter().collect(),
        seconds_per_item: 1.0,
        ..Default::default()
    });
    let mut p2 = Picker::new(picker::Parameters {
        pickable_items: vec!["B"].into_iter().collect(),
        seconds_per_item: 1.0,
        ..Default::default()
    });
    p1.set_next_line_member(&mut p2);
    let mut pick_ticket = ItemPicks::new();
    pick_ticket.insert("A", 1);
    pick_ticket.insert("B", 2);
    let mut contents = ItemPicks::new();
    let result = p1.process_pick_ticket(0.0, &pick_ticket, &mut contents);
    println!("{}, {}", result, contents["A"]);
}
