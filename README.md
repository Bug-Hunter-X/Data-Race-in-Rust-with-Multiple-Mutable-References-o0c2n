# Data Race in Rust with Multiple Mutable References

This repository demonstrates a potential data race in Rust that can occur when multiple mutable references point to the same variable.  The program attempts to modify the value through two mutable references concurrently, leading to undefined behavior. 

The `bug.rs` file contains the buggy code, while `bugSolution.rs` provides a corrected version demonstrating how to avoid the data race.