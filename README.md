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
    last    Document last N commands
    s       Set a marker to save the last command

```


### Example
````sh
$> IMPORTANT_TEXT="This is an important command"
$> rmd s
$> ls /
$> cd ~
$> echo $IMPORTANT_TEXT > important_file.txt
$> rmd s
$> rmd -g last 2
Documenting last 2 saves
```sh
IMPORTANT_TEXT="This is an important command"
echo $IMPORTANT_TEXT > important_file.txt
```
````