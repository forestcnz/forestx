const PERSONAL_ACCESS_TOKEN_PREFIX: &str = "at-";

pub(super) enum ForestxAccessToken<'a> {
    PersonalAccessToken(&'a str),
    AgentIdentityJwt(&'a str),
}

pub(super) fn classify_forestx_access_token(access_token: &str) -> ForestxAccessToken<'_> {
    if access_token.starts_with(PERSONAL_ACCESS_TOKEN_PREFIX) {
        ForestxAccessToken::PersonalAccessToken(access_token)
    } else {
        ForestxAccessToken::AgentIdentityJwt(access_token)
    }
}

#[cfg(test)]
#[path = "access_token_tests.rs"]
mod tests;
