# Crab-Finder🦀
- The BEST way (probably) to find does files in your completely not chaotic file system. 
## How to start?
- First you need to install your rust compiler and run this command on your terminal:
```
cargo build
```
- Then go to the 'target' folder and search for your executable
## How to use?
- Go to your beautifull terminal and run the command:
```
find.out -c "name_file" -p "/home"
```
- And they will start looking for it.
- Here you have all the info you need to know for the moment:
```
Finds the name of a file in your device

Usage: find.exe [OPTIONS]

Options:
  -c, --content <CONTENT>  Sequence of characters to find [default: ]
  -r, --regex <REGEX>      Pattern to find [default: ]
  -p, --paths <PATHS>      Root Paths
  -t, --threads <THREADS>  threads to use [default: 1]
  -h, --help               Print help
  -V, --version            Print version
  ```