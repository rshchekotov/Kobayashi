use pest::{Parser, iterators::Pair};
use std::{str::FromStr, assert_matches::assert_matches};
use super::{MochiParser, Rule, discord::Permission};
use crate::error::PropertyTypeError;

fn test_prop<F>(key: &str, val: &str, val_type: Rule, additonal_val_tests: F)
where F: FnOnce(Pair<'_, Rule>) -> () {
    let prop_input = format!("{} -> {}", key, val);
    let result = MochiParser::parse(Rule::property, &prop_input);
    assert_eq!(result.is_ok(), true);
    let mut result = result.unwrap();
    let prop = result.next().unwrap();
    assert_eq!(prop.as_rule(), Rule::property);
    let mut prop = prop.into_inner();
    let prop_name = prop.next().unwrap();
    assert_eq!(prop_name.as_rule(), Rule::identifier);
    assert_eq!(prop_name.as_str(), key);
    let prop_value = prop.next().unwrap();
    assert_eq!(prop_value.as_rule(), val_type);
    assert_eq!(prop_value.as_str(), val);
    additonal_val_tests(prop_value);
    assert_eq!(prop.next(), None);
}

#[test]
fn verify_property() {
    test_prop("property", "\"value\"", Rule::string, |_| ());
    test_prop("property", "true", Rule::bool, |_| ());
    test_prop("property", "#960e29", Rule::color, |_| ());
    test_prop("property", "123", Rule::integer, |_| ());
    test_prop("property", "thor", Rule::identifier, |_| ());
    // Test Array
    test_prop("property", "[\"value1\", \"value2\"]", Rule::array, |array| {
        let mut index = 1;
        for value in array.into_inner() {
            assert_eq!(value.as_rule(), Rule::string);
            assert_eq!(value.as_str(), format!("\"value{}\"", index));
            index += 1;
        }
    });
}

