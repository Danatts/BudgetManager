# BUDGET MANAGER

Budget manager CLI application written in Rust🦀.

## BASIC USAGE

Run `budget help` to check all available commands.

```
$ budget help

Usage: budget [OPTIONS] <COMMAND>

Commands:
  current   Set current budget funds
  delete    Delete a budget
  history   Print transaction history
  initial   Set initial budget funds
  increase  Increase budget funds
  list      List all budgets
  new       Create a new budget
  reduce    Reduce budget funds
  rename    Rename a budget
  reset     Reset a budget to initial funds
  help      Print this message or the help of the given subcommand(s)

Options:
  -d, --database <FILE NAME>  Select a database file
  -h, --help                  Print help
  -V, --version               Print version
```

Also you can run `budget <COMMAND> help` to check each command syntax.

```
$ budget increase help

Increase budget funds

Usage: budget increase [OPTIONS] <ID> <AMOUNT>

Arguments:
  <ID>      
  <AMOUNT>  

Options:
  -d, --description <DESCRIPTION>  Add small description
  -h, --help                       Print help
  -V, --version                    Print version
```

## TODO

- [ ] Remove and edit transactions
- [ ] Handle more than one budget
- [ ] Display warning when user is getting close to going over budget
