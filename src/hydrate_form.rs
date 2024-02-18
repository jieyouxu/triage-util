use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::Path;
use std::str::FromStr;

use chrono::{Datelike, Utc};
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use indicatif::{ProgressBar, ProgressIterator, ProgressState, ProgressStyle};
use miette::{miette, Context, IntoDiagnostic};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use toml_datetime::{Date, Datetime};
use tracing::*;

use crate::config::Config;
use crate::pr_common::{PullRequestNumber, StatusLabel};

type DateTime = chrono::DateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github_api.schema.graphql",
    query_path = "src/github_pr_request.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct PullRequestMetadataQuery;

pub fn handle_hydrate_form(config: &Config, out: &Path) -> miette::Result<()> {
    let pb = ProgressBar::new(config.pull_requests.len() as u64);
    pb.set_style(ProgressStyle::with_template("fetching pull requests: {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    let mut metadata = Vec::new();

    for pr in config.pull_requests.iter().progress_with(pb) {
        let metadatum = fetch_pull_request_metadata(config, *pr)?;
        metadata.push(metadatum);
    }

    let form = generate_partially_filled_form(&metadata);
    std::fs::write(out, form)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write form to `{}`", out.display()))?;

    info!(
        "successfully created partially filled form at `{}`",
        out.display()
    );

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AutomatablePullRequestMetadata {
    pub issue_number: PullRequestNumber,
    pub author: String,
    pub assignees: BTreeSet<String>,
    pub status_label: StatusLabel,
    // FIXME: find a way to distinguish between last updated at (author) vs last updated at
    // (reviewer/assignee).
    pub _last_updated_at: Datetime,
}

fn fetch_pull_request_metadata(
    config: &Config,
    issue_number: usize,
) -> miette::Result<AutomatablePullRequestMetadata> {
    let variables = pull_request_metadata_query::Variables {
        owner: "rust-lang".to_string(),
        repo: "rust".to_string(),
        number: issue_number as i64,
    };

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!(
                    "Bearer {}",
                    config.github_personal_access_token
                ))
                .unwrap(),
            ))
            .collect(),
        )
        .build()
        .into_diagnostic()?;

    let body = PullRequestMetadataQuery::build_query(pull_request_metadata_query::Variables {
        owner: "rust-lang".to_string(),
        repo: "rust".to_string(),
        number: issue_number as i64,
    });
    let res = client
        .post("https://api.github.com/graphql")
        .json(&body)
        .send()
        .into_diagnostic()?;
    debug!("response body:\n{}", res.text().unwrap());

    let response_body = post_graphql::<PullRequestMetadataQuery, _>(
        &client,
        "https://api.github.com/graphql",
        variables,
    )
    .into_diagnostic()?;

    let response_data: pull_request_metadata_query::ResponseData = response_body
        .data
        .ok_or_else(|| miette!("failed to process response body"))?;

    let Some(pr) = response_data.repository.and_then(|repo| repo.pull_request) else {
        Err(miette!("failed to extract repository from response"))?
    };

    let Some(author) = pr.author.map(|author| author.login) else {
        Err(miette!("failed to extract author from response"))?
    };

    let Some(assignees) = pr.assignees.nodes.map(|nodes| {
        nodes
            .into_iter()
            .flat_map(|node| node.map(|node| node.login))
            .collect::<BTreeSet<_>>()
    }) else {
        Err(miette!("failed to extract assignees from response"))?
    };

    let Some(labels) = pr.labels.map(|labels| {
        labels
            .nodes
            .into_iter()
            .flatten()
            .flat_map(|node| node.map(|node| node.name))
            .collect::<BTreeSet<_>>()
    }) else {
        Err(miette!("failed to extract labels from response"))?
    };

    let status_label = labels
        .iter()
        .find(|label| {
            matches!(
                label.as_str(),
                "S-waiting-on-review" | "S-waiting-on-author" | "S-blocked"
            )
        })
        .map(|s| StatusLabel::from_str(s))
        .transpose()
        .unwrap()
        .unwrap_or(StatusLabel::None);

    let _last_updated_at = pr.updated_at.date_naive();
    let _last_updated_at = Datetime {
        date: Some(Date {
            year: _last_updated_at.year() as u16,
            month: (_last_updated_at.month0() + 1) as u8,
            day: (_last_updated_at.day0() + 1) as u8,
        }),
        time: None,
        offset: None,
    };

    Ok(AutomatablePullRequestMetadata {
        issue_number: PullRequestNumber(issue_number),
        author,
        assignees,
        status_label,
        _last_updated_at,
    })
}

const QUICK_REFERENCE: &str = "\
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
";

// We want to generate entries like:
// ```toml
// ["#12345"]
// author = "jieyouxu"
// assignees = ["a", "b", "c"]
// status_label = "S-waiting-on-review"
// last_activity_author =
// last_activity_assignee =
// waiting_on =
// most_recent_activity =
// ```
fn generate_partially_filled_form(metadata: &Vec<AutomatablePullRequestMetadata>) -> String {
    let mut buf = String::new();

    buf.push_str(QUICK_REFERENCE);
    buf.push('\n');

    for metadatum in metadata {
        buf.push_str(&format!(
            "[pull_requests.{}]\n",
            serde_json::to_string(&metadatum.issue_number).unwrap()
        ));
        buf.push_str(&format!("author = \"{}\"\n", metadatum.author));
        buf.push_str(&format!(
            "assignees = {}\n",
            serde_json::to_string(&metadatum.assignees).unwrap()
        ));
        buf.push_str(&format!(
            "status_label = {}\n",
            serde_json::to_string(&metadatum.status_label).unwrap()
        ));
        buf.push_str("last_activity_author = \n");
        buf.push_str("last_activity_assignee = \n");
        buf.push_str("waiting_on = \n");
        buf.push_str("most_recent_activity = \n");
        buf.push_str("remarks = \"\"\n");
        buf.push('\n');
    }

    buf
}
