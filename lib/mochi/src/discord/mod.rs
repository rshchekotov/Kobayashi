use std::{str::FromStr, ops::Index};
use crate::error::{InvalidPermissionIDError, InvalidPermissionNameError};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Permission {
    CreateInstantInvite, KickMembers, BanMembers, Administrator, ManageChannels,
    ManageGuild, AddReactions, ViewAuditLog, PrioritySpeaker, Stream, ViewChannel,
    SendMessages, SendTtsMessages, ManageMessages, EmbedLinks, AttachFiles,
    ReadMessageHistory, MentionEveryone, UseExternalEmojis, ViewGuildInsights,
    Connect, Speak, MuteMembers, DeafenMembers, MoveMembers, UseVad, ChangeNickname,
    ManageNicknames, ManageRoles, ManageWebhooks, ManageGuildExpressions,
    UseApplicationCommands, RequestToSpeak, ManageEvents, ManageThreads,
    CreatePublicThreads, CreatePrivateThreads, UseExternalStickers,
    SendMessagesInThreads, UseEmbeddedActivities, ModerateMembers,
    ViewCreatorMonetizationAnalytics, UseSoundboard, CreateGuildExpressions,
    CreateEvents, UseExternalSounds, SendVoiceMessages,
}

impl FromStr for Permission {
    type Err = InvalidPermissionNameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "create_instant_invite" => Ok(Permission::CreateInstantInvite),
            "kick_members" => Ok(Permission::KickMembers),
            "ban_members" => Ok(Permission::BanMembers),
            "administrator" => Ok(Permission::Administrator),
            "manage_channels" => Ok(Permission::ManageChannels),
            "manage_guild" => Ok(Permission::ManageGuild),
            "add_reactions" => Ok(Permission::AddReactions),
            "view_audit_log" => Ok(Permission::ViewAuditLog),
            "priority_speaker" => Ok(Permission::PrioritySpeaker),
            "stream" => Ok(Permission::Stream),
            "view_channel" => Ok(Permission::ViewChannel),
            "send_messages" => Ok(Permission::SendMessages),
            "send_tts_messages" => Ok(Permission::SendTtsMessages),
            "manage_messages" => Ok(Permission::ManageMessages),
            "embed_links" => Ok(Permission::EmbedLinks),
            "attach_files" => Ok(Permission::AttachFiles),
            "read_message_history" => Ok(Permission::ReadMessageHistory),
            "mention_everyone" => Ok(Permission::MentionEveryone),
            "use_external_emojis" => Ok(Permission::UseExternalEmojis),
            "view_guild_insights" => Ok(Permission::ViewGuildInsights),
            "connect" => Ok(Permission::Connect),
            "speak" => Ok(Permission::Speak),
            "mute_members" => Ok(Permission::MuteMembers),
            "deafen_members" => Ok(Permission::DeafenMembers),
            "move_members" => Ok(Permission::MoveMembers),
            "use_vad" => Ok(Permission::UseVad),
            "change_nickname" => Ok(Permission::ChangeNickname),
            "manage_nicknames" => Ok(Permission::ManageNicknames),
            "manage_roles" => Ok(Permission::ManageRoles),
            "manage_webhooks" => Ok(Permission::ManageWebhooks),
            "manage_guild_expressions" => Ok(Permission::ManageGuildExpressions),
            "use_application_commands" => Ok(Permission::UseApplicationCommands),
            "request_to_speak" => Ok(Permission::RequestToSpeak),
            "manage_events" => Ok(Permission::ManageEvents),
            "manage_threads" => Ok(Permission::ManageThreads),
            "create_public_threads" => Ok(Permission::CreatePublicThreads),
            "create_private_threads" => Ok(Permission::CreatePrivateThreads),
            "use_external_stickers" => Ok(Permission::UseExternalStickers),
            "send_messages_in_threads" => Ok(Permission::SendMessagesInThreads),
            "use_embedded_activities" => Ok(Permission::UseEmbeddedActivities),
            "moderate_members" => Ok(Permission::ModerateMembers),
            "view_creator_monetization_analytics" => Ok(Permission::ViewCreatorMonetizationAnalytics),
            "use_soundboard" => Ok(Permission::UseSoundboard),
            "create_guild_expressions" => Ok(Permission::CreateGuildExpressions),
            "create_events" => Ok(Permission::CreateEvents),
            "use_external_sounds" => Ok(Permission::UseExternalSounds),
            "send_voice_messages" => Ok(Permission::SendVoiceMessages),
            _ => Err(InvalidPermissionNameError { name: s.to_string() }),
        }
    }
}

