mod commands;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(mut command) = interaction {

            let content = match command.data.name.as_str() {
                "help" => commands::help::help(),
                "whoami" => commands::whoami::whoami(&mut command),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(content))
            })
            .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(356901968653844484);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
            .create_application_command(|command| commands::help::register(command))
            .create_application_command(|command| commands::whoami::register(command))
        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", commands);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    

    // Build our client.
    let mut client = Client::builder("MTA0OTU1MTg0Mzk2MjkyMDk4Mg.Gsa8V6.bLfBk1T8k9QFtZU4d6w_XcAmJYFMEs5IIwjWXw", GatewayIntents::empty())
    .event_handler(Handler)
    .await
    .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}