# This is my improvement of the ethers-rs address book

This crate is a global database that can be used to store common addresses and ABIs for each chain.

db.json structure:

<<<<<<< HEAD
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
=======
you need root privileges to `/var/lib` in order to use this binary, as it's gonna try to write the database at this location.
Run `sudo chown -R $USER:$USER /var/lib/` in order to fix this

`alias cbook = /../target/debug/contract-book`

`cbook --help`
>>>>>>> 1ff89e5a4d33324186e95bed921fc6478d5bc354
