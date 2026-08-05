use crate::bespoke_event_handling::apply_bespoke_event_handling;
use crate::command_exec::CommandExecManager;
use crate::command_exec::StartCommandExecParams;
use crate::config_manager::ConfigManager;
use crate::error_code::INPUT_TOO_LARGE_ERROR_CODE;
use crate::error_code::invalid_params;
use crate::models::supported_models;
use crate::outgoing_message::ConnectionId;
use crate::outgoing_message::ConnectionRequestId;
use crate::outgoing_message::OutgoingMessageSender;
use crate::outgoing_message::RequestContext;
use crate::outgoing_message::ThreadScopedOutgoingMessageSender;
use crate::skills_watcher::SkillsWatcher;
use crate::thread_status::ThreadWatchManager;
use crate::thread_status::resolve_thread_status;
use chrono::Duration as ChronoDuration;
use chrono::SecondsFormat;
use forestx_analytics::AnalyticsEventsClient;
use forestx_analytics::AnalyticsJsonRpcError;
use forestx_analytics::InputError;
use forestx_analytics::TurnSteerRequestError;
use forestx_app_server_protocol::Account;
use forestx_app_server_protocol::AccountLoginCompletedNotification;
use forestx_app_server_protocol::AccountTokenUsageDailyBucket;
use forestx_app_server_protocol::AccountTokenUsageSummary;
use forestx_app_server_protocol::AccountUpdatedNotification;
use forestx_app_server_protocol::AddCreditsNudgeCreditType;
use forestx_app_server_protocol::AddCreditsNudgeEmailStatus;
use forestx_app_server_protocol::AdditionalContextEntry;
use forestx_app_server_protocol::AdditionalContextKind;
use forestx_app_server_protocol::AppListUpdatedNotification;
use forestx_app_server_protocol::AppSummary;
use forestx_app_server_protocol::AppTemplateSummary;
use forestx_app_server_protocol::AppTemplateUnavailableReason;
use forestx_app_server_protocol::AppsInstalledParams;
use forestx_app_server_protocol::AppsInstalledResponse;
use forestx_app_server_protocol::AppsListParams;
use forestx_app_server_protocol::AppsListResponse;
use forestx_app_server_protocol::AppsReadParams;
use forestx_app_server_protocol::AppsReadResponse;
use forestx_app_server_protocol::AskForApproval;
use forestx_app_server_protocol::AuthMode;
use forestx_app_server_protocol::CancelLoginAccountParams;
use forestx_app_server_protocol::CancelLoginAccountResponse;
use forestx_app_server_protocol::CancelLoginAccountStatus;
use forestx_app_server_protocol::ClientInfo;
use forestx_app_server_protocol::ClientRequest;
use forestx_app_server_protocol::ClientResponsePayload;
use forestx_app_server_protocol::ForestxErrorInfo;
use forestx_app_server_protocol::CollaborationModeListParams;
use forestx_app_server_protocol::CollaborationModeListResponse;
use forestx_app_server_protocol::CommandExecParams;
use forestx_app_server_protocol::CommandExecResizeParams;
use forestx_app_server_protocol::CommandExecTerminateParams;
use forestx_app_server_protocol::CommandExecWriteParams;
use forestx_app_server_protocol::ConfigWarningNotification;
use forestx_app_server_protocol::ConsumeAccountRateLimitResetCreditOutcome;
use forestx_app_server_protocol::ConsumeAccountRateLimitResetCreditParams;
use forestx_app_server_protocol::ConsumeAccountRateLimitResetCreditResponse;
use forestx_app_server_protocol::ConversationGitInfo;
use forestx_app_server_protocol::ConversationSummary;
use forestx_app_server_protocol::DeprecationNoticeNotification;
use forestx_app_server_protocol::DynamicToolFunctionSpec;
use forestx_app_server_protocol::DynamicToolNamespaceTool;
use forestx_app_server_protocol::DynamicToolSpec;
use forestx_app_server_protocol::EnvironmentAddParams;
use forestx_app_server_protocol::EnvironmentAddResponse;
use forestx_app_server_protocol::EnvironmentInfoParams;
use forestx_app_server_protocol::EnvironmentInfoResponse;
use forestx_app_server_protocol::EnvironmentShellInfo;
use forestx_app_server_protocol::EnvironmentStatusKind;
use forestx_app_server_protocol::EnvironmentStatusParams;
use forestx_app_server_protocol::EnvironmentStatusResponse;
use forestx_app_server_protocol::ExperimentalFeature as ApiExperimentalFeature;
use forestx_app_server_protocol::ExperimentalFeatureListParams;
use forestx_app_server_protocol::ExperimentalFeatureListResponse;
use forestx_app_server_protocol::ExperimentalFeatureStage as ApiExperimentalFeatureStage;
use forestx_app_server_protocol::FeedbackUploadParams;
use forestx_app_server_protocol::FeedbackUploadResponse;
use forestx_app_server_protocol::GetAccountParams;
use forestx_app_server_protocol::GetAccountRateLimitsResponse;
use forestx_app_server_protocol::GetAccountResponse;
use forestx_app_server_protocol::GetAccountTokenUsageResponse;
use forestx_app_server_protocol::GetAuthStatusParams;
use forestx_app_server_protocol::GetAuthStatusResponse;
use forestx_app_server_protocol::GetConversationSummaryParams;
use forestx_app_server_protocol::GetConversationSummaryResponse;
use forestx_app_server_protocol::GetWorkspaceMessagesResponse;
use forestx_app_server_protocol::GitDiffToRemoteParams;
use forestx_app_server_protocol::GitDiffToRemoteResponse;
use forestx_app_server_protocol::GitInfo as ApiGitInfo;
use forestx_app_server_protocol::HookMetadata;
use forestx_app_server_protocol::HooksListParams;
use forestx_app_server_protocol::HooksListResponse;
use forestx_app_server_protocol::InitializeParams;
use forestx_app_server_protocol::InitializeResponse;
use forestx_app_server_protocol::InstalledApp;
use forestx_app_server_protocol::JSONRPCErrorError;
use forestx_app_server_protocol::ListMcpServerStatusParams;
use forestx_app_server_protocol::ListMcpServerStatusResponse;
use forestx_app_server_protocol::LoginAccountParams;
use forestx_app_server_protocol::LoginAccountResponse;
use forestx_app_server_protocol::LoginApiKeyParams;
use forestx_app_server_protocol::LoginAppBrand;
use forestx_app_server_protocol::LogoutAccountResponse;
use forestx_app_server_protocol::MarketplaceAddParams;
use forestx_app_server_protocol::MarketplaceAddResponse;
use forestx_app_server_protocol::MarketplaceInterface;
use forestx_app_server_protocol::MarketplaceRemoveParams;
use forestx_app_server_protocol::MarketplaceRemoveResponse;
use forestx_app_server_protocol::MarketplaceUpgradeErrorInfo;
use forestx_app_server_protocol::MarketplaceUpgradeParams;
use forestx_app_server_protocol::MarketplaceUpgradeResponse;
use forestx_app_server_protocol::McpResourceReadParams;
use forestx_app_server_protocol::McpResourceReadResponse;
use forestx_app_server_protocol::McpServerOauthLoginCompletedNotification;
use forestx_app_server_protocol::McpServerOauthLoginParams;
use forestx_app_server_protocol::McpServerOauthLoginResponse;
use forestx_app_server_protocol::McpServerRefreshResponse;
use forestx_app_server_protocol::McpServerStatus;
use forestx_app_server_protocol::McpServerStatusDetail;
use forestx_app_server_protocol::McpServerToolCallParams;
use forestx_app_server_protocol::McpServerToolCallResponse;
use forestx_app_server_protocol::MemoryResetResponse;
use forestx_app_server_protocol::MockExperimentalMethodParams;
use forestx_app_server_protocol::MockExperimentalMethodResponse;
use forestx_app_server_protocol::ModelListParams;
use forestx_app_server_protocol::ModelListResponse;
use forestx_app_server_protocol::PermissionProfileListParams;
use forestx_app_server_protocol::PermissionProfileListResponse;
use forestx_app_server_protocol::PermissionProfileSummary;
use forestx_app_server_protocol::PluginDetail;
use forestx_app_server_protocol::PluginInstallParams;
use forestx_app_server_protocol::PluginInstallResponse;
use forestx_app_server_protocol::PluginInstalledParams;
use forestx_app_server_protocol::PluginInstalledResponse;
use forestx_app_server_protocol::PluginInterface;
use forestx_app_server_protocol::PluginListMarketplaceKind;
use forestx_app_server_protocol::PluginListParams;
use forestx_app_server_protocol::PluginListResponse;
use forestx_app_server_protocol::PluginMarketplaceEntry;
use forestx_app_server_protocol::PluginReadParams;
use forestx_app_server_protocol::PluginReadResponse;
use forestx_app_server_protocol::PluginShareCheckoutParams;
use forestx_app_server_protocol::PluginShareCheckoutResponse;
use forestx_app_server_protocol::PluginShareContext;
use forestx_app_server_protocol::PluginShareDeleteParams;
use forestx_app_server_protocol::PluginShareDeleteResponse;
use forestx_app_server_protocol::PluginShareDiscoverability;
use forestx_app_server_protocol::PluginShareListItem;
use forestx_app_server_protocol::PluginShareListParams;
use forestx_app_server_protocol::PluginShareListResponse;
use forestx_app_server_protocol::PluginSharePrincipal;
use forestx_app_server_protocol::PluginSharePrincipalType;
use forestx_app_server_protocol::PluginShareSaveParams;
use forestx_app_server_protocol::PluginShareSaveResponse;
use forestx_app_server_protocol::PluginShareTarget;
use forestx_app_server_protocol::PluginShareUpdateDiscoverability;
use forestx_app_server_protocol::PluginShareUpdateTargetsParams;
use forestx_app_server_protocol::PluginShareUpdateTargetsResponse;
use forestx_app_server_protocol::PluginSkillReadParams;
use forestx_app_server_protocol::PluginSkillReadResponse;
use forestx_app_server_protocol::PluginSource;
use forestx_app_server_protocol::PluginSummary;
use forestx_app_server_protocol::PluginUninstallParams;
use forestx_app_server_protocol::PluginUninstallResponse;
use forestx_app_server_protocol::RateLimitResetCredit;
use forestx_app_server_protocol::RateLimitResetCreditStatus;
use forestx_app_server_protocol::RateLimitResetCreditsSummary;
use forestx_app_server_protocol::RateLimitResetType;
use forestx_app_server_protocol::RequestId;
use forestx_app_server_protocol::ReviewDelivery as ApiReviewDelivery;
use forestx_app_server_protocol::ReviewStartParams;
use forestx_app_server_protocol::ReviewStartResponse;
use forestx_app_server_protocol::ReviewTarget as ApiReviewTarget;
use forestx_app_server_protocol::SandboxMode;
use forestx_app_server_protocol::SendAddCreditsNudgeEmailParams;
use forestx_app_server_protocol::SendAddCreditsNudgeEmailResponse;
use forestx_app_server_protocol::ServerNotification;
use forestx_app_server_protocol::ServerRequestResolvedNotification;
use forestx_app_server_protocol::SkillSummary;
use forestx_app_server_protocol::SkillsConfigWriteParams;
use forestx_app_server_protocol::SkillsConfigWriteResponse;
use forestx_app_server_protocol::SkillsExtraRootsSetParams;
use forestx_app_server_protocol::SkillsExtraRootsSetResponse;
use forestx_app_server_protocol::SkillsListParams;
use forestx_app_server_protocol::SkillsListResponse;
use forestx_app_server_protocol::SortDirection;
use forestx_app_server_protocol::Thread;
use forestx_app_server_protocol::ThreadApproveGuardianDeniedActionParams;
use forestx_app_server_protocol::ThreadApproveGuardianDeniedActionResponse;
use forestx_app_server_protocol::ThreadArchiveParams;
use forestx_app_server_protocol::ThreadArchiveResponse;
use forestx_app_server_protocol::ThreadArchivedNotification;
use forestx_app_server_protocol::ThreadBackgroundTerminal;
use forestx_app_server_protocol::ThreadBackgroundTerminalsCleanParams;
use forestx_app_server_protocol::ThreadBackgroundTerminalsCleanResponse;
use forestx_app_server_protocol::ThreadBackgroundTerminalsListParams;
use forestx_app_server_protocol::ThreadBackgroundTerminalsListResponse;
use forestx_app_server_protocol::ThreadBackgroundTerminalsTerminateParams;
use forestx_app_server_protocol::ThreadBackgroundTerminalsTerminateResponse;
use forestx_app_server_protocol::ThreadClosedNotification;
use forestx_app_server_protocol::ThreadCompactStartParams;
use forestx_app_server_protocol::ThreadCompactStartResponse;
use forestx_app_server_protocol::ThreadDecrementElicitationParams;
use forestx_app_server_protocol::ThreadDecrementElicitationResponse;
use forestx_app_server_protocol::ThreadDeleteParams;
use forestx_app_server_protocol::ThreadDeleteResponse;
use forestx_app_server_protocol::ThreadDeletedNotification;
use forestx_app_server_protocol::ThreadForkParams;
use forestx_app_server_protocol::ThreadForkResponse;
use forestx_app_server_protocol::ThreadGoal;
use forestx_app_server_protocol::ThreadGoalClearParams;
use forestx_app_server_protocol::ThreadGoalClearResponse;
use forestx_app_server_protocol::ThreadGoalClearedNotification;
use forestx_app_server_protocol::ThreadGoalGetParams;
use forestx_app_server_protocol::ThreadGoalGetResponse;
use forestx_app_server_protocol::ThreadGoalSetParams;
use forestx_app_server_protocol::ThreadGoalSetResponse;
use forestx_app_server_protocol::ThreadGoalStatus;
use forestx_app_server_protocol::ThreadGoalUpdatedNotification;
use forestx_app_server_protocol::ThreadHistoryBuilder;
#[cfg(test)]
use forestx_app_server_protocol::ThreadHistoryMode;
use forestx_app_server_protocol::ThreadIncrementElicitationParams;
use forestx_app_server_protocol::ThreadIncrementElicitationResponse;
use forestx_app_server_protocol::ThreadInjectItemsParams;
use forestx_app_server_protocol::ThreadInjectItemsResponse;
use forestx_app_server_protocol::ThreadItem;
use forestx_app_server_protocol::ThreadItemEntry;
use forestx_app_server_protocol::ThreadItemsListParams;
use forestx_app_server_protocol::ThreadItemsListResponse;
use forestx_app_server_protocol::ThreadListCwdFilter;
use forestx_app_server_protocol::ThreadListParams;
use forestx_app_server_protocol::ThreadListResponse;
use forestx_app_server_protocol::ThreadLoadedListParams;
use forestx_app_server_protocol::ThreadLoadedListResponse;
use forestx_app_server_protocol::ThreadMemoryModeSetParams;
use forestx_app_server_protocol::ThreadMemoryModeSetResponse;
use forestx_app_server_protocol::ThreadMetadataGitInfoUpdateParams;
use forestx_app_server_protocol::ThreadMetadataUpdateParams;
use forestx_app_server_protocol::ThreadMetadataUpdateResponse;
use forestx_app_server_protocol::ThreadNameUpdatedNotification;
use forestx_app_server_protocol::ThreadReadParams;
use forestx_app_server_protocol::ThreadReadResponse;
use forestx_app_server_protocol::ThreadRealtimeAppendAudioParams;
use forestx_app_server_protocol::ThreadRealtimeAppendAudioResponse;
use forestx_app_server_protocol::ThreadRealtimeAppendSpeechParams;
use forestx_app_server_protocol::ThreadRealtimeAppendSpeechResponse;
use forestx_app_server_protocol::ThreadRealtimeAppendTextParams;
use forestx_app_server_protocol::ThreadRealtimeAppendTextResponse;
use forestx_app_server_protocol::ThreadRealtimeListVoicesResponse;
use forestx_app_server_protocol::ThreadRealtimeStartParams;
use forestx_app_server_protocol::ThreadRealtimeStartResponse;
use forestx_app_server_protocol::ThreadRealtimeStartTransport;
use forestx_app_server_protocol::ThreadRealtimeStopParams;
use forestx_app_server_protocol::ThreadRealtimeStopResponse;
use forestx_app_server_protocol::ThreadResumeInitialTurnsPageParams;
use forestx_app_server_protocol::ThreadResumeParams;
use forestx_app_server_protocol::ThreadResumeResponse;
use forestx_app_server_protocol::ThreadRollbackParams;
use forestx_app_server_protocol::ThreadSearchOccurrence;
use forestx_app_server_protocol::ThreadSearchOccurrencesParams;
use forestx_app_server_protocol::ThreadSearchOccurrencesResponse;
use forestx_app_server_protocol::ThreadSearchParams;
use forestx_app_server_protocol::ThreadSearchResponse;
use forestx_app_server_protocol::ThreadSearchResult;
use forestx_app_server_protocol::ThreadSearchSortKey;
use forestx_app_server_protocol::ThreadSearchTextRange;
use forestx_app_server_protocol::ThreadSetNameParams;
use forestx_app_server_protocol::ThreadSetNameResponse;
use forestx_app_server_protocol::ThreadSettings;
use forestx_app_server_protocol::ThreadSettingsUpdateParams;
use forestx_app_server_protocol::ThreadSettingsUpdateResponse;
use forestx_app_server_protocol::ThreadShellCommandParams;
use forestx_app_server_protocol::ThreadShellCommandResponse;
use forestx_app_server_protocol::ThreadSortKey;
use forestx_app_server_protocol::ThreadSourceKind;
use forestx_app_server_protocol::ThreadStartParams;
use forestx_app_server_protocol::ThreadStartResponse;
use forestx_app_server_protocol::ThreadStartedNotification;
use forestx_app_server_protocol::ThreadStatus;
use forestx_app_server_protocol::ThreadTurnsListParams;
use forestx_app_server_protocol::ThreadTurnsListResponse;
use forestx_app_server_protocol::ThreadUnarchiveParams;
use forestx_app_server_protocol::ThreadUnarchiveResponse;
use forestx_app_server_protocol::ThreadUnarchivedNotification;
use forestx_app_server_protocol::ThreadUnsubscribeParams;
use forestx_app_server_protocol::ThreadUnsubscribeResponse;
use forestx_app_server_protocol::ThreadUnsubscribeStatus;
use forestx_app_server_protocol::Turn;
use forestx_app_server_protocol::TurnEnvironmentParams;
use forestx_app_server_protocol::TurnError;
use forestx_app_server_protocol::TurnInterruptParams;
use forestx_app_server_protocol::TurnInterruptResponse;
use forestx_app_server_protocol::TurnItemsView;
use forestx_app_server_protocol::TurnStartParams;
use forestx_app_server_protocol::TurnStartResponse;
use forestx_app_server_protocol::TurnStatus;
use forestx_app_server_protocol::TurnSteerParams;
use forestx_app_server_protocol::TurnSteerResponse;
use forestx_app_server_protocol::UserInput as V2UserInput;
use forestx_app_server_protocol::WindowsSandboxReadiness;
use forestx_app_server_protocol::WindowsSandboxReadinessResponse;
use forestx_app_server_protocol::WindowsSandboxSetupCompletedNotification;
use forestx_app_server_protocol::WindowsSandboxSetupMode;
use forestx_app_server_protocol::WindowsSandboxSetupStartParams;
use forestx_app_server_protocol::WindowsSandboxSetupStartResponse;
use forestx_app_server_protocol::WorkspaceMessage;
use forestx_app_server_protocol::WorkspaceMessageType;
use forestx_arg0::Arg0DispatchPaths;
use forestx_backend_client::AddCreditsNudgeCreditType as BackendAddCreditsNudgeCreditType;
use forestx_backend_client::Client as BackendClient;
use forestx_backend_client::ForestxWorkspaceMessage as BackendWorkspaceMessage;
use forestx_backend_client::ForestxWorkspaceMessageType as BackendWorkspaceMessageType;
use forestx_backend_client::ForestxWorkspaceMessagesResponse as BackendWorkspaceMessagesResponse;
use forestx_backend_client::ConsumeRateLimitResetCreditCode as BackendConsumeRateLimitResetCreditCode;
use forestx_backend_client::RateLimitResetCreditDetails as BackendRateLimitResetCreditDetails;
use forestx_backend_client::RateLimitResetCreditsDetails as BackendRateLimitResetCreditsDetails;
use forestx_backend_client::RequestError as BackendRequestError;
use forestx_backend_client::TokenUsageProfile;
use forestx_chatgpt::connectors;
use forestx_chatgpt::workspace_settings;
use forestx_config::CloudConfigBundleLoadError;
use forestx_config::CloudConfigBundleLoadErrorCode;
use forestx_config::ConfigLayerStack;
use forestx_config::loader::project_trust_key;
use forestx_config::types::McpServerTransportConfig;
use forestx_connectors::AppInfo;
use forestx_core::ForestxThread;
use forestx_core::ForestxThreadSettingsOverrides;
use forestx_core::ForkSnapshot;
use forestx_core::McpManager;
use forestx_core::NewThread;
#[cfg(test)]
use forestx_core::SessionMeta;
use forestx_core::StartThreadOptions;
use forestx_core::SteerInputError;
use forestx_core::ThreadConfigSnapshot;
use forestx_core::ThreadManager;
use forestx_core::config::Config;
use forestx_core::config::ConfigOverrides;
use forestx_core::config::NetworkProxyAuditMetadata;
use forestx_core::config::edit::ConfigEdit;
use forestx_core::config::edit::ConfigEditsBuilder;
use forestx_core::connectors::AccessibleConnectorsStatus;
use forestx_core::exec::ExecCapturePolicy;
use forestx_core::exec::ExecExpiration;
use forestx_core::exec::ExecParams;
use forestx_core::exec_env::create_env;
use forestx_core::path_utils;
#[cfg(test)]
use forestx_core::read_head_for_summary;
use forestx_core::sandboxing::SandboxPermissions;
use forestx_core::truncate_rollout_after_turn_id;
use forestx_core::truncate_rollout_before_turn_id;
use forestx_core::windows_sandbox::WindowsSandboxLevelExt;
use forestx_core::windows_sandbox::WindowsSandboxSetupMode as CoreWindowsSandboxSetupMode;
use forestx_core::windows_sandbox::WindowsSandboxSetupRequest;
use forestx_core::windows_sandbox::sandbox_setup_is_complete;
use forestx_core_plugins::PluginInstallError as CorePluginInstallError;
use forestx_core_plugins::PluginInstallRequest;
use forestx_core_plugins::PluginReadRequest;
use forestx_core_plugins::PluginUninstallError as CorePluginUninstallError;
use forestx_core_plugins::PluginsManager;
use forestx_core_plugins::loader::load_plugin_apps;
use forestx_core_plugins::manifest::PluginManifestInterface;
use forestx_core_plugins::marketplace::MarketplaceError;
use forestx_core_plugins::marketplace::MarketplacePluginSource;
use forestx_core_plugins::marketplace_add::MarketplaceAddError;
use forestx_core_plugins::marketplace_add::MarketplaceAddRequest;
use forestx_core_plugins::marketplace_add::add_marketplace as add_marketplace_to_forestx_home;
use forestx_core_plugins::marketplace_remove::MarketplaceRemoveError;
use forestx_core_plugins::marketplace_remove::MarketplaceRemoveRequest as CoreMarketplaceRemoveRequest;
use forestx_core_plugins::marketplace_remove::remove_marketplace;
use forestx_core_plugins::remote::RemoteMarketplace;
use forestx_core_plugins::remote::RemoteMarketplaceSource;
use forestx_core_plugins::remote::RemotePluginCatalogError;
use forestx_core_plugins::remote::RemotePluginDetail as RemoteCatalogPluginDetail;
use forestx_core_plugins::remote::RemotePluginServiceConfig;
use forestx_core_plugins::remote::RemotePluginShareContext as RemoteCatalogPluginShareContext;
use forestx_core_plugins::remote::RemotePluginShareSummary as RemoteCatalogPluginShareSummary;
use forestx_core_plugins::remote::RemotePluginSummary as RemoteCatalogPluginSummary;
use forestx_exec_server::EnvironmentManager;
use forestx_exec_server::EnvironmentObservedStatus;
use forestx_exec_server::LOCAL_ENVIRONMENT_ID;
use forestx_exec_server::LOCAL_FS;
use forestx_features::FEATURES;
use forestx_features::Feature;
use forestx_features::Stage;
use forestx_feedback::ForestxFeedback;
use forestx_feedback::FeedbackAttachmentPath;
use forestx_feedback::FeedbackUploadOptions;
use forestx_git_utils::git_diff_to_remote;
use forestx_git_utils::resolve_root_git_project_for_trust;
use forestx_login::AuthManager;
use forestx_login::FORESTX_OPEN_APP_URL;
use forestx_login::ForestxAuth;
use forestx_login::LoginSuccessPage;
use forestx_login::LoginSuccessPageBrand;
use forestx_login::ServerOptions as LoginServerOptions;
use forestx_login::ShutdownHandle;
use forestx_login::complete_device_code_login;
use forestx_login::login_with_api_key;
use forestx_login::login_with_bedrock_api_key;
use forestx_login::oauth_client_id;
use forestx_login::request_device_code;
use forestx_login::run_login_server;
use forestx_mcp::McpRuntimeContext;
use forestx_mcp::McpServerStatusSnapshot;
use forestx_mcp::McpSnapshotDetail;
use forestx_mcp::collect_mcp_server_status_snapshot_with_detail;
use forestx_mcp::discover_supported_scopes;
use forestx_mcp::read_mcp_resource as read_mcp_resource_without_thread;
use forestx_mcp::resolve_oauth_scopes;
use forestx_memories_write::clear_memory_roots_contents;
use forestx_model_provider::create_model_provider;
use forestx_models_manager::collaboration_mode_presets::builtin_collaboration_mode_presets;
use forestx_protocol::ThreadId;
use forestx_protocol::config_types::CollaborationMode;
use forestx_protocol::config_types::ForcedLoginMethod;
use forestx_protocol::config_types::Personality;
use forestx_protocol::config_types::ReasoningSummary;
use forestx_protocol::config_types::TrustLevel;
use forestx_protocol::config_types::WindowsSandboxLevel;
use forestx_protocol::error::ForestxErr;
use forestx_protocol::error::Result as ForestxResult;
#[cfg(test)]
use forestx_protocol::items::TurnItem;
use forestx_protocol::models::ResponseItem;
use forestx_protocol::openai_models::ReasoningEffort;
#[cfg(test)]
use forestx_protocol::permissions::FileSystemSandboxPolicy;
use forestx_protocol::protocol::AgentStatus;
use forestx_protocol::protocol::ConversationAudioParams;
use forestx_protocol::protocol::ConversationSpeechParams;
use forestx_protocol::protocol::ConversationStartParams;
use forestx_protocol::protocol::ConversationStartTransport;
use forestx_protocol::protocol::ConversationTextParams;
use forestx_protocol::protocol::EventMsg;
#[cfg(test)]
use forestx_protocol::protocol::GitInfo as CoreGitInfo;
use forestx_protocol::protocol::InitialHistory;
use forestx_protocol::protocol::McpAuthStatus as CoreMcpAuthStatus;
use forestx_protocol::protocol::Op;
use forestx_protocol::protocol::RealtimeVoicesList;
use forestx_protocol::protocol::ResumedHistory;
use forestx_protocol::protocol::ReviewDelivery as CoreReviewDelivery;
use forestx_protocol::protocol::ReviewRequest;
use forestx_protocol::protocol::ReviewTarget as CoreReviewTarget;
use forestx_protocol::protocol::RolloutItem;
use forestx_protocol::protocol::SessionConfiguredEvent;
#[cfg(test)]
use forestx_protocol::protocol::SessionMetaLine;
use forestx_protocol::protocol::TurnEnvironmentSelection;
use forestx_protocol::protocol::TurnEnvironmentSelections;
use forestx_protocol::protocol::W3cTraceContext;
use forestx_protocol::protocol::strip_user_message_prefix;
use forestx_protocol::user_input::MAX_USER_INPUT_TEXT_CHARS;
use forestx_protocol::user_input::UserInput as CoreInputItem;
use forestx_rmcp_client::StreamableHttpRedirectMode;
use forestx_rmcp_client::perform_oauth_login_return_url;
use forestx_rollout::is_persisted_rollout_item;
use forestx_rollout::state_db::StateDbHandle;
use forestx_rollout::state_db::reconcile_rollout;
use forestx_state::ThreadMetadata;
use forestx_state::log_db::LogDbLayer;
use forestx_thread_store::ArchiveThreadParams as StoreArchiveThreadParams;
use forestx_thread_store::ArchiveThreadsParams as StoreArchiveThreadsParams;
use forestx_thread_store::DeleteThreadsParams as StoreDeleteThreadsParams;
use forestx_thread_store::GitInfoPatch as StoreGitInfoPatch;
use forestx_thread_store::ItemSortKey as StoreItemSortKey;
use forestx_thread_store::ListItemsParams as StoreListItemsParams;
use forestx_thread_store::ListThreadsParams as StoreListThreadsParams;
use forestx_thread_store::ListTurnsParams as StoreListTurnsParams;
use forestx_thread_store::LoadThreadHistoryParams as StoreLoadThreadHistoryParams;
use forestx_thread_store::LocalThreadStore;
use forestx_thread_store::ReadThreadByRolloutPathParams as StoreReadThreadByRolloutPathParams;
use forestx_thread_store::ReadThreadParams as StoreReadThreadParams;
use forestx_thread_store::SearchThreadOccurrencesParams as StoreSearchThreadOccurrencesParams;
use forestx_thread_store::SearchThreadsParams as StoreSearchThreadsParams;
use forestx_thread_store::SortDirection as StoreSortDirection;
use forestx_thread_store::StoredThread;
use forestx_thread_store::StoredTurn;
use forestx_thread_store::StoredTurnItemsView;
use forestx_thread_store::StoredTurnStatus;
use forestx_thread_store::ThreadMetadataPatch as StoreThreadMetadataPatch;
use forestx_thread_store::ThreadRelationFilter as StoreThreadRelationFilter;
use forestx_thread_store::ThreadSortKey as StoreThreadSortKey;
use forestx_thread_store::ThreadStore;
use forestx_thread_store::ThreadStoreError;
use forestx_utils_absolute_path::AbsolutePathBuf;
use forestx_utils_pty::DEFAULT_OUTPUT_BYTES_CAP;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error as IoError;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;
use tokio::sync::broadcast;
use tokio::sync::oneshot;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tokio_util::sync::DropGuard;
use tokio_util::task::TaskTracker;
use toml::Value as TomlValue;
use tracing::Instrument;
use tracing::error;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

