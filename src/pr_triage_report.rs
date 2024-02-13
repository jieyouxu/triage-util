use std::collections::BTreeMap;

use time::Date;

/// Triage report to be posted to the weekly `#t-release/triage` Zulip stream. A triage report is
/// a collection of PR triages.
#[derive(Debug)]
pub struct PullRequestTriageReport {
    /// Report generation date.
    pub date: Date,
    /// Collection of PR triages.
    pub triages: BTreeMap<PullRequestId, PullRequestTriage>,
}

#[derive(Debug)]
pub struct PullRequestTriage {
    /// Pull Request number (e.g. #12345).
    pub id: PullRequestId,
    /// Date of last activity. "Activity" can refer to:
    /// - Author, review or team member commented or reviewed
    /// - bors commented about merge conflicts
    /// - PR was pushed to
    /// and other kinds of activities.
    pub last_activity: Date,
    /// PR author.
    pub author: String,
    /// Who or what the PR is waiting on. This can be the author, the reviewer, another person,
    /// a team, another PR, or perhaps a combination of them or others.
    pub waiting_on: WaitingOn,
    /// The current status of the PR (most recent activity).
    pub most_recent_activity: MostRecentActivity,
    /// Response to the PR. This can include actions such as pinging and closing the issue.
    pub resolution: Resolution,
    /// Additional remarks for this triage.
    pub remarks: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum WaitingOn {
    Author,
    Reviewer,
    Other(String),
}

#[derive(Debug, PartialEq)]
pub enum MostRecentActivity {
    MergeConflicts,
    ReviewerCommented,
    Other(String),
}

#[derive(Debug, PartialEq)]
pub enum Resolution {
    Ping,
    Close,
    Undetermined,
    Other(String),
}

/// The Pull Request number (e.g. #12345). The Rust Zulip will autolink PR (and issue) numbers.
#[derive(Debug, Clone)]
pub struct PullRequestId(pub String);
