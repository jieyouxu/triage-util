# triage-util

Utility for [rust-lang/rust](https://github.com/rust-lang/rust) triaging.

```
Usage: triage-util <COMMAND>

Commands:
  generate-config  Generate a default configuration file under the same directory as the executable [aliases: cfg]
  hydrate-form     Given a list of PR IDs, fetch their information, and generate a form with some of the information filled out. This information should be provided through the config file. You can specify the path for the template report [aliases: form]
  format-report    Format a PR triage report in markdown using information from a fully filled out form [aliases: report]
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Setup and Workflow

This utility is designed to only *assist* in your
[Pull request triage procedure](https://forge.rust-lang.org/release/triage-procedure.html). It will
fetch some basic metadata for each PR you are interested in, and hydrate a form which allows you
to manually input other required information. The util can then be used to generate a markdown
report from the fully filled out form.

To perform a PR triage with this tool, you'll need to perform the following steps:

1. Generate a config file with `triage-util generate-config`. A default config file will be created
   for you under the same directory as the executable.
2. Acquire a
   [GitHub personal access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
   with `public_repo` scope, and set the value of `github_personal_access_token` inside
   `config.toml`.
3. Determine a set of pull requests (and specify their numbers in `config.toml`'s `pull_requests`
   field) you wish to perform triage on, presumably using the
   [Triagebot Dashboard](https://triage.rust-lang.org/triage/rust-lang/rust), or via one of the
   following links:
    - S-waiting-on-review: <https://github.com/rust-lang/rust/pulls?q=is%3Aopen+draft%3Afalse+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-review+-label%3AI-nominated+-label%3Aneeds-fcp>
    - S-waiting-on-author: <https://github.com/rust-lang/rust/pulls?q=is%3Aopen+draft%3Afalse+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-author+-label%3AI-nominated+-label%3Aneeds-fcp>
    - S-blocked: <https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-blocked+sort%3Aupdated-asc>
4. Run `triage-util hydrate-form <output_form_path>`, specifying the desired output path for
   the partially filled form.
5. Fill in the missing mandatory information in the partially filled form.
6. Run `triage-util format-report <form_path> <output_report_path>`, specifying the path to the
   aforementioned input form, and the desired output path for the Markdown report file, to
   generate the Markdown-formatted report.

Steps (1) and (2) are only required for first-time setups.

## Quirks

Due to toml-rs's strange Date handling for (de-)serialization, you'll need to double-quote the
dates for `last_activity_author` and `last_activity_assignee`.

## Examples

### Example form (filled out)

```toml
# Valid status_label:
# - `S-waiting-for-author`
# - `S-waiting-for-review`
# - `S-blocked`
#
# Valid most_recent_activity:
# - `MergeConflicts`
# - `ReviewerCommented`
# - `AuthorCommittedOrCommented`
# - or any other strings

[pull_requests."#99790"]
author = "bors"
assignees = ["ferrisClueless"]
status_label = "S-waiting-on-author"
last_activity_author = "1234-01-01"
last_activity_assignee = "5678-01-01"
waiting_on = "T-clueless"
most_recent_activity = "ReviewerCommented"

[pull_requests."#118569"]
author = "ferris"
assignees = ["ferrisCluelesser"]
status_label = "S-waiting-on-review"
last_activity_author = "2024-01-01"
last_activity_assignee = "2025-01-01"
waiting_on = "The heat death of the universe"
most_recent_activity = "MergeConflicts"
```

### Example report

```md
### Pull request triage report (2024-02-18)

| PR number | Author | Assignees        | Status          | Last activity date (author) | Last activity date (assignee) | Waiting on                     | Most recent activity kind |
|-----------|--------|------------------|-----------------|-----------------------------|-------------------------------|--------------------------------|---------------------------|
| #99790    | bors   | ferrisClueless   | WaitingOnAuthor | 1234-01-01                  | 5678-01-01                    | T-clueless                     | Reviewer commented        |
| #118569   | ferris | ferrisCluelesser | WaitingOnReview | 2024-01-01                  | 2025-01-01                    | The heat death of the universe | Merge conflicts           |
```
