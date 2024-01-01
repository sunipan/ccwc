# ccwc
A clone of the wc CLI tool made with Rust that supports both files and input

## Setup for command line
1. Make sure you have Rust installed, if not, click [here](https://rustup.rs/)
2. Clone the repository
3. Run `cargo build --release`
4. A binary file called `ccwc` will be created in your `target/release` directory relative of where you clone the repository.
5. Add the path of this binary file to your PATH
6. You can now use this tool inside your command line!
   
## Flags
- `-c` prints out the number of bytes
- `-l` prints out the number of lines
- `-w` prints out the number of words
- `-m` prints out the number of characters

## Usage
- Can be used with command line input:
```
$ cat test.txt | ccwc -clmw
Bytes: 342190
Lines: 7145
Words: 58164
Characters: 339292
```
- Can be used with file names:
```
ccwc -clmw test.txt
Bytes: 342190
Lines: 7145
Words: 58164
Characters: 339292
```
- Can be used with user entered input (press Ctrl+D, sometimes twice, to get results)
```
$ ccwc -clmw
Hello world!
Bytes: 13
Lines: 1
Words: 2
Characters: 13
```
- Can also be used without flags to get the bytes, lines, and words of the file or input:
```
$ ccwc test.txt
Bytes: 342190
Lines: 7145
Words: 58164
```
