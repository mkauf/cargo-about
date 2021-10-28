use super::ClarificationFile;
use anyhow::Context as _;

pub fn get(krate: &crate::Krate) -> anyhow::Result<Option<super::Clarification>> {
    if !["prost", "prost-build", "prost-derive", "prost-types"].contains(&krate.name.as_str()) {
        return Ok(None);
    }

    // It looks like between 0.8.0 and 0.9.0 the release process was changed so
    // now there is no longer a .cargo_vcs_info.json file, so we use the commit
    // tag for the release to pull the info

    Ok(Some(super::Clarification {
        license: spdx::Expression::parse("Apache-2.0")
            .context("failed to parse license expression")?,
        override_git_commit: Some(format!("v{}", krate.version)),
        git: vec![ClarificationFile {
            path: "LICENSE".into(),
            license: None,
            checksum: "a60eea817514531668d7e00765731449fe14d059d3249e0bc93b36de45f759f2".to_owned(),
            start: None,
            end: None,
        }],
        files: Vec::new(),
    }))
}
