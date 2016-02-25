using System;
using System.Collections.Generic;

namespace DfaMinComparisonCSharp.CSharp
{
	public class AdjacentTransitions
	{
		private readonly int[] adjacent; // transition indexes grouped by state they are adjacent to
		private readonly int[] offset; // offsets into adjacent list for a given state

		public AdjacentTransitions(int stateCount, IList<Transition> transitions, Func<Transition, int> getState)
		{
			adjacent = new int[transitions.Count];
			offset = new int[stateCount + 1];

			// Count transitions per state
			foreach(var transition in transitions)
				++offset[getState(transition)];

			// Running addition
			for(var state = 0; state < stateCount; ++state)
			{
				offset[state + 1] += offset[state];
			}

			// Place transitions, and correct offsets
			for(var transition = transitions.Count - 1; transition >= 0; transition--)
				adjacent[--offset[getState(transitions[transition])]] = transition;
		}

		public IEnumerable<int> this[int state]
		{
			get
			{
				for(var i = offset[state]; i < offset[state + 1]; ++i)
					yield return adjacent[i];
			}
		}
	}
}
