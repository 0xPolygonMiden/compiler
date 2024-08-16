# Miden Debugger

This crate implements a TUI-based interactive debugger for the Miden VM,  designed  to
interoperate with `midenc`.

# Usage

The easiest way to use the debugger, is via `midenc debug`, and giving it a path to a
program compiled by `midenc compile`. See [Program Inputs](#program-inputs) for information
on how to provide inputs to the program you wish to debug. Run `midenc help debug` for more
detailed usage documentation.

The debugger may also be used as a library, but that is left as an exercise for the reader for now.

## Example

```shell
# Compile a program to MAST from a rustc-generated Wasm module
midenc compile foo.wasm -o foo.masl

# Load that program into the debugger and start executing it
midenc debug foo.masl
```

## Program Inputs

To pass arguments to the program on the operand stack, or via the advice provider, you have two
options, depending on the needs of the program:

1. Pass arguments to `midenc debug` in the same order you wish them to appear on the stack. That
   is, the first argument you specify will be on top of the stack, and so on.
2. Specify a configuration file from which to load inputs for the program, via the `--inputs` option.

### Via Command Line

To specify the contents of the operand stack, you can do so following the raw arguments separator `--`.
Each operand must be a valid field element value, in either decimal or hexadecimal format. For example:

```shell
midenc debug foo.masl -- 1 2 0xdeadbeef
```

If you pass arguments via the command line in conjunction with `--inputs`, then the command line arguments
will be used instead of the contents of the `inputs.stack` option (if set). This lets you specify a baseline
set of inputs, and then try out different arguments using the command line.

### Via Inputs Config

While simply passing operands to the `midenc debug` command is useful, it only allows you to specify
inputs to be passed via operand stack. To provide inputs via the advice provider, you will need to use
the `--inputs` option. The configuration file expected by `--inputs` also lets you tweak the execution
options for the VM, such as the maximum and expected cycle counts.

An example configuration file looks like so:

```toml
# This section is used for execution options
[options]
max_cycles = 5000
expected_cycles = 4000

# This section is the root table for all inputs
[inputs]
# Specify elements to place on the operand stack, leftmost element will be on top of the stack
stack = [1, 2, 0xdeadbeef]

# The `inputs.rodata` section is a list of rodata segments that should be placed
# in the advice map before the program is executed. Programs compiled by midenc
# will have a prologue generated in their entrypoint that writes this data to linear
# memory, by moving it from the advice map to the advice stack (using the commitment
# digest), and then invoking `std::mem::pipe_preimage_to_memory`.
#
# The raw binary data is chunked up into 4 byte chunks, and then converted to field
# elements by first treating each chunk as a big-endian u32 value, and then creating
# the field element from that value. The data will arrive on the advice stack in an
# order that ensures it is written to linear memory in the same order as it appears
# in the raw binary data.
#
# You can specify one or more of these segments
[[inputs.rodata]]
digest = '0xb9691da1d9b4b364aca0a0990e9f04c446a2faa622c8dd0d8831527dbec61393'
# Specify a path to the binary data for this segment
path = 'foo.bin'
# Or, alternatively, specify the binary data in hexadecimal form directly
# data = '0x...'

# This section contains input options for the advice provider
[inputs.advice]
# Specify elements to place on the advice stack, leftmost element will be on top
stack = [1, 2, 3, 4]

# The `inputs.advice.map` section is a list of advice map entries that should be
# placed in the advice map before the program is executed. Entries with duplicate
# keys are handled on a last-write-wins basis.
[[inputs.advice.map]]
# The key for this entry in the advice map
digest = '0x3cff5b58a573dc9d25fd3c57130cc57e5b1b381dc58b5ae3594b390c59835e63'
# The values to be stored under this key
values = [1, 2, 3, 4]

[[inputs.advice.map]]
digest = '0x20234ee941e53a15886e733cc8e041198c6e90d2a16ea18ce1030e8c3596dd38''
values = [5, 6, 7, 8]
```

# Debugger Usage

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
* `debug` (debug log) - display internal debug log for the debugger itself
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
