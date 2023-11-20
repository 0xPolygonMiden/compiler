use std::path::PathBuf;

use anyhow::Context;
use cargo_generate::GenerateArgs;
use cargo_generate::TemplatePath;

pub fn new_project(path: PathBuf) -> anyhow::Result<()> {
    let name = path
        .file_name()
        .ok_or_else(|| {
            anyhow::anyhow!("Failed to get the last segment of the provided path for the project name")
        })?
        .to_str()
        .ok_or_else(|| {
            anyhow::anyhow!("The last segment of the provided path must be valid UTF8 to generate a valid project name")
        })?
        .to_string();

    let generate_args = GenerateArgs {
        template_path: TemplatePath {
            git: Some("https://github.com/greenhat/miden-project-template".into()),
            ..Default::default()
        },
        destination: path
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
    return Ok(());
}
