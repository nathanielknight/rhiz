# Rhizome

A deliberately minimal cross-platform task runner.

This is **VERY** much an in-progress-pre-alpha project.

# Commands

- `log`: Print a message.
- `exec`: Run an external command.
- `empty-dir`: Create an empty directory, or delete an existing directory's contents.
- `delete`: Delete a file (if it exists).
- `copy`: Copy a file; wont' overwrite an existing file.
- `rec-copy`: Recursively copy one directory's contents into another.


# TODO

- Document operation and functions
- Record motivation
- Set up CI


# Caveats

- Doesn't do any shell expansion
- Doens't play well with symlinks
- Only intended to operate on files and directories
