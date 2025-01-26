# shell_interface
![](shell_interface_screenshot.jpg?raw=true)  
This project allows you to quickly implement a GUI that performs operations on a shell.

It is intended to run scripts that do not need input, however if you run the compiled program in a shell, it is possible for the user to use operations that require input (sudo, ...) .

# Usage
1. Customize "src/main.rs" to your needs
2. Compile the project with cargo
```bash
cargo build --release
```
3. You will find the compiled program under "target/release/shell_interface" 

# Documentation
Run
```bash
cargo doc
```
You will find the documentation under "target/doc" .