#[test]
fn verify_environment() {
    // ? Test: Verify Simple Environment generically, without env-specific checks
    let env_input = "channel \"general\" {
        type -> \"Text\"
    }";
    let result = MochiParser::parse(Rule::environment, env_input);
    assert_eq!(result.is_ok(), true);
    let mut result = result.unwrap();
    let env = result.next().unwrap();
    assert_eq!(env.as_rule(), Rule::environment);
    let mut env = env.into_inner();
    let env_type = env.next().unwrap();
    assert_eq!(env_type.as_rule(), Rule::env_type);
    assert_eq!(env_type.as_str(), "channel");
    let env_name = env.next().unwrap();
    assert_eq!(env_name.as_rule(), Rule::string);
    assert_eq!(env_name.as_str(), "\"general\"");
    let property = env.next().unwrap();
    assert_eq!(property.as_rule(), Rule::property);
    let mut property = property.into_inner();
    let prop_name = property.next().unwrap();
    assert_eq!(prop_name.as_rule(), Rule::identifier);
    assert_eq!(prop_name.as_str(), "type");
    let prop_value = property.next().unwrap();
    assert_eq!(prop_value.as_rule(), Rule::string);
    assert_eq!(prop_value.as_str(), "\"Text\"");
    assert_eq!(property.next(), None);

    // ? Test: Verify Environment with multiple properties
    let env_input = "channel \"general\" {
        type -> \"Text\"
        topic -> \"General chat\"
    }";
    let result = MochiParser::parse(Rule::environment, env_input);
    assert_eq!(result.is_ok(), true);
    let mut result = result.unwrap();
    let env = result.next().unwrap();
    assert_eq!(env.as_rule(), Rule::environment);
    let mut env = env.into_inner();
    let env_type = env.next().unwrap();
    assert_eq!(env_type.as_rule(), Rule::env_type);
    assert_eq!(env_type.as_str(), "channel");
    let env_name = env.next().unwrap();
    assert_eq!(env_name.as_rule(), Rule::string);
    assert_eq!(env_name.as_str(), "\"general\"");
    let mut index = 0;
    for property in env {
        assert_eq!(property.as_rule(), Rule::property);
        let mut property = property.into_inner();
        let prop_name = property.next().unwrap();
        assert_eq!(prop_name.as_rule(), Rule::identifier);
        let prop_value = property.next().unwrap();
        assert_eq!(prop_value.as_rule(), Rule::string);
        match index {
            0 => {
                assert_eq!(prop_name.as_str(), "type");
                assert_eq!(prop_value.as_str(), "\"Text\"");
            },
            1 => {
                assert_eq!(prop_name.as_str(), "topic");
                assert_eq!(prop_value.as_str(), "\"General chat\"");
            },
            _ => panic!("Unexpected property found in environment!"),
        }
        index += 1;
    }
    assert_eq!(index, 2);

    // ? Test: Verify Nested Environment
    let env_input = "category \"General\" {
        permission -> \"@everyone\"
        channel \"general\" {
            type -> \"Text\"
            topic -> \"General chat\"
        }
    }";
    let result = MochiParser::parse(Rule::environment, env_input);
    assert_eq!(result.is_ok(), true);
    let mut result = result.unwrap();
    let env = result.next().unwrap();
    assert_eq!(env.as_rule(), Rule::environment);
    let mut env = env.into_inner();
    let env_type = env.next().unwrap();
    assert_eq!(env_type.as_rule(), Rule::env_type);
    assert_eq!(env_type.as_str(), "category");
    let env_name = env.next().unwrap();
    assert_eq!(env_name.as_rule(), Rule::string);
    assert_eq!(env_name.as_str(), "\"General\"");
    let permission_prop = env.next().unwrap();
    assert_eq!(permission_prop.as_rule(), Rule::property);
    let mut permission_prop = permission_prop.into_inner();
    let prop_name = permission_prop.next().unwrap();
    assert_eq!(prop_name.as_rule(), Rule::identifier);
    assert_eq!(prop_name.as_str(), "permission");
    let prop_value = permission_prop.next().unwrap();
    assert_eq!(prop_value.as_rule(), Rule::string);
    assert_eq!(prop_value.as_str(), "\"@everyone\"");
    let channel_env = env.next().unwrap();
    assert_eq!(channel_env.as_rule(), Rule::environment);
    let mut channel_env = channel_env.into_inner();
    let channel_env_type = channel_env.next().unwrap();
    assert_eq!(channel_env_type.as_rule(), Rule::env_type);
    assert_eq!(channel_env_type.as_str(), "channel");
    let channel_env_name = channel_env.next().unwrap();
    assert_eq!(channel_env_name.as_rule(), Rule::string);
    assert_eq!(channel_env_name.as_str(), "\"general\"");
    let mut index = 0;
    for property in channel_env {
        assert_eq!(property.as_rule(), Rule::property);
        let mut property = property.into_inner();
        let prop_name = property.next().unwrap();
        assert_eq!(prop_name.as_rule(), Rule::identifier);
        let prop_value = property.next().unwrap();
        assert_eq!(prop_value.as_rule(), Rule::string);
        match index {
            0 => {
                assert_eq!(prop_name.as_str(), "type");
                assert_eq!(prop_value.as_str(), "\"Text\"");
            },
            1 => {
                assert_eq!(prop_name.as_str(), "topic");
                assert_eq!(prop_value.as_str(), "\"General chat\"");
            },
            _ => panic!("Unexpected property found in environment!"),
        }
        index += 1;
    }
}

#[test]
fn verify_permission_enum() {
    // ? Test: Verify Permissions Enum (from str)
    let first_str = "create_instant_invite";
    let first = super::discord::Permission::from_str(first_str);
    assert_eq!(first.is_ok(), true);
    let permission = first.unwrap();
    assert_eq!(permission, super::discord::Permission::CreateInstantInvite);
    assert_eq!(permission as u64, 0);
    let perm_pow: u64 = permission.into();
    assert_eq!(perm_pow, 1);

    // ? Test: Verify Permissions Enum (from u64)
    let first_u64 = 1;
    let permission = super::discord::Permission::try_from(first_u64);
    assert_eq!(permission.is_ok(), true);
    let permission = permission.unwrap();
    assert_eq!(permission, super::discord::Permission::CreateInstantInvite);
    assert_eq!(permission as u64, 0);
    let perm_pow: u64 = permission.into();
    assert_eq!(perm_pow, first_u64);

    // ? Test: Verify Permissions Enum (from str)
    let last_str = "send_voice_messages";
    let last = super::discord::Permission::from_str(last_str);
    assert_eq!(last.is_ok(), true);
    let permission = last.unwrap();
    assert_eq!(permission, super::discord::Permission::SendVoiceMessages);
    assert_eq!(permission as u64, 46);
    let perm_pow: u64 = permission.into();
    assert_eq!(perm_pow, 1 << 46);

    // ? Test: Verify Permissions Enum (from u64)
    let last_u64 = 1 << 46;
    let permission = super::discord::Permission::try_from(last_u64);
    assert_eq!(permission.is_ok(), true);
    let permission = permission.unwrap();
    assert_eq!(permission, super::discord::Permission::SendVoiceMessages);
    assert_eq!(permission as u64, 46);
    let perm_pow: u64 = permission.into();
    assert_eq!(perm_pow, last_u64);

    // ? Test: Verify Permissions from Vec<&str>
    let perms = vec!["create_instant_invite", "send_voice_messages"];
    let permissions = super::discord::parse_permissions_from_vec(perms);
    assert_eq!(permissions, 1 | (1 << 46));
}

