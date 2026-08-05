use forestx_analytics::CompactionImplementation;
use forestx_analytics::CompactionReason;
use forestx_otel::SessionTelemetry;
use forestx_protocol::error::ForestxErr;
use forestx_protocol::error::ForestxErrorDetails;
use tracing::warn;

/// Retries failures that may be model-specific and succeed with a different model.
pub(crate) fn should_retry_with_current_model(error: &ForestxErr) -> bool {
    matches!(
        error.details(),
        ForestxErrorDetails::InvalidRequest(_)
            | ForestxErrorDetails::UnexpectedStatus(_)
            | ForestxErrorDetails::ContextWindowExceeded
            | ForestxErrorDetails::UsageLimitReached(_)
            | ForestxErrorDetails::ServerOverloaded
            | ForestxErrorDetails::InternalServerError
            | ForestxErrorDetails::RetryLimit(_)
    )
}

pub(crate) fn record_model_fallback(
    session_telemetry: &SessionTelemetry,
    previous_model: &str,
    current_model: &str,
    reason: CompactionReason,
    implementation: CompactionImplementation,
    fallback_error: Option<&ForestxErr>,
) {
    let reason_tag = match reason {
        CompactionReason::UserRequested => "user_requested",
        CompactionReason::ContextLimit => "context_limit",
        CompactionReason::ModelDownshift => "model_downshift",
        CompactionReason::CompHashChanged => "comp_hash_changed",
    };
    let implementation_tag = match implementation {
        CompactionImplementation::Responses => "responses",
        CompactionImplementation::ResponsesCompactionV2 => "responses_compaction_v2",
        CompactionImplementation::ResponsesCompact => "responses_compact",
    };
    let outcome = if fallback_error.is_none() {
        "succeeded"
    } else {
        "failed"
    };
    session_telemetry.counter(
        "forestx.compaction.model_fallback",
        /*inc*/ 1,
        &[
            ("reason", reason_tag),
            ("implementation", implementation_tag),
            ("outcome", outcome),
        ],
    );
    warn!(
        previous_model,
        current_model,
        ?reason,
        ?implementation,
        outcome,
        ?fallback_error,
        "previous-model compaction failed; retried with current model"
    );
}
