namespace DfaMinComparisonCSharp.CSharp
{
	public struct Transition
	{
		public readonly int From;
		public readonly int OnInput;
		public readonly int To;

		public Transition(int @from, int onInput, int to)
		{
			From = @from;
			OnInput = onInput;
			To = to;
		}
	}
}
