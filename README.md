# This is my improvement of the ethers-rs address book

This crate is a global database that can be used to store common addresses and ABIs for each chain.

## Download

`$ git clone https://github.com/iFrostizz/contract-book.git`
`$ cargo build --release`
add ~/.../contract-book/target/release/cbook to your `$PATH`
`cbook --help`

## Usage

`cbook --name WETH9 --address 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2 --abi '[{"constant":true,"inputs":[], ...}]' --chain mainnet`

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

## TODO

- [x] Finish the contract-book implementation
- [ ] Provide an API to find elements easily
- [ ] Allow database migrations
- [ ] Write tests (or not ;))
