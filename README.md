# This is my improvement of the ethers-rs address book

This crate is a global database based on kv that can be used to store common addresses and ABIs.

note:

you need root privileges to `/var/lib` in order to use this binary, as it's gonna try to write the database at this location.
Run `sudo chown -R $USER:$USER /var/lib/` in order to fix this
