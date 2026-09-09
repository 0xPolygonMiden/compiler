---
name: commit-hygiene
description: Commit message format and pull request commit rules for this repository. Use when committing, amending, squashing, or drafting or reviewing commit messages so the history matches the project's conventions.
---

# Commit Hygiene

## Message format

Commit messages should be written in a short, descriptive manner and adhere to the following format:

```text
<scope>: <summary>

<description>
```

Where:

- `<scope>` indicates the scope or component of the compiler affected by the change. This is sometimes a specific crate, e.g. `midenc`, but more often than not a change may modify multiple crates, in which case the scope should indicate what the cross-cutting concern is. For example, `analysis` would be a good scope for a change that implements a new dataflow analysis pass; `arith` would be a good scope for a change that is focused on the `arith` dialect; `build` or `ci` are commonly used for changes that modify build tooling or CI workflows. Don't overthink it - just try to convey to the reader what the scope of the change is, if they were to read the commit without any additional context. Look at existing commits for reference if you are unsure.
- `<summary>` should be a terse description of the change, no more than 80 characters if possible. You may omit the `<scope>: ` prefix if `<summary>` would exceed 80 characters otherwise, and already adequately conveys the scoping of the commit.
- `<description>` should be a more detailed description of what changed, primarily focusing on the _why_. The reader can look at the diff to see the _what_, so your goal is only to summarize that aspect briefly. What the reader can't tell from the diff is the context on _why_ the change was made - focus on that.

An example of a good commit message is the following:

```
ci: benchmark each revision with its own SDK and examples

The baseline compiler was driven over candidate sources, so SDK changes such as
allocator fixes were included on both sides and their size differences
disappeared. Use the baseline checkout for its examples, SDK, and inputs while
retaining a common benchmark runner and VM executor. Read result revisions from
their source checkouts and fail on either side’s build errors instead of
publishing a partial baseline.
```

The PRs should adhere to the following rules:

- Each commit is logically distinct from the others
- Each commit is valid on its own (i.e. tests pass and lints are clean)

