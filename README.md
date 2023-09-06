# deploy-smart-contract

## Description
Example on how complile, deploy and interact with smart contract using Rust.

## Prerequisites
Make sure you have installed ganache-cli

## Usage
Execute
``` 
ganache-cli
```
Select any of the printed private keys and export it.
For example:
```
export PRIVATE_KEY=0x646f1ce2fdad0e6deeeb5c7e8e5543bdde65e86029e2fd9fc169899c440a7913
```
then execute:
```
cargo run
```
The following output should printed
```
Init msg: Hello World
Updated msg: Updated hello world!
```





