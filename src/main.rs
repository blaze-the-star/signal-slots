
mod SignalSlots;

use SignalSlots::{Hub, StateTree, Input};

fn callee(is_on:bool,power:f32)->() {
    print!("On state: {is_on} at power: {power}");
}

fn main() {
    println!("Hello, world!");
    let mut tree = StateTree::new();
    let o_id = tree.new_output();
    let i_id = tree.new_input();
    tree.connect_slots(o_id, i_id);
    tree.emit_slots(o_id, 0.5);
    println!("End");
}
