# CosmWasm from zero to hero

## On Chain Polls

As our first project, we're going to build and store polls on the chain. Its functionality is as follows:

-   Any user can create a poll.
-   Any user can vote on a poll.
-   Polls can have different options.

For a textual example I will talk through a scenario with some users:

1. User A can create a poll for example:
    - What Cosmos coin is your favorite?
        1. Juno
        1. Osmosis
        1. Cosmos Hub
2. User A decides to vote on their own poll, they vote for `Juno`
3. User B can also vote on the poll, they vote for `Cosmos Hub`
4. After a certain amount of time User A (as the owner of the poll) can close the poll. User A closes the poll
5. User C attempts to vote on a closed poll. They cannot
6. The poll results are now visible for everyone to see on the chain

## CosmWasm Starter Pack

This is a template to build smart contracts in Rust to run inside a
[Cosmos SDK](https://github.com/cosmos/cosmos-sdk) module on all chains that enable it.
To understand the framework better, please read the overview in the
[cosmwasm repo](https://github.com/CosmWasm/cosmwasm/blob/master/README.md),
and dig into the [cosmwasm docs](https://www.cosmwasm.com).
This assumes you understand the theory and just want to get coding.

## Creating a new repo from template

Assuming you have a recent version of rust and cargo (v1.58.1+) installed
(via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and cargo-run-script.
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:


**Latest: 1.0.0-beta6**

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME
````

**Older Version**

Pass version as branch flag:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --branch <version> --name PROJECT_NAME
````

Example:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --branch 0.16 --name PROJECT_NAME
```

You will now have a new folder called `PROJECT_NAME` (I hope you changed that to something else)
containing a simple working contract and build system that you can customize.