#[cfg(test)]
use forestx_app_server_protocol::ServerRequest;

mod account_processor;
mod apps_processor;
mod bedrock_auth;
mod catalog_processor;
mod command_exec_processor;
mod config_processor;
mod environment_processor;
mod feedback_doctor_report;
mod feedback_processor;
mod fs_processor;
mod git_processor;
mod initialize_processor;
mod marketplace_processor;
mod mcp_processor;
mod plugins;
mod process_exec_processor;
mod remote_control_processor;
mod search;
mod thread_enrichment;
mod thread_fork_goal;
mod thread_processor;
mod thread_sections;
mod token_usage_replay;
mod turn_processor;
mod windows_sandbox_processor;

pub(crate) use account_processor::AccountRequestProcessor;
pub(crate) use apps_processor::AppsRequestProcessor;
pub(crate) use catalog_processor::CatalogRequestProcessor;
pub(crate) use command_exec_processor::CommandExecRequestProcessor;
pub(crate) use config_processor::ConfigRequestProcessor;
pub(crate) use environment_processor::EnvironmentRequestProcessor;
pub(crate) use feedback_processor::FeedbackRequestProcessor;
pub(crate) use fs_processor::FsRequestProcessor;
pub(crate) use git_processor::GitRequestProcessor;
pub(crate) use initialize_processor::InitializeRequestProcessor;
pub(crate) use marketplace_processor::MarketplaceRequestProcessor;
pub(crate) use mcp_processor::McpRequestProcessor;
pub(crate) use plugins::PluginRequestProcessor;
pub(crate) use process_exec_processor::ProcessExecRequestProcessor;
pub(crate) use remote_control_processor::RemoteControlRequestProcessor;
pub(crate) use search::SearchRequestProcessor;
pub(crate) use thread_goal_processor::ThreadGoalRequestProcessor;
pub(crate) use thread_processor::ThreadRequestProcessor;
pub(crate) use turn_processor::TurnRequestProcessor;
pub(crate) use windows_sandbox_processor::WindowsSandboxRequestProcessor;

