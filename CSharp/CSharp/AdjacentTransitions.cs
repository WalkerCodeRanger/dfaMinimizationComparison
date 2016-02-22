using System;
using System.Collections.Generic;

namespace DfaMinComparisonCSharp.CSharp
{
	public class AdjacentTransitions
	{
		private readonly int[] adjacent;
		private readonly int[] offset;

		public AdjacentTransitions(int stateCount, IList<Transition> transitions, Func<Transition, int> getState)
		{
			adjacent = new int[transitions.Count];
			offset = new int[stateCount + 1];

			// Count transitions per state
			foreach(var transition in transitions)
				++offset[getState(transition)];

			// Running addition
			for(var state = 0; state < stateCount; ++state)
				offset[state + 1] += offset[state];

			// Place transitions, and correct offsets
			for(var t = transitions.Count - 1; t >= 0; t--)
				adjacent[--offset[getState(transitions[t])]] = t;
		}

		public IEnumerable<int> this[int state]
		{
			get
			{
				for(var j = offset[state]; j < offset[state + 1]; ++j)
					yield return adjacent[j];
			}
		}
	}
}
