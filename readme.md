What is stash?
=====
Stash is an application to quickly stash and retrieve files locally from the command line



Why Should I use stash?
=====

- If there is a file you want to just keep a hold of but dont know where to move it to yet

- If there is a file you might want in many places, a config file

- If there is a notes file you want to have access from anywhere


Commands
======
stash send filename : send

stash ret filename : retrieve

stash del filename : delete 

stash lst : list

stash cat filename : cat



Planned Updates
=====
- support for moving multiple files at a time
- Betters Errors and Printing
- partial file name matching


Building
====

Stash is built entirely in Rust, to compile you must have Rust <a href="https://www.rust-lang.org/en-US/install.html">installed</a>,
then you can simply type 
```cargo build```
to compile the application

For now, running requires you to have a stash folder in your home directory
It will search for a file Stash.toml here, this file is for configuration
of the savepath.
```
save_path = "C:/path/to/my/storage/directory"
```
