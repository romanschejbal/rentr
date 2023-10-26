## Program Description

This program is a utility for running arbitrary commands when files change. It is written in Rust and is similar to the `entr` tool.

The program takes a list of paths to watch for changes from stdin. When any of the watched files are modified, the program executes a provided command.

The program supports the following options:

- `-c` or `--clear`: Clears the screen before running the command.
- `-r` or `--reset`: Reloads a persistent child process.

The program uses the `notify` crate to watch for file changes and the `structopt` crate for command line argument parsing.

## Usage

To use the program, you can pipe a list of paths to watch from stdin and provide a command to execute when the files change. For example:

```
echo "my-folder" | rentr -c -r echo "my-folder has changed"
```

This will watch the `my-folder` directory for changes and execute the command `echo "my-folder has changed"` whenever a change occurs.

`find . | grep \.rs | rentr -c cargo build`
`find . | grep \.rs | rentr -c echo "CHANGED"