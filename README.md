# Rhiz

A deliberately minimal task runner.

This is an in-progress-pre-alpha project.

Rhiz executes tasks defined in a "Rhizfile", containing task descriptions
with a Lisp like syntax.

```scheme
(task "hello"
  (log "Rhiz says hello"))

;; Comments start with a semicolon
(task "fizzbuzz"
  "Tasks can have an optional description"
  (exec fizzbuzz.exe))

(task "clean"
  (delete "./output"))
```

# Commands

- `log`: Print a message.
- `exec`: Run an external command.
- `empty-dir`: Create an empty directory, or delete an existing directory's contents.
- `delete`: Delete a file (if it exists).
- `copy`: Copy a file; wont' overwrite an existing file.
- `rec-copy`: Recursively copy one directory's contents into another.
- `par`: Perform a set of commands in parallel.


# TODO (Code)

- Set up CI?
- Migrate to GitHub?

# TODO (Documentation)

- Overview (motivation, high-level operation)
- Reference information on each function (e.g. "do copy and rec-copy follow symlinks")
- Example Rhizfile(s) (with equivalent script/makefile?)
