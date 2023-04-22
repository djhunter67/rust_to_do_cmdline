# This is a command line TO DO application

## This application takes advantage of the Rust programming language features

## This application uses Enums, Structs, and Traits

## Usage

```bash
cargo run [COMMAND] "TO DO"
```

## Commands

```text
- edit
- delayed
- abandoned
- completed
- delete
```

## Installation

```bash
git clone git@github.com:djhunter67/rust_to_do_cmdline.git
cargo build --release
cd rust_to_do_cmdline
```

## Development

```bash
git clone git@github.com:djhunter67/rust_to_do_cmdline.git
cd rust_to_do_cmdline
code .
```

``` bash
      1 .
      2 ├── src
      3 │   ├── to_do
      4 │   │   ├── structs
      5 │   │   │   ├── abandoned.rs
      6 │   │   │   ├── base.rs
      7 │   │   │   ├── completed.rs
      8 │   │   │   ├── delayed.rs
      9 │   │   │   ├── mod.rs
     10 │   │   │   ├── pending.rs
     11 │   │   │   └── started.rs
     12 │   │   ├── traits
     13 │   │   │   ├── create.rs
     14 │   │   │   ├── delete.rs
     15 │   │   │   ├── edit.rs
     16 │   │   │   ├── get.rs
     17 │   │   │   └── mod.rs
     18 │   │   ├── enums.rs
     19 │   │   └── mod.rs
     20 │   ├── args.rs
     21 │   ├── main.rs
     22 │   └── processes.rs
     23 ├── Cargo.lock
     24 ├── Cargo.toml
     25 ├── CHANGELOG.md
     26 ├── .gitignore
     27 ├── README.md
     28 └── state.json
     29
     30 5 directories, 23 files
```
