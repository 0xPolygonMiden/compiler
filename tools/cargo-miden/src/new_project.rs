use std::path::PathBuf;

use anyhow::Context;
use cargo_generate::{GenerateArgs, TemplatePath};
use clap::Args;

/// Create a new Miden project at <path>
#[derive(Args)]
#[clap(disable_version_flag = true)]
pub struct NewCommand {
    /// The path for the generated package.
    #[clap(value_name = "path")]
    pub path: PathBuf,
}

impl NewCommand {
    pub fn exec(self) -> anyhow::Result<PathBuf> {
        let name = self
            .path
            .file_name()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Failed to get the last segment of the provided path for the project name"
                )
            })?
            .to_str()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "The last segment of the provided path must be valid UTF8 to generate a valid \
                     project name"
                )
            })?
            .to_string();

        let generate_args = GenerateArgs {
            template_path: TemplatePath {
                git: Some("https://github.com/0xPolygonMiden/rust-templates".into()),
                auto_path: Some("account".into()),
                // Preparation for alpha release
                // committed in https://github.com/0xPolygonMiden/rust-templates/pull/1
                revision: Some("93e61ba087a982f0b53d098e2581152c34bf801c".to_string()),
                ..Default::default()
            },
            destination: self
                .path
                .parent()
                .map(|p| {
                    use path_absolutize::Absolutize;
                    p.absolutize().map(|p| p.to_path_buf())
                })
                .transpose()
                .context("Failed to convert destination path to an absolute path")?,
            name: Some(name),
            force_git_init: true,
            verbose: true,
            ..Default::default()
        };
        cargo_generate::generate(generate_args)
            .context("Failed to scaffold new Miden project from the template")?;
        return Ok(self.path);
    }
}