use crate::error_code::internal_error;
use crate::error_code::invalid_request;
use crate::filters::compute_source_filters;
use crate::filters::source_kind_matches;
use crate::thread_state::ConnectionCapabilities;
use crate::thread_state::ThreadListenerCommand;
use crate::thread_state::ThreadState;
use crate::thread_state::ThreadStateManager;
use token_usage_replay::restored_token_usage_turn_id;
use token_usage_replay::send_thread_token_usage_update_to_connection;

fn resolve_request_cwd(cwd: Option<PathBuf>) -> Result<Option<AbsolutePathBuf>, JSONRPCErrorError> {
    cwd.map(|cwd| {
        AbsolutePathBuf::relative_to_current_dir(path_utils::normalize_for_native_workdir(cwd))
            .map_err(|err| invalid_request(format!("invalid cwd: {err}")))
    })
    .transpose()
}

fn resolve_turn_environment_selections(
    thread_manager: &ThreadManager,
    environments: Option<Vec<TurnEnvironmentParams>>,
) -> Result<Option<Vec<TurnEnvironmentSelection>>, JSONRPCErrorError> {
    let Some(environments) = environments else {
        return Ok(None);
    };
    let mut selections = Vec::with_capacity(environments.len());
    for environment in environments {
        let environment_id = environment.environment_id;
        let cwd = environment
            .cwd
            .to_inferred_path_uri()
            .ok_or_else(|| {
                invalid_request(format!(
                    "invalid cwd for environment `{environment_id}`: path `{}` does not use absolute POSIX or Windows path syntax",
                    environment.cwd
                ))
            })?;
        let workspace_roots = environment
            .runtime_workspace_roots
            .map(|roots| {
                let mut resolved_roots = Vec::new();
                for root in roots {
                    let root = root.to_inferred_path_uri().ok_or_else(|| {
                        invalid_request(format!(
                            "invalid runtime workspace root for environment `{environment_id}`: path `{root}` does not use absolute POSIX or Windows path syntax"
                        ))
                    })?;
                    if !resolved_roots.contains(&root) {
                        resolved_roots.push(root);
                    }
                }
                Ok::<_, JSONRPCErrorError>(resolved_roots)
            })
            .transpose()?
            .unwrap_or_else(|| vec![cwd.clone()]);
        selections.push(TurnEnvironmentSelection {
            environment_id,
            cwd,
            workspace_roots,
        });
    }
    thread_manager
        .validate_environment_selections(&selections)
        .map_err(environment_selection_error)?;
    Ok(Some(selections))
}

