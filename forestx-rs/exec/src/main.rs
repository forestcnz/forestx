//! Entry-point for the `forestx-exec` binary.
//!
//! When this CLI is invoked normally, it parses the standard `forestx-exec` CLI
//! options and launches the non-interactive Forestx agent. However, if it is
//! invoked with arg0 as `forestx-linux-sandbox`, we instead treat the invocation
//! as a request to run the logic for the standalone `forestx-linux-sandbox`
//! executable (i.e., parse any -s args and then run a *sandboxed* command under
//! Landlock + seccomp.
//!
//! This allows us to ship a completely separate set of functionality as part
//! of the `forestx-exec` binary.
use clap::Parser;
use forestx_arg0::Arg0DispatchPaths;
use forestx_arg0::arg0_dispatch_or_else;
use forestx_exec::Cli;
use forestx_exec::run_main;
use forestx_utils_cli::CliConfigOverrides;

#[derive(Parser, Debug)]
struct TopCli {
    #[arg(long, global = true, hide = true)]
    psp: bool,

    #[clap(flatten)]
    config_overrides: CliConfigOverrides,

    #[clap(flatten)]
    inner: Cli,
}

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|arg0_paths: Arg0DispatchPaths| async move {
        let top_cli = TopCli::parse();
        // Merge root-level overrides into inner CLI struct so downstream logic remains unchanged.
        let mut inner = top_cli.inner;
        inner.psp = top_cli.psp;
        inner
            .config_overrides
            .prepend_root_overrides(top_cli.config_overrides);

        run_main(inner, arg0_paths).await?;
        Ok(())
    })
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod tests;
