### Project Title: Learn-blackboard_rustrest
### Description:
We are always interested in demonstrating how Blackboard's REST APIs can be called from many different programming languages. A fairly recent language, Rust is catching attention because as described on Wikipedia "Rust is a general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety—meaning that all references point to valid memory—without a garbage collector." Hence this contribution to the Blackboard Learn developer community.

Note that Cargo, the Rust project & package manager builds a project with a very specific directory format. To comply with Anthology's community contribution standards we put the entire Rust project as created by Cargo under the src directory of the Anthology contribution Hence we have Learn-blackboard_rustrest/src/blackboard_rustrest. The Cargo project is blackboard_rustrest and the Cargo.toml must live in that directory alongside a src folder for the project's Rust code.

### Prerequisites:
* Install Rust and Cargo on your system. See https://www.rust-lang.org/
* Get a bit familiar with Cargo and Rust. 
** https://doc.rust-lang.org/book/ch01-00-getting-started.html
* If you use Visual Studio Code, install the Rust extensions.

### Installation:
* Use Git to clone this project to your local machine.

### Configuration
* Copy the config_template.rs file to config.rs
* cd Learn-blackboard_rustrest/src/blackboard_rustrest/src
* cp config_template.rs config.rs
* vi config.rs  (or however you like to edit files...)


### Usage (Launching the project):
* cd Learn-blackboard_rustrest/src/blackboard_rustrest
* cargo run

### Other Useful Things
* https://www.rust-lang.org/
* https://code.visualstudio.com/docs/languages/rust 


