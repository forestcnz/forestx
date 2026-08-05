use super::*;

#[test]
fn classifies_personal_access_tokens_by_prefix() {
    assert!(matches!(
        classify_forestx_access_token("at-example"),
        ForestxAccessToken::PersonalAccessToken("at-example")
    ));
    assert!(matches!(
        classify_forestx_access_token("header.payload.signature"),
        ForestxAccessToken::AgentIdentityJwt("header.payload.signature")
    ));
}
