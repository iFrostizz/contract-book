# This is my improvement of the ethers-rs address book

This crate is a global database that can be used to store common addresses and ABIs for each chain.

db.json structure:

{
  name: {
    abi: Abi,
    address: {
      chain: Address
    }
  }
}

# TODO

- [ ] Finish the contract-book implementation
- [ ] Provide an API to find elements easily
- [ ] Allow database migrations
- [ ] Write tests (or not ;))
