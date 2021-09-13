# Rhiz

A deliberately minimal task runner.

This is an in-progress-pre-alpha project.

Rhiz executes tasks defined in a "Rhizfile" which contains task descriptions
with a Lisp-like syntax.

```scheme
(task "hello"
  (log "Rhiz says hello"))

;; Comments start with a semicolon
(task "fizzbuzz"
  "Tasks can have an optional description"  ;; Strings with spaces are double-quoted.
  (exec fizzbuzz.exe)
  (log "The fizz was buzzed"))

(task "clean"
  (delete "./output"))

("task" echo
  ("echo" "Bare words and quoted strings are equivalent")))
```


# Task execution

Tasks are executed **relative to the directory containing the `Rhizfile`**, not
the directory where `rhiz` is invoked. When `rhiz` is invoked.

The commands in a task are executed one after the other, and if a command
returns a non-zero exit code the the Rhiz immediately exits.

# Commands

<dl>
<dt><code>log</code></dt>
<dd>
  <p>Prints a message to the standard output. Takes a single argument, 
  which should be a string.</p>
</dd>

<dt><code>exec</code></dt>
<dd>
  <p>
    Executes an external command (<code>cargo</code>, <code>npm</code>, etc.) in the Rhizfile's directory
    (usually the project root).
  </p>
  <p>
    Takes one or more  argument(s). The arguments are converted to strings; the first
    should be the name of an external program; the remainder should be it's arguments.
    They're executed using
    <a 
    href="https://doc.rust-lang.org/std/process/struct.Command.html">
      <code>std::process:Command</code>
    </a>
    (effectively: <code>Command::new(first_arg).args(rest_of_args)</code>).
  </p>
</dd>

<dt><code>empty-dir</code></dt>
<dd>
  <p>
    Ensure a directory exists and is empty. Takes a single argument, which should be the
    path to a directory (relative to the Rhizfile).
  </p>
  <p>
    If the directory exists, it's contents are deleted. Otherwise, it's created.
  </p>
</dd>

<dt><code>delete</code></dt>
<dd>
  <p>
    Delete a file. It takes a single argument, which should be
    a file name or path (relative to the Rhizfile).
  </p>
  <p>
    If the file indicated by the path exists it's delete with
    <a
      href="https://doc.rust-lang.org/std/fs/fn.remove_file.html">
      <code>fs::remove_file</code>
    </a>
    If the file doesn't exist, this command is ignored.
  </p>
</dt>

<dt><code>copy</code></dt>
<dd>
  <p>
    Copy a file. Takes two arguments: the source and destination paths for the copy.
  </p>
  <p>
    The source should be the path to a file (relative to the Rhizfile).
    The destination can be a path to a directory (in which case the source file
    is copied there with the same name) or to a new file (in which case it's 
    copied with the new name). If the destination file already exists, this
    command exits with an error.
  </p>
  <p>
    The copy is performed using 
    <a 
      href="https://doc.rust-lang.org/std/fs/fn.copy.html">
      <code>fs::copy</code>
    </a>
  </p>
</dd>

<dt><code>rec-copy</code></dt>
<dd>
  <p>
    Recursively copies a directory. Takes to arguments: the source and destination
    paths for the copy.
  </p>
  <p>
    Both the source and target directories should exist. The files and directories in the
    source are copied into the target.
  </p>
</dd>

<dt><code>par</code></dt>
<dd>
  <p>Execute commands in parallel. Takes any number of tasks (written as s-expressions) as arguments.</p>
</dd>


<!-- TODO(nknight): examples of each command type -->
