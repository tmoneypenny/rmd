# rmd

Document what works. A simple CLI tool to document past commands as markdown by specifying "markers" as you go.

### Usage
```
rmd 0.1.0
Todd Moneypenny
Document what works

USAGE:
    main [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -g, --group      Group commands together
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    last    Document last N markers (Default: 5)
    s       Set a marker to save the last N commands

```

### Example
````sh
$> IMPORTANT_TEXT="This is an important command"
$> rmd s
$> ls /
$> cd ~
$> touch important_file.txt
$> echo "Start of my important file!!" > important_file.txt
$> echo $IMPORTANT_TEXT >> important_file.txt
$> rmd s 3
$> rmd -g last 2
```sh
IMPORTANT_TEXT="This is an important command"
touch important_file.txt
echo "Start of my important file!!" > important_file.txt
echo $IMPORTANT_TEXT > important_file.txt
```
````