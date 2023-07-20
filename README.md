# Polkadot-Rust-Bootcamp-Week-2
TASK: Create a simple calculator using enums and pattern matching

Task Details:

In this task, a simple calculator that can perform basic arithmetic operations using enums and pattern matching is created in Rust.

Steps:

Create an enum called Operation with variants Add, Subtract, Multiply, and Divide. Each variant should hold two f64 values.

Create a function called calculate that takes an Operation enum as an argument and returns an f64 result.

Implement the calculate function using pattern matching to perform the appropriate arithmetic operation based on the variant of the Operation enum.

In the main function, prompt the user to input the first number, the operation to be performed, and the second number.

Parse the user input into appropriate variables.

Create an Operation enum instance with the parsed input values.

Call the calculate function with the created Operation enum instance.

Print the result to the console.

Compile and run the program to ensure it works as expected.

