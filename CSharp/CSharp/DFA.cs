using System;
using System.Collections.Generic;

namespace DfaMinComparisonCSharp.CSharp
{
	public class DFA
	{
		private readonly List<Transition> transitions = new List<Transition>();
		private readonly HashSet<int> finalStates = new HashSet<int>();

		public DFA(int stateCount, int startState)
		{
			StateCount = stateCount;
			StartState = startState;
		}

		public int StateCount { get; }
		public int StartState { get; }
		public IReadOnlyList<Transition> Transitions => transitions;
		public IReadOnlyCollection<int> FinalStates => finalStates;

		public void AddTransition(int fromState, int onInput, int toState)
		{
			transitions.Add(new Transition(fromState, onInput, toState));
		}
		public void AddFinalState(int state)
		{
			finalStates.Add(state);
		}

		public DFA Minimize()
		{
			// We will be modifying this list of transitions and we don't want to mess up our own
			var transitions = new List<Transition>(this.transitions);

			var blocks = new Partition(StateCount);

			// Reachable from start
			blocks.Mark(StartState);

			DiscardNotReachable(blocks, transitions, t => t.From, t => t.To);

			// Reachable from final
			foreach(var finalState in finalStates)
				blocks.Mark(finalState);

			DiscardNotReachable(blocks, transitions, t => t.To, t => t.From);

			// Split final states from non-final
			foreach(var finalState in finalStates)
				blocks.Mark(finalState);

			blocks.SplitSets();

			// Cords partition to manage transitions
			var cords = new Partition(transitions.Count);

			// Split transitions by input
			cords.PartitionBy(transition => transitions[transition].OnInput);

			//Split blocks and cords
			var adjacentTransitions = new AdjacentTransitions(StateCount, transitions, t => t.To);
			var blockSet = 1;
			for(var cordSet = 0; cordSet < cords.SetCount; cordSet++)
			{
				foreach(var transition in cords.Set(cordSet))
					blocks.Mark(transitions[transition].From);

				blocks.SplitSets();

				for(; blockSet < blocks.SetCount; blockSet++)
				{
					foreach(var state in blocks.Set(blockSet))
						foreach(var transition in adjacentTransitions[state])
							cords.Mark(transition);

					cords.SplitSets();
				}
			}

			// Generate minimized DFA
			var minDFA = new DFA(blocks.SetCount, blocks.SetOf(StartState));

			// Set Final States
			for(var set = 0; set < blocks.SetCount; set++)
				// Sets are either all final or non-final states
				if(finalStates.Contains(blocks.SomeElementOf(set)))
					minDFA.AddFinalState(set);

			// Create transitions
			for(var set = 0; set < cords.SetCount; set++)
			{
				var transition = transitions[cords.SomeElementOf(set)];
				var @from = blocks.SetOf(transition.From);
				var to = blocks.SetOf(transition.To);
				minDFA.AddTransition(@from, transition.OnInput, to);
			}

			return minDFA;
		}

		private void DiscardNotReachable(Partition blocks, List<Transition> transitions, Func<Transition, int> getFrom, Func<Transition, int> getTo)
		{
			var adjacentTransitions = new AdjacentTransitions(StateCount, transitions, getFrom);

			foreach(var state in blocks.Marked(0))
				foreach(var transition in adjacentTransitions[state])
					blocks.Mark(getTo(transitions[transition]));

			blocks.DiscardUnmarked();

			transitions.RemoveAll(transition => blocks.SetOf(getFrom(transition)) == -1);
		}
	}
}