impl Into<u64> for Permission {
    fn into(self) -> u64 {
        return 1 << self as u64;
    }
}

impl TryFrom<u64> for Permission {
    type Error = InvalidPermissionIDError;
    fn try_from(value: u64) -> Result<Permission, Self::Error> {
        match value {
            1 => Ok(Permission::CreateInstantInvite),
            2 => Ok(Permission::KickMembers),
            4 => Ok(Permission::BanMembers),
            8 => Ok(Permission::Administrator),
            16 => Ok(Permission::ManageChannels),
            32 => Ok(Permission::ManageGuild),
            64 => Ok(Permission::AddReactions),
            128 => Ok(Permission::ViewAuditLog),
            256 => Ok(Permission::PrioritySpeaker),
            512 => Ok(Permission::Stream),
            1024 => Ok(Permission::ViewChannel),
            2048 => Ok(Permission::SendMessages),
            4096 => Ok(Permission::SendTtsMessages),
            8192 => Ok(Permission::ManageMessages),
            16384 => Ok(Permission::EmbedLinks),
            32768 => Ok(Permission::AttachFiles),
            65536 => Ok(Permission::ReadMessageHistory),
            131072 => Ok(Permission::MentionEveryone),
            262144 => Ok(Permission::UseExternalEmojis),
            524288 => Ok(Permission::ViewGuildInsights),
            1048576 => Ok(Permission::Connect),
            2097152 => Ok(Permission::Speak),
            4194304 => Ok(Permission::MuteMembers),
            8388608 => Ok(Permission::DeafenMembers),
            16777216 => Ok(Permission::MoveMembers),
            33554432 => Ok(Permission::UseVad),
            67108864 => Ok(Permission::ChangeNickname),
            134217728 => Ok(Permission::ManageNicknames),
            268435456 => Ok(Permission::ManageRoles),
            536870912 => Ok(Permission::ManageWebhooks),
            1073741824 => Ok(Permission::ManageGuildExpressions),
            2147483648 => Ok(Permission::UseApplicationCommands),
            4294967296 => Ok(Permission::RequestToSpeak),
            8589934592 => Ok(Permission::ManageEvents),
            17179869184 => Ok(Permission::ManageThreads),
            34359738368 => Ok(Permission::CreatePublicThreads),
            68719476736 => Ok(Permission::CreatePrivateThreads),
            137438953472 => Ok(Permission::UseExternalStickers),
            274877906944 => Ok(Permission::SendMessagesInThreads),
            549755813888 => Ok(Permission::UseEmbeddedActivities),
            1099511627776 => Ok(Permission::ModerateMembers),
            2199023255552 => Ok(Permission::ViewCreatorMonetizationAnalytics),
            4398046511104 => Ok(Permission::UseSoundboard),
            8796093022208 => Ok(Permission::CreateGuildExpressions),
            17592186044416 => Ok(Permission::CreateEvents),
            35184372088832 => Ok(Permission::UseExternalSounds),
            70368744177664 => Ok(Permission::SendVoiceMessages),
            _ => Err(InvalidPermissionIDError { id: value }),
        }
    }
}

impl Index<Permission> for Permissions {
    type Output = Result<(),()>;
    fn index(&self, index: Permission) -> &Self::Output {
        return if (self & (1 << index as u64)) != 0 {
            &Ok(())
        } else {
            &Err(())
        };
    }
}

pub type Permissions = u64;

pub fn parse_permissions_from_vec(permissions: Vec<&str>) -> Permissions {
    let mut perms: Permissions = 0;
    for permission in permissions {
        let perm = Permission::from_str(permission);
        if perm.is_ok() {
            let perm_val: u64 = perm.unwrap().into();
            perms |= perm_val;
        }
    }
    return perms;
}

pub fn parse_permissions_from_perm_vec(permissions: Vec<Permission>) -> Permissions {
    let mut perms: Permissions = 0;
    for permission in permissions {
        let perm_val: u64 = permission.into();
        perms |= perm_val;
            
    }
    return perms;
}

pub fn separate_permissions(permissions: Permissions) -> Vec<Permission> {
    let mut perms: Vec<Permission> = Vec::new();
    let mut perm_pow: u64 = 1;
    while perm_pow <= permissions {
        if (permissions & perm_pow) != 0 {
            let potential_perm = Permission::try_from(perm_pow);
            if potential_perm.is_ok() {
                perms.push(potential_perm.unwrap());
            }
        }
        perm_pow <<= 1;
    }
    return perms;
}