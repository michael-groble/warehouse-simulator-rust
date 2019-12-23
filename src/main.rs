extern crate warehouse_simulator;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use structopt::StructOpt;
use warehouse_simulator::*;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    line_member_path: PathBuf,
    #[structopt(parse(from_os_str))]
    pick_ticket_path: PathBuf,
}

fn main() {
    let options = Options::from_args();
    let line_parameters = Line::parameters_from_file(options.line_member_path).unwrap();
    let mut line = Line::new(line_parameters);
    let file = File::open(options.pick_ticket_path).unwrap();
    let reader = io::BufReader::new(file);
    for picks in reader.lines() {
        let mut pick_ticket = ItemPicks::new();
        let picks = picks.unwrap();
        for s in picks.split('\t') {
            let pair: Vec<&str> = s.split(':').collect();
            let quantity: PickQuantity = pair[1].parse().unwrap();
            pick_ticket.insert(pair[0], quantity);
        }
        line.process_pick_ticket(&pick_ticket);
    }
    let mut max: SimulationTime = 0.0;
    line.times().iter().for_each(|t| max = max.max(t.elapsed));
    println!("Elapsed: {}", max);
    println!("Idle times:");
    for time in line.times() {
        let percent_idle = (100.0 * time.idle / max).round();
        println!("{} ({}%)", time.idle, percent_idle);
    }
}
