del /F Results\*.dfa
set RUST_BACKTRACE=1

ECHO Running for Sample.dfa
Cpp\Debug\Original.exe < TestData\Sample.dfa > Results\Sample.Original.min.dfa
Cpp\Debug\Modified.exe < TestData\Sample.dfa > Results\Sample.Modified.min.dfa
CSharp\CSharp\bin\Debug\CSharp.exe < TestData\Sample.dfa > Results\Sample.CSharp.min.dfa
Rust\target\debug\dfa_min_comparison_rust.exe < TestData\Sample.dfa > Results\Sample.Rust.min.dfa

ECHO Running for Lex.dfa
Cpp\Debug\Original.exe < TestData\Lex.dfa > Results\Lex.Original.min.dfa
Cpp\Debug\Modified.exe < TestData\Lex.dfa > Results\Lex.Modified.min.dfa
CSharp\CSharp\bin\Debug\CSharp.exe < TestData\Lex.dfa > Results\Lex.CSharp.min.dfa
Rust\target\debug\dfa_min_comparison_rust.exe < TestData\Lex.dfa > Results\Lex.Rust.min.dfa