use std::io::{stdin, stdout, BufRead, Write};
use std::str::FromStr;
use std::collections::HashSet;

fn main()
{
	let dfa = read_dfa();
	let min_dfa = dfa.minimize();
	print_dfa(&min_dfa);
}

fn read_dfa() -> DFA
{
	let console = stdin(); // Can't inline this because "borrowed value does not live long enough"
	let mut stdin = console.lock();
	let mut buffer = String::new();

	// Read Header
	let state_count;
	let transition_count;
	let start_state;
	let final_state_count;
	stdin.read_line(&mut buffer).unwrap();
	{
		let header: Vec<&str> = buffer.split(' ').collect();
		state_count = i32::from_str(header[0]).unwrap();
		transition_count = i32::from_str(header[1]).unwrap();
		start_state = i32::from_str(header[2]).unwrap();
		final_state_count = i32::from_str(header[3]).unwrap();
	}

	// Create DFA
	let mut dfa = DFA::new(state_count, start_state);

	// Read Transitions
	for _ in 0..transition_count
	{
		stdin.read_line(&mut buffer).unwrap();
		let transition: Vec<&str> = buffer.split(' ').collect();
		let from_state = i32::from_str(transition[0]).unwrap();
		let input = i32::from_str(transition[0]).unwrap();
		let to_state = i32::from_str(transition[0]).unwrap();
		dfa.add_transition(from_state, input, to_state);
	}

	// Read Final States
	for _ in 0..final_state_count
	{
		stdin.read_line(&mut buffer).unwrap();
		let final_state = i32::from_str(&buffer).unwrap();
		dfa.add_final_state(final_state);
	}

	return dfa;
}

fn print_dfa(dfa: &DFA)
{
	let console = stdout();
	let mut stdout = console.lock();

	// Header
	writeln!(&mut stdout, "{0} {1} {2} {3}", dfa.state_count(), dfa.transitions().len(), dfa.start_state(), dfa.final_states().len()).unwrap();
	for transition in dfa.transitions()
	{
		writeln!(&mut stdout, "{0} {1} {2}", transition.from, transition.on_input, transition.to).unwrap();
	}
	unimplemented!();
}

struct DFA
{
	state_count: i32,
	start_state: i32,
	transitions: Vec<Transition>,
	final_states: HashSet<i32>
}

impl DFA
{
	pub fn new(state_count: i32, start_state: i32) -> DFA
	{
		DFA {state_count: state_count, start_state: start_state, transitions: Vec::new(), final_states: HashSet::new()}
	}

	pub fn state_count(&self) -> i32
	{
		self.state_count
	}

	pub fn start_state(&self) -> i32
	{
		self.start_state
	}

	pub fn add_transition(&mut self, from_state: i32, on_input: i32, to_state: i32)
	{
		self.transitions.push(Transition {from: from_state, on_input: on_input, to: to_state});
	}

	pub fn add_final_state(&mut self, state: i32) -> bool
	{
		self.final_states.insert(state)
	}

	pub fn minimize(&self) -> DFA
	{
		unimplemented!();
	}

	pub fn transitions(&self) -> &Vec<Transition>
	{
		&self.transitions
	}

	pub fn final_states(&self) -> &HashSet<i32>
	{
		&self.final_states
	}
}

struct Transition
{
	pub from: i32,
	pub on_input: i32,
	pub to: i32
}