#![recursion_limit = "256"]

use forestx_arg0::Arg0DispatchPaths;
use forestx_arg0::arg0_dispatch_or_else;
use forestx_mcp_server::run_main;
use forestx_utils_cli::CliConfigOverrides;

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|arg0_paths: Arg0DispatchPaths| async move {
        run_main(
            arg0_paths,
            CliConfigOverrides::default(),
            /*strict_config*/ false,
        )
        .await?;
        Ok(())
    })
}
