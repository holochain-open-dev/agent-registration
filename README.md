# Agent registration zome

Provides automatic registration of agents entering a network, and query capability to determine who is registered.

## Usage

> Todo

## Development setup

Using Holonix setup should work fine.

To build the extended DNAs, you will need to clone the [social triangulation zome](https://github.com/holochain-open-dev/social-triangulation/) in a folder adjacent to this one, with the name `happ-social-triangulation`. This is a temporary measure until a proper build & deployment system for Holochain zomes is available.

## To do

- Tests
- Define query API as a zome trait
- Handle agent deletion when removed from the network
- Update indexing to use an efficient storage structure for large networks

## License

Licensed under an Apache 2.0 license.
