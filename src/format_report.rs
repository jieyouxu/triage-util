use std::collections::BTreeMap;
use std::path::Path;

use chrono::{NaiveDate, Utc};
use miette::{miette, Context, IntoDiagnostic};
use serde::{Deserialize, Serialize};
use tabled::settings::Style;
use tabled::{Table, Tabled};
use tracing::*;

use crate::config::Config;
use crate::pr_common::{Activity, FullPullRequestMetadata, PullRequestNumber};

#[derive(Debug, Serialize, Deserialize)]
pub struct TriagedPullRequests {
    pub pull_requests: BTreeMap<PullRequestNumber, FullPullRequestMetadata>,
}

#[derive(Debug, Tabled)]
pub struct TableEntry {
    #[tabled(rename = "PR #")]
    pub pr_number: String,
    #[tabled(rename = "Author")]
    pub author: String,
    #[tabled(rename = "Assignees")]
    pub assignees: String,
    #[tabled(rename = "Last act. (author)")]
    pub last_activity_author: NaiveDate,
    #[tabled(rename = "Last act. (assignee)")]
    pub last_activity_assignee: NaiveDate,
    #[tabled(rename = "Waiting on")]
    pub waiting_on: String,
    #[tabled(rename = "Activity")]
    pub most_recent_activity: Activity,
    #[tabled(rename = "Remarks")]
    pub remarks: String,
}

pub fn handle_format_report(
    _config: &Config,
    in_form_path: &Path,
    out_report_path: &Path,
) -> miette::Result<()> {
    let raw = std::fs::read_to_string(in_form_path)
        .into_diagnostic()
        .wrap_err_with(|| miette!("failed to read form file at `{}`", in_form_path.display()))?;

    let triaged_prs: TriagedPullRequests = toml::from_str(&raw)
        .into_diagnostic()
        .wrap_err_with(|| miette!("failed to parse form as expected TOML format"))?;

    let entries = triaged_prs
        .pull_requests
        .iter()
        .map(|(pr_number, pr)| TableEntry {
            pr_number: pr_number.to_string(),
            author: pr.author.to_string(),
            assignees: pr
                .assignees
                .iter()
                .cloned()
                .intersperse(", ".to_string())
                .collect::<String>(),
            last_activity_author: pr.last_activity_author,
            last_activity_assignee: pr.last_activity_assignee,
            waiting_on: pr.waiting_on.to_string(),
            most_recent_activity: pr.most_recent_activity.clone(),
            remarks: pr.remarks.clone(),
        })
        .collect::<Vec<_>>();

    let mut table = Table::new(entries);
    table.with(Style::markdown());
    debug!("table = \n{}", table);

    let mut report = String::new();
    report.push_str(&format!(
        "### Pull request triage report ({})\n\n",
        Utc::now().date_naive()
    ));
    report.push_str(&format!("{}", table));

    std::fs::write(out_report_path, report)
        .into_diagnostic()
        .wrap_err_with(|| miette!("failed to write report to `{}`", out_report_path.display()))?;

    info!(
        "successfully generated report at `{}`",
        out_report_path.display()
    );

    Ok(())
}
