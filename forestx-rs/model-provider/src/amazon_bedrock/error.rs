use forestx_api::ApiError;
use forestx_protocol::error::ForestxErr;
use forestx_protocol::error::ForestxErrorDetails;
use http::StatusCode;

pub(super) const BEDROCK_EXPIRED_SIGNATURE_MESSAGE: &str = concat!(
    "Amazon Bedrock rejected the request because its AWS signature has expired. ",
    "Refresh your AWS credentials and retry. If `AWS_BEARER_TOKEN_BEDROCK` is set, ",
    "update or unset it, then restart Forestx",
);

pub(super) fn map_api_error(error: ApiError) -> ForestxErr {
    let error = forestx_api::map_api_error(error);
    if let ForestxErrorDetails::UnexpectedStatus(response) = error.details()
        && response.status == StatusCode::UNAUTHORIZED
        && response.body.contains("Signature expired:")
    {
        let mut response = response.clone();
        response.user_message = Some(BEDROCK_EXPIRED_SIGNATURE_MESSAGE.to_string());
        let mapped_error = ForestxErr::new(ForestxErrorDetails::UnexpectedStatus(response));
        return match error.retry_delay() {
            Some(retry_delay) => mapped_error.with_retry_delay(retry_delay),
            None => mapped_error,
        };
    }
    error
}
