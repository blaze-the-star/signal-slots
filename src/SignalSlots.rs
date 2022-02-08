
// --- Use ---

use std::collections::{HashMap, HashSet};
use rand::thread_rng;


// --- Types ---

pub type InputCallable = fn(is_on:bool,power:f32)->();

// --- Structs ---

pub struct StateTree {
	inputs:HashMap<usize, Input>,
	outputs:HashMap<usize, Output>,
	connections:HashMap<usize, HashSet<usize>>,

} impl StateTree {
	pub fn new() -> Self{
		Self{
			inputs:HashMap::new(),
			outputs:HashMap::new(),
			connections:HashMap::new(),
		}
	}

	pub fn add_input_call(&mut self, input_id:usize, callee: InputCallable) {
		self
			.inputs
			.get(&input_id)
			.unwrap()
			.callees
			.push(callee);
	}

	pub fn emit_slots(&mut self, slot:usize, power:f32) {
		if self.connections.contains_key(&slot) {
			let inputs = self.connections.get(&slot).unwrap();
			for inp_slot in inputs {
				for call in &self.inputs.get(&inp_slot).unwrap().callees {
					call(true,1.0);
				}
			}
		}
	}

	pub fn connect_slots(&mut self, from:usize, to:usize) {
		let connected = self.connections.entry(from).or_insert(HashSet::new());
		if !connected.contains(&to) {
			connected.insert(to);
		}
	}

	fn new_input_id(&self) -> usize {
		let mut rng = thread_rng();
		loop {
			use rand::Rng;
			let id = rng.gen() ;
			if self.inputs.contains_key(&id) {
				continue;
			}
			return id;
		}
	}

	pub fn new_input(&mut self) -> usize {
		let mut rng = thread_rng();
		let mut id = self.new_input_id();

		let slot = Input::new(id);
		self.inputs.insert(id.clone(), slot);

		return id;
	}

	fn new_output_id(&self) -> usize {
		let mut rng = thread_rng();
		loop {
			use rand::Rng;
			let id:usize = rng.gen() ;
			if self.outputs.contains_key(&id) {
				continue;
			}
			return id;
		};
		return 0;
	}

	pub fn new_output(&mut self) -> usize {
		let mut rng = thread_rng();
		let mut id = self.new_output_id();

		let slot = Output::new(id);
		self.outputs.insert(id.clone(), slot);

		return id;
	}
}

pub struct Hub {
	usize:i32,

}

pub struct Output {
	usize:usize,

} impl Output {
	fn new(usize_:usize) -> Self {
		Output{
			usize:usize_,
		}
	}
}

pub struct Input {
	pub usize:usize,
	pub is_on:bool,
	pub power:f32,
	pub callees:Vec<InputCallable>,
} impl Input {
	fn activation(&mut self, power_:f32) {
		self.power = power_;
		self.is_on = (self.is_on && self.power > 0.33) || (!self.is_on && self.power > 0.66);
	}
	fn new(usize_:usize) -> Self {
		Input{
			usize:usize_,
			is_on:false,
			power:0.0,
			callees:vec![],
		}
	}
	
}

pub struct Connection<'a, I, O> {
	pub from:&'a I,
	pub to:&'a O,
}

// --- Traits ---

pub trait Activatable: Sized {
	fn activation(&self, power:f32);
	fn new() -> Self;
}

pub trait Activator: Sized {
	fn activate(&self, power:f32);
	fn new() -> Self;
}

pub trait Slot: Sized {
	fn get_usize(&self) -> &usize;
	fn set_usize(&self, usize:usize);
}
