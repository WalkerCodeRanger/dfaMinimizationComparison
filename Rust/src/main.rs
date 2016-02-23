pub mod min;
pub mod fa;

use std::io::{stdin, stdout, BufRead, Write};
use std::str::FromStr;
use fa::DFA;

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
		state_count = usize::from_str(header[0]).unwrap();
		transition_count = usize::from_str(header[1]).unwrap();
		start_state = usize::from_str(header[2]).unwrap();
		final_state_count = usize::from_str(header[3]).unwrap();
	}

	// Create DFA
	let mut dfa = DFA::new(state_count, start_state);

	// Read Transitions
	for _ in 0..transition_count
	{
		stdin.read_line(&mut buffer).unwrap();
		let transition: Vec<&str> = buffer.split(' ').collect();
		let from_state = usize::from_str(transition[0]).unwrap();
		let input = i32::from_str(transition[0]).unwrap();
		let to_state = usize::from_str(transition[0]).unwrap();
		dfa.add_transition(from_state, input, to_state);
	}

	// Read Final States
	for _ in 0..final_state_count
	{
		stdin.read_line(&mut buffer).unwrap();
		let final_state = usize::from_str(&buffer).unwrap();
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

	for state in dfa.final_states()
	{
		writeln!(&mut stdout, "{}", state).unwrap();
	}
}