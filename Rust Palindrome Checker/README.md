# Palindrome Checker in Rust

This is a simple Rust program that checks if a given word or phrase is a palindrome.

# How to use

Clone the repository to your local machine.

Navigate to the project directory in your terminal.

Run 'cargo run' to start the program.

Enter a word or phrase when prompted.

The program will output whether the input is a palindrome or not.

# How it works

The program takes a string input from the user and checks whether it is a palindrome. It first converts the string to lowercase and removes any non-alphanumeric characters. It then compares the resulting string to its reversed version. If the two are equal, the string is a palindrome and the program outputs that the input is a palindrome. If they are not equal, the program outputs that the input is not a palindrome.

```python

Enter a word or phrase:
racecar
'racecar' is a palindrome

# Configuration Steps

Create a Virtual Environment
The purpose of virtual environments is to create a self-contained environment for each of your projects, allowing you to manage dependencies, libraries, and versions separately for each project.

`python3 -m venv rustenv`

`source rustenv/bin/activate`

`cd rustenv`

Install Rust Go to https://rustup.rs/ and run the command curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Run source "$HOME/.cargo/env" to configure your current shell.

create new project The cargo tool is the default package manager for Rust and provides an easy way to manage dependencies and build projects.

Run cargo new (project name) (my Eg: cargo new hello)

This will create a binary (application) microservice package

Create main.rs and lib.rs files in the src project

touch main.rs and touch lib.rs

Run Cargo build
This is a command in the Rust programming language that is used to compile a Rust project. It compiles the project's source code and its dependencies, and produces an executable binary file. The cargo build command can be run from the root directory of the project.

Set up Cargo.toml to determine the dependencies and build configuration of the project.
