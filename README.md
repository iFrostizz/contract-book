# This is my improvement of the ethers-rs address book

This crate is a global database that can be used to store common addresses and ABIs for each chain.

## Download

```console
foo@bar:~$ cargo install --git https://github.com/iFrostizz/contract-book.git
foo@bar:~$ cbook --help
```

## Usage

```console
foo@bar:~$ cbook --name WETH9 --address 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2 --abi '[{"constant":true,"inputs":[], ...}]' --chain mainnet
```

I wrote a foundry library to retrieve data in tests: https://github.com/iFrostizz/sbook

## Info

db.json structure:

```json
{
  "name": {
    "abi": [
	{
	    "constant": true,
	    "inputs": []
	}
    ],
    "address": {
      "chain": "0x..."
    }
  }
}
```

## Code style

`cargo clippy`
`cargo +nightl fmt`

## TODO

- [x] Finish the contract-book implementation
- [ ] Write a foundry library that uses `ffi` (in progress)
- [ ] Provide an API to find elements easily (in progress see `get`)
- [ ] Let store local instances of cbook for solidity testing 
- [ ] Allow database migrations
- [ ] Write tests (or not ;))
