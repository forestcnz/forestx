use super::*;
use forestx_protocol::error::ForestxErrorDetails;

pub(super) fn environment_selection_error(err: ForestxErr) -> JSONRPCErrorError {
    match err.details() {
        ForestxErrorDetails::InvalidRequest(message) => invalid_request(message.clone()),
        _ => internal_error(format!("failed to validate environment selections: {err}")),
    }
}
