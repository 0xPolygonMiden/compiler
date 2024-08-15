# Miden Debugger

This crate implements a TUI-based interactive debugger for the Miden VM,  designed  to
interoperate with `midenc`.

# Usage

The easiest way to use the debugger, is via `midenc run`, and giving it a path  to a
program compiled by `midenc compile`.

Once started, you will be dropped into the main debugger UI, stopped at the first cycle of
the program. The UI is organized into pages and panes, with the main/home page being the
one you get dropped into when the debugger starts. The home page contains the following panes:

* Source Code - displays source code for the current instruction, if available, with
  the relevant line and span highlighted, with syntax highlighting (when available)
* Disassembly - displays the 5 most recently executed VM instructions, and the current
  cycle count
* Stack Trace - displays a stack trace for the current instruction, if the program was
  compiled with tracing enabled. If frames are unavailable, this pane may be empty.
* Operand Stack - displays the contents of the operand stack and its current depth
* Breakpoints - displays the set of current breakpoints, along with how many were hit
  at the current instruction, when relevant

On the home page, the following keyboard shortcuts are available:

* `q` (quit) - exit the debugger
* `h`,`l` (pane movement) - cycle focus to the next pane (`h`) or previous pane (`l`)
* `s` (step) - advance the VM one cycle
* `n` (step next) - advance the VM to the next instruction (i.e. skip over all the cycles
  of a multi-cycle instructions)
* `c` (continue) - advance the VM to the next breakpoint, or until execution terminates
* `d` (delete) - delete an item (where applicable, for example, the breakpoints pane)
* `:` (command prompt) - bring up the command prompt (described further below)

When various panes have focus, additional keyboard shortcuts are available, in any pane
with a list of items, or multiple lines (e.g. source code), `j` and `k` (or the up and
down arrows) will select the next item up and down, respectively. As more features are
added, I will document their keyboard shortcuts below.

## Commands

From the home page, typing `:` will bring up the command prompt in the footer pane.

You will know the prompt is active because the keyboard shortcuts normally shown there will
no longer appear, and instead you will see the prompt, starting with `:`. It supports any
of the following commands:

* `q` or `quit` (quit) - exit the debugger
* `history` (command history) - display prompt history
* `reload` (reload current program) - reloads the program from disk, and resets the UI, with the
  exception of breakpoints, which are retained across reloads
* `b` or `break` or `breakpoint` (breakpoints) - manage breakpoints (see [Breakpoints](#breakpoints))
* `r` or `read` (read memory) - read values from linear memory (see [Reading Memory](#read-memory))

## Breakpoints

One of the most common things you will want to do with the debugger is set and manage breakpoints.
Using the command prompt, you can create breakpoints by typing `b` (or `break` or `breakpoint`),
followed by a space, and then the desired breakpoint expression to do any of the following:

* Break at an instruction which corresponds to a source file (or file and line) whose name/path
  matches a pattern
* Break at the first instruction which causes a call frame to be pushed for a procedure whose name
  matches a pattern
* Break any time a specific opcode is executed
* Break at the next instruction
* Break after N cycles
* Break at CYCLE

The syntax for each of these can be found below, in the same order (shown using `b` as the command):

* `b FILE[:LINE]` - where `FILE` is a glob pattern matched against the source file path. The `:LINE`
  part is optional, as indicated by the brackets. If specified, only instructions with source
  locations in `FILE` _and_ that occur on `LINE`, will cause a hit.
* `b in NAME` - where `NAME` is a glob pattern matched against the fully-qualified procedure name
* `b for OPCODE` - where `OPCODE` is the exact opcode you want to break on (including immediates)
* `b next`
* `b after N`
* `b at CYCLE` - if `CYCLE` is in the past, this breakpoint will have no effect

When a breakpoint is hit, it will be highlighted, and the breakpoint window will display the number
of hit breakpoints in the lower right.

After a breakpoint is hit, it expires if it is one of the following types:

* Break after N
* Break at CYCLE
* Break next

When a breakpoint expires, it is removed from the breakpoint list on the next cycle.

## Read Memory

Another useful diagnostic task is examining the contents of linear memory, to verify that expected
data has been written. You can do this via the command prompt, using `r` (or `read`), followed by
a space, and then the desired memory address and options:

The format for read expressions is `:r ADDR [OPTIONS..]`, where `ADDR` is a memory address in
decimal or hexadecimal format (the latter requires the `0x` prefix). The `read` command supports
the following for `OPTIONS`:

* `-m MODE` or `-mode MODE`, specify a memory addressing mode, either `words` or `bytes` (aliases
  `w`/`b`, `word`/`byte`, or `miden`/`rust` are permitted). This determines whether `ADDR` is an
  address in units of words or bytes. (default `words`)
* `-f FORMAT` or `-format FORMAT`, specify the format used to print integral values
  (default `decimal`):
  - `d`, `decimal`: print as decimal/base-10
  - `x`, `hex`, `hexadecimal`: print as hexadecimal/base-16
  - `b`, `bin`, `binary`, `bits`: print as binary/base-2
* `-c N` or `-count N`, specify the number of units to read (default `1`)
* `-t TYPE` or `-type TYPE`, specify the type of value to read. In addition to modifying the default
  for `-format`, and the unit size for `-count`, this will also attempt to interpret the memory as
  a value of the specified type, and notify you if the value is invalid. The default type is `word`.
  Available types are listed below:
  - `iN` and `uN`: integer of `N` bits, with the `i` or `u` prefix determining its signedness.
    `N` must be a power of two.
  - `felt`: a field element
  - `word`: a word, i.e. an array of four `felt`
  - `ptr` or `pointer`: a 32-bit memory address (defaults `-format hex`)
  - In the future, more types will be supported, namely structs/arrays

Any invalid combination of options, or invalid syntax, will display an error in the status bar.

# Roadmap

The following are some features planned for the near future:

* Watchpoints, i.e. cause execution to break when a memory store touches a specific address
* Conditional breakpoints, i.e. only trigger a breakpoint when an expression attached to it
  evaluates to true
* More robust type support in the `read` command
* Display procedure locals and their contents in a dedicated pane
