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
			for(var transition = transitions.Count - 1; transition >= 0; transition--)
				adjacent[--offset[getState(transitions[transition])]] = transition;
		}

		public IEnumerable<int> this[int state]
		{
			get
			{
				for(var transition = offset[state]; transition < offset[state + 1]; ++transition)
					yield return adjacent[transition];
			}
		}
	}
}