#[test]
fn verify_role_struct() {
    // ? Test: Verify Role Struct (minimal)
    let minimal_role_input = "role \"Test Role\" {}";
    let role = crate::Role::from_str(minimal_role_input);
    assert_eq!(role.is_ok(), true);
    let role = role.unwrap();
    assert_eq!(role.name, "Test Role");
    assert_eq!(role.color, 0);
    assert_eq!(role.separate, false);
    assert_eq!(role.mentionable, false);
    assert_eq!(role.permissions, 0);

    // ? Test: Verify Role Struct (with color)
    let color_role_input = "role \"Crimson\" {
        color -> #960e29
    }";
    let role = crate::Role::from_str(color_role_input);
    assert_eq!(role.is_ok(), true);
    let role = role.unwrap();
    assert_eq!(role.name, "Crimson");
    assert_eq!(role.color, 0x960e29);
    assert_eq!(role.separate, false);
    assert_eq!(role.mentionable, false);
    assert_eq!(role.permissions, 0);

    // ? Test: Verify Role Struct (with invalid color)
    let invalid_color_role_input = "role \"Crimson\" {
        color -> \"crimson\"
    }";
    let role = crate::Role::from_str(invalid_color_role_input);
    assert_eq!(role.is_err(), true);
    assert_matches!(role.unwrap_err().downcast_ref::<PropertyTypeError>(), Some(_));
    

    // ? Test: Verify Role Struct (with everything except Permissions)
    let most_role_input = "role \"Staff\" {
        color -> #006ad1
        separate -> true
        mentionable -> true
    }";
    let role = crate::Role::from_str(most_role_input);
    assert_eq!(role.is_ok(), true);
    let role = role.unwrap();
    assert_eq!(role.name, "Staff");
    assert_eq!(role.color, 0x006ad1);
    assert_eq!(role.separate, true);
    assert_eq!(role.mentionable, true);
    assert_eq!(role.permissions, 0);

    // ? Test: Verify Role Struct (with everything)
    let full_role_input = "role \"Moderator\" {
        color -> #006ad1
        separate -> true
        mentionable -> true
        permissions -> [
            VIEW_CHANNEL, MANAGE_ROLES, VIEW_AUDIT_LOG, CREATE_INSTANT_INVITE, CHANGE_NICKNAME, 
            MODERATE_MEMBERS, SEND_MESSAGES, EMBED_LINKS, ATTACH_FILES, ADD_REACTIONS,
            USE_EXTERNAL_EMOJIS, MANAGE_MESSAGES, READ_MESSAGE_HISTORY, USE_APPLICATION_COMMANDS,
            CONNECT, SPEAK, STREAM, USE_VAD, MUTE_MEMBERS, MOVE_MEMBERS, REQUEST_TO_SPEAK
        ]
    }";
    let role = crate::Role::from_str(full_role_input);
    assert_eq!(role.is_ok(), true);
    let role = role.unwrap();
    assert_eq!(role.name, "Moderator");
    assert_eq!(role.color, 0x006ad1);
    assert_eq!(role.separate, true);
    assert_eq!(role.mentionable, true);
    let perms = vec![
        Permission::ViewChannel, Permission::ManageRoles, Permission::ViewAuditLog, Permission::CreateInstantInvite,
        Permission::ChangeNickname, Permission::ModerateMembers, Permission::SendMessages, Permission::EmbedLinks,
        Permission::AttachFiles, Permission::AddReactions, Permission::UseExternalEmojis, Permission::ManageMessages,
        Permission::ReadMessageHistory, Permission::UseApplicationCommands, Permission::Connect, Permission::Speak,
        Permission::Stream, Permission::UseVad, Permission::MuteMembers, Permission::MoveMembers, Permission::RequestToSpeak,
    ];
    for perm in perms {
        assert_eq!(role.permissions[perm], Ok(()));
    }
    assert_eq!(role.permissions[Permission::ViewGuildInsights], Err(()));
    assert_eq!(1106347683521, role.permissions);
}

#[test]
fn verify_channel_struct() {
}

#[test]
fn verify_category_struct() {
}