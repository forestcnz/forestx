use anyhow::Context;
use anyhow::Result;
use forestx_cloud_config::cloud_config_bundle_loader_for_storage;
use forestx_config::CloudConfigBundleLoader;
use forestx_config::ConfigLoadOptions;
use forestx_core::config::Config;
use forestx_core::config::ConfigBuilder;
use forestx_core::config::LoaderOverrides;
use forestx_core::config::find_forestx_home;
use forestx_core::config::load_config_toml_with_layer_stack;
use forestx_core::config::resolve_bootstrap_auth_keyring_backend_kind;
use forestx_core::config::resolve_bootstrap_auth_route_config;
use forestx_utils_absolute_path::AbsolutePathBuf;
use forestx_utils_cli::CliConfigOverrides;

pub(super) async fn load_mcp_config(
    config_overrides: &CliConfigOverrides,
    loader_overrides: LoaderOverrides,
) -> Result<Config> {
    let cli_overrides = config_overrides
        .parse_overrides()
        .map_err(anyhow::Error::msg)?;
    let forestx_home = find_forestx_home().context("failed to resolve FORESTX_HOME")?;
    let cwd = AbsolutePathBuf::current_dir().context("failed to resolve current directory")?;
    let bootstrap_config = load_config_toml_with_layer_stack(
        forestx_home.as_path(),
        Some(&cwd),
        cli_overrides.clone(),
        ConfigLoadOptions {
            loader_overrides: loader_overrides.clone(),
            strict_config: false,
            cloud_config_bundle: CloudConfigBundleLoader::default(),
        },
    )
    .await
    .context("failed to load bootstrap configuration")?;
    let bootstrap_config_toml = &bootstrap_config.config_toml;
    let auth_route_config = resolve_bootstrap_auth_route_config(
        bootstrap_config_toml,
        bootstrap_config
            .config_layer_stack
            .requirements()
            .feature_requirements
            .as_ref(),
    )
    .context("failed to resolve cloud configuration authentication")?;
    let cloud_config_bundle = cloud_config_bundle_loader_for_storage(
        forestx_home.to_path_buf(),
        /*enable_forestx_api_key_env*/ false,
        bootstrap_config_toml
            .cli_auth_credentials_store
            .unwrap_or_default(),
        resolve_bootstrap_auth_keyring_backend_kind(&bootstrap_config)
            .context("failed to resolve cloud configuration credential storage")?,
        bootstrap_config_toml
            .chatgpt_base_url
            .clone()
            .unwrap_or_else(|| "https://chatgpt.com/backend-api/".to_string()),
        auth_route_config,
    )
    .await;

    ConfigBuilder::default()
        .forestx_home(forestx_home.to_path_buf())
        .cli_overrides(cli_overrides)
        .loader_overrides(loader_overrides)
        .cloud_config_bundle(cloud_config_bundle)
        .build()
        .await
        .context("failed to load configuration")
}
