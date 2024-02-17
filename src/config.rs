use std::collections::BTreeSet;

use confique::Config as DeriveConfig;

#[derive(Debug, Default, DeriveConfig)]
pub struct Config {
    /// GitHub personal access token used for authenticating the requests. See
    /// <https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens>
    /// for how to create and manage your personal access tokens.
    pub github_personal_access_token: String,

    /// Set of pull request numbers for which you want the tool to fetch information for.
    #[config(default = [])]
    pub pull_requests: BTreeSet<usize>,
}
