use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub fn whoami(command_interaction: &mut ApplicationCommandInteraction) -> String {
    let user_parent = command_interaction.member.as_mut().unwrap();
    format!(
        "Your name: {}#{}\nYour user ID: {}\n",
        user_parent.user.name, user_parent.user.discriminator, user_parent.user.id
    )
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("whoami")
        .description("See some info about you.")
}
