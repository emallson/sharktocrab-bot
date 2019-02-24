use serenity::model::prelude::*;
use serenity::http;
use crate::GUILD_ID;

static LFG_ROLE: u64 = 549291995017904129;

fn remove_lfg(user_id: UserId) {
    http::raw::remove_member_role(GUILD_ID, user_id.into(), LFG_ROLE).expect("Unable to remove LFG role");
}

pub(crate) fn update_lfg_status(update: PresenceUpdateEvent) {
    if !update.roles.map(|v| v.contains(&LFG_ROLE.into())).unwrap_or(false) {
        return; // not lfg, do nothing
    }

    match update.presence.status {
        OnlineStatus::Invisible => remove_lfg(update.presence.user_id),
        OnlineStatus::Offline => remove_lfg(update.presence.user_id),
        OnlineStatus::Idle => remove_lfg(update.presence.user_id),
        _ => {},
    }
}

command!(lfg(_ctx, msg) {
    http::raw::add_member_role(GUILD_ID, msg.author.id.into(), LFG_ROLE).expect("Unable to add LFG role");
});

command!(notlfg(_ctx, msg) {
    remove_lfg(msg.author.id.into());
});
