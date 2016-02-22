using System;
using System.Linq;

namespace DfaMinComparisonCSharp.CSharp
{
	public class Program
	{
		public static void Main(string[] args)
		{
			// Read Header
			var header = Console.ReadLine().Split();
			var stateCount = int.Parse(header[0]);
			var transitionCount = int.Parse(header[1]);
			var initialState = int.Parse(header[2]);
			var finalStateCount = int.Parse(header[3]);

			// Create DFA
			var dfa = new DFA(stateCount, initialState);

			// Read Transitions
			for(var i = 0; i < transitionCount; i++)
			{
				var transition = Console.ReadLine().Split();
				var fromState = int.Parse(transition[0]);
				var input = int.Parse(transition[1]);
				var toState = int.Parse(transition[2]);
				dfa.AddTransition(fromState, input, toState);
			}

			// Read Final States
			for(var i = 0; i < finalStateCount; i++)
			{
				var state = int.Parse(Console.ReadLine());
				dfa.SetFinal(state);
			}

			var minDfa = dfa.Minimize();

			// Print Results
			var finalStates = minDfa.States.Where(s => minDfa.IsFinal(s)).ToList();
			// Header
			Console.WriteLine($"{minDfa.StateCount} {minDfa.Transitions.Count} {minDfa.StartState} {finalStates.Count}");
			// Transitions
			foreach(var transition in minDfa.Transitions)
				Console.WriteLine($"{transition.From} {transition.OnInput} {transition.To}");
			// Final States
			foreach(var finalState in finalStates)
				Console.WriteLine(finalState);
		}
	}
}
