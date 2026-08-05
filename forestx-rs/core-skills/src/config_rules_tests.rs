use forestx_config::CONFIG_TOML_FILE;
use forestx_config::ConfigLayerEntry;
use forestx_config::ConfigLayerSource;
use forestx_config::ConfigLayerStack;
use forestx_config::ConfigRequirementsToml;
use forestx_utils_absolute_path::AbsolutePathBuf;
use forestx_utils_absolute_path::test_support::PathBufExt;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::SkillConfigRule;
use super::SkillConfigRuleSelector;
use super::SkillConfigRules;
use super::skill_config_rules_from_stack;

fn user_layer(forestx_home: &TempDir, config: &str) -> ConfigLayerEntry {
    let config_path = AbsolutePathBuf::try_from(forestx_home.path().join(CONFIG_TOML_FILE))
        .expect("absolute config path");
    ConfigLayerEntry::new(
        ConfigLayerSource::User {
            file: config_path,
            profile: None,
        },
        toml::from_str(config).expect("valid user config"),
    )
}

fn stack(forestx_home: &TempDir, user: &str, session: &str) -> ConfigLayerStack {
    ConfigLayerStack::new(
        vec![
            user_layer(forestx_home, user),
            ConfigLayerEntry::new(
                ConfigLayerSource::SessionFlags,
                toml::from_str(session).expect("valid session config"),
            ),
        ],
        Default::default(),
        ConfigRequirementsToml::default(),
    )
    .expect("valid config stack")
}

fn path_toggle_config(path: &std::path::Path, enabled: bool) -> String {
    format!(
        r#"[[skills.config]]
path = "{}"
enabled = {enabled}
"#,
        path.display()
    )
}

#[cfg_attr(windows, ignore)]
#[test]
fn session_flags_can_reenable_user_disabled_path() {
    let forestx_home = TempDir::new().expect("temp dir");
    let skill_path = forestx_home.path().join("skills/demo/SKILL.md");

    assert_eq!(
        skill_config_rules_from_stack(&stack(
            &forestx_home,
            &path_toggle_config(&skill_path, /*enabled*/ false),
            &path_toggle_config(&skill_path, /*enabled*/ true),
        )),
        SkillConfigRules {
            entries: vec![SkillConfigRule {
                selector: SkillConfigRuleSelector::Path(skill_path.abs()),
                enabled: true,
            }],
        }
    );
}

#[cfg_attr(windows, ignore)]
#[test]
fn session_flags_can_disable_user_enabled_path() {
    let forestx_home = TempDir::new().expect("temp dir");
    let skill_path = forestx_home.path().join("skills/demo/SKILL.md");

    assert_eq!(
        skill_config_rules_from_stack(&stack(
            &forestx_home,
            &path_toggle_config(&skill_path, /*enabled*/ true),
            &path_toggle_config(&skill_path, /*enabled*/ false),
        )),
        SkillConfigRules {
            entries: vec![SkillConfigRule {
                selector: SkillConfigRuleSelector::Path(skill_path.abs()),
                enabled: false,
            }],
        }
    );
}

#[test]
fn preserves_name_selectors() {
    let forestx_home = TempDir::new().expect("temp dir");

    assert_eq!(
        skill_config_rules_from_stack(&stack(
            &forestx_home,
            r#"
[[skills.config]]
name = "github:yeet"
enabled = false
"#,
            "",
        )),
        SkillConfigRules {
            entries: vec![SkillConfigRule {
                selector: SkillConfigRuleSelector::Name("github:yeet".to_string()),
                enabled: false,
            }],
        }
    );
}

#[cfg_attr(windows, ignore)]
#[test]
fn preserves_order_across_path_and_name_selectors() {
    let forestx_home = TempDir::new().expect("temp dir");
    let skill_path = forestx_home.path().join("skills/demo/SKILL.md");

    assert_eq!(
        skill_config_rules_from_stack(&stack(
            &forestx_home,
            &path_toggle_config(&skill_path, /*enabled*/ false),
            r#"
[[skills.config]]
name = "github:yeet"
enabled = true
"#,
        )),
        SkillConfigRules {
            entries: vec![
                SkillConfigRule {
                    selector: SkillConfigRuleSelector::Path(skill_path.abs()),
                    enabled: false,
                },
                SkillConfigRule {
                    selector: SkillConfigRuleSelector::Name("github:yeet".to_string()),
                    enabled: true,
                },
            ],
        }
    );
}
