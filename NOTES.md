# triage-util Notes

## PR Triage

It would be nice to have some CLI tooling to help with the PR triage procedure.
According to <https://forge.rust-lang.org/release/triage-procedure.html>, we
mostly deal with three kinds of status labels: `S-waiting-on-review`,
`S-waiting-on-author` and `S-blocked`.

### `S-waiting-on-review`

Link:
<https://github.com/rust-lang/rust/pulls?q=is%3Aopen+draft%3Afalse+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-review+-label%3AI-nominated+-label%3Aneeds-fcp>

- Only triage PRs that were last updated 15 days or more ago (give or take a
  day).
- For each PR:
    - If the PR has new conflicts, CI failed, or a new review has been made
      then:
        - Change the label to `S-waiting-on-author`.
        - Ping the author.
    - Add PR to triage to report.

### `S-waiting-on-author`

Link:
<https://github.com/rust-lang/rust/pulls?q=is%3Aopen+draft%3Afalse+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-author+-label%3AI-nominated+-label%3Aneeds-fcp>

- Only triage PRs that were last updated 15 days or more ago (give or take a
  day).
- For each PR:
    - If the author did what the PR was waiting on them for then update the
      label to `S-waiting-on-review`.
    - Otherwise, if the author still needs to do something, then ping the
      author if they are **not** a member of a Rust team (does not include
      working groups — only teams like `T-compiler`, `T-lang`, `T-rustdoc`,
      etc.).
    - Add PR triage to report.

### `S-blocked`

Link: <https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-blocked+sort%3Aupdated-asc>

- For each PR:
    - If it is still blocked then leave it as-is.
    - Otherwise, if it is no longer blocked, then remove `S-blocked` (and add a
      status label like `S-waiting-on-review` if appropriate).
    - Add PR triage to report.

## Issue Triage

A contributor might not have permission to directly assign labels to an issue
via the github UI. It can be helpful to have some kind of GUI/TUI for label
management -- that is, the user can manage labels via the GUI/TUI and then
the corresponding `rustbot` command could be generated, e.g.

```
@rustbot label +T-compiler +C-bug +A-linkage +O-macos -needs-triage
```

It would be especially nice to validate that these labels exist before posting
a comment, that the `rustbot` command is at least syntactically valid.

For valid labels, see e.g.
<https://github.com/rust-lang/rust/blob/a84bb95a1f65bfe25038f188763a18e096a86ab2/triagebot.toml#L4>.
