#[macro_use] extern crate log;
#[macro_use] extern crate serenity;
extern crate env_logger;

use std::{collections::HashSet, env};

use serenity::{
    framework::{StandardFramework, standard::help_commands},
    prelude::*,
    model::prelude::*,
    http,
};

mod commands;

struct Handler;

static ROLE_EMOJI: [u64; 6] = [547550216728084480, 547550216530690107, 547550216514175027, 547550216455323689, 547550216514174987, 547550216593604649];
static ROLE_GAIN_MSG: u64 = 547540401402281992;

static GUILD_ID: u64 = 547528380329754639;
static CHAT_ROLE_ID: u64 = 547548706816131082;
static MM_ROLE_ID: u64 = 547548240116187136;
static MM_BANNED_ROLE_ID: u64 = 547558765214433281;

fn is_role_emoji(emoji: &ReactionType) -> bool{
    match emoji {
        ReactionType::Custom { id, .. } => ROLE_EMOJI.contains(id.as_u64()),
        _ => false
    }
}

impl EventHandler for Handler {
    fn reaction_add(&self, _ctx: Context, reaction: Reaction) {
        if reaction.message_id != ROLE_GAIN_MSG {
            // do nothing
        } else if !is_role_emoji(&reaction.emoji) {
            // wrong kind of emoji
            reaction.delete().expect("Failed to remove emoji from role message.");
        } else if !reaction.user().unwrap().has_role(GUILD_ID, MM_BANNED_ROLE_ID) {
            // user reacted with correct kind of emoji and is allowed to get roles
            http::raw::add_member_role(GUILD_ID, reaction.user_id.into(), CHAT_ROLE_ID).expect("Unable to add chat role");
            http::raw::add_member_role(GUILD_ID, reaction.user_id.into(), MM_ROLE_ID).expect("Unable to add matchmaking role");
        }
    }

    fn presence_update(&self, _ctx: Context, update: PresenceUpdateEvent) {
        commands::lfg::update_lfg_status(update);
    }
}

fn main() {
    env_logger::init();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment.");

    let mut client = Client::new(&token, Handler).expect("Unable to create client");

    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
                          .configure(|c| c.owners(owners).prefix("!"))
                          .help(help_commands::with_embeds)
                          .group("Looking for Games", |g| g.prefix("lfg")
                                 .allowed_roles(&["Post in Matchmaking"])
                                 .default_cmd(commands::lfg::lfg)
                                 .cmd("off", commands::lfg::notlfg)));
    if let Err(why) = client.start() {
        error!("Client error: {}", why);
    }
}