fn resolve_runtime_workspace_roots(workspace_roots: Vec<AbsolutePathBuf>) -> Vec<AbsolutePathBuf> {
    let mut resolved_roots = Vec::new();
    for root in workspace_roots {
        if !resolved_roots.iter().any(|existing| existing == &root) {
            resolved_roots.push(root);
        }
    }
    resolved_roots
}

mod config_errors;
mod request_errors;
mod thread_delete;
mod thread_goal_processor;
mod thread_lifecycle;
mod thread_resume_redaction;
mod thread_summary;

use self::config_errors::*;
use self::request_errors::*;
use self::thread_goal_processor::api_thread_goal_from_state;
use self::thread_lifecycle::*;
use self::thread_resume_redaction::*;
use self::thread_summary::*;

pub(crate) use self::thread_lifecycle::populate_thread_turns_from_history;
pub(crate) use self::thread_processor::thread_from_stored_thread;
#[cfg(test)]
pub(crate) use self::thread_summary::read_summary_from_rollout;
#[cfg(test)]
pub(crate) use self::thread_summary::summary_to_thread;
pub(crate) use self::thread_summary::thread_settings_from_config_snapshot;
pub(crate) use self::thread_summary::thread_settings_from_core_snapshot;

pub(crate) fn build_legacy_api_turns_from_rollout_items(items: &[RolloutItem]) -> Vec<Turn> {
    let mut builder = ThreadHistoryBuilder::new();
    for item in items {
        if is_persisted_rollout_item(item, forestx_protocol::protocol::ThreadHistoryMode::Legacy) {
            builder.handle_rollout_item(item);
        }
    }
    builder.finish()
}
