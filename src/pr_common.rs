use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PullRequestNumber(pub usize);

impl<'de> Deserialize<'de> for PullRequestNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        let num_str = if let Some((pre, num_str)) = s.split_once('#')
            && pre.is_empty()
        {
            num_str
        } else {
            return Err(Error::custom("failed to split as (#, number)"));
        };
        let num = num_str.parse().map_err(Error::custom)?;
        Ok(PullRequestNumber(num))
    }
}

impl Serialize for PullRequestNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for PullRequestNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReportTemplate {
    pub entries: BTreeMap<PullRequestNumber, ()>,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
    strum_macros::Display,
)]
pub enum StatusLabel {
    #[serde(rename = "S-waiting-on-review")]
    WaitingOnReview,
    #[serde(rename = "S-waiting-on-author")]
    WaitingOnAuthor,
    #[serde(rename = "S-blocked")]
    Blocked,
    None,
}

impl FromStr for StatusLabel {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        Ok(match s {
            "S-waiting-on-review" => StatusLabel::WaitingOnReview,
            "S-waiting-on-author" => StatusLabel::WaitingOnAuthor,
            "S-blocked" => StatusLabel::Blocked,
            _ => StatusLabel::None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Activity {
    MergeConflicts,
    ReviewerCommented,
    AuthorCommittedOrCommented,
    #[serde(untagged)]
    Other(String),
}

impl std::fmt::Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Activity::MergeConflicts => write!(f, "Merge conflicts"),
            Activity::ReviewerCommented => write!(f, "Reviewer commented"),
            Activity::AuthorCommittedOrCommented => write!(f, "Author commited or commented"),
            Activity::Other(s) => write!(f, "Other ({})", s),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FullPullRequestMetadata {
    pub author: String,
    pub assignees: BTreeSet<String>,
    pub status_label: StatusLabel,
    pub last_activity_author: NaiveDate,
    pub last_activity_assignee: NaiveDate,
    pub waiting_on: String,
    pub most_recent_activity: Activity,
    pub remarks: String,
}
