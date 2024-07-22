### Project Title: Learn-blackboard_rustrest
### Description:
We are always interested in demonstrating how Blackboard's REST APIs can be called from many different programming languages. A fairly recent language, Rust is catching attention because as described on Wikipedia "Rust is a general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety—meaning that all references point to valid memory—without a garbage collector." Hence this contribution to the Blackboard Learn developer community.

Note that Cargo, the Rust project & package manager builds a project with a very specific directory format. To comply with Anthology's community contribution standards we put the entire Rust project as created by Cargo under the src directory of the Anthology contribution Hence we have Learn-blackboard_rustrest/src/blackboard_rustrest. The Cargo project is blackboard_rustrest and the Cargo.toml must live in that directory alongside a src folder for the project's Rust code.

This paragraph is a high-level description of what's in the code. The main.rs is file is required by all Rust applications. It imports config.rs containing the Config structure that we use to hold the REST app key and secret, the hostname of the Learn server we connect to, the 2LO token endpoint and the Users endpoint. The main function makes a call to get an access token, uses that token to call the users endpoint, then iterates over the users returned to display their id (pk1) and userName. 

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


