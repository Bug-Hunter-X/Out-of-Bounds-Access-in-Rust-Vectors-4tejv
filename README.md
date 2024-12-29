# Out of Bounds Access in Rust Vectors

This repository demonstrates a common error in Rust: attempting to access an element in a vector using an index that is out of bounds.  This leads to a runtime panic, halting the program's execution.

The `bug.rs` file contains the erroneous code.  The `bugSolution.rs` file provides a corrected version, illustrating how to safely access vector elements and handle potential out-of-bounds errors.

This example highlights the importance of careful index management when working with vectors in Rust to prevent unexpected runtime crashes.