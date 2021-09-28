# Agent registration zome

Provides automatic registration of agents entering a network, and query capability to determine who is registered.

## Usage

> Todo

## Development setup

Using the included Nix environment should be sufficient, otherwise you can attempt installing the Holochain tooling yourself.

Simply run `nix-shell` from within this directory to load all necessary utilities. Development environments (`sublime-text`, `code` etc) should be executed from within this context, rather than being launched separately.

## To do

- Tests
- Define query API as a zome trait
- Handle agent deletion when removed from the network
- Update indexing to use an efficient storage structure for large networks

## License

Licensed under an Apache 2.0 license.
