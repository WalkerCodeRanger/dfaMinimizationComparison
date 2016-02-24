del /F Results\*.dfa
Cpp\Debug\Original.exe < TestData\Sample.dfa > Results\Sample.Original.min.dfa
Cpp\Debug\Modified.exe < TestData\Sample.dfa > Results\Sample.Modified.min.dfa
CSharp\CSharp\bin\Debug\CSharp.exe < TestData\Sample.dfa > Results\Sample.CSharp.min.dfa
set RUST_BACKTRACE=1
Rust\target\debug\dfa_min_comparison_rust.exe < TestData\Sample.dfa > Results\Sample.Rust.min.dfa