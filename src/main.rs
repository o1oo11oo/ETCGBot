// Adding the allow lint on the actual enum variant messes with the derive macro
#![allow(clippy::upper_case_acronyms)]

use std::error::Error;

use teloxide::{
    prelude2::*,
    types::{MessageKind, ParseMode},
    utils::{command::BotCommand, markdown::escape},
};

#[derive(BotCommand, Clone)]
#[command(description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.", rename = "lowercase")]
    Help,
    #[command(description = "apply for ETCG, totally legit.")]
    ETCG,
    #[command(description = "wish everyone farewell.", rename = "lowercase")]
    Goodbye,
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // try to find out who sent this message, feels like there's a better way to do this
    let sender = if let MessageKind::Common(common) = &message.kind {
        if let Some(user) = &common.from {
            if let Some(last_name) = &user.last_name {
                format!("{} {last_name}", user.first_name)
            } else {
                user.first_name.clone()
            }
        } else {
            "someone".to_owned()
        }
    } else {
        "someone".to_owned()
    };

    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions())
                .await?;
        }

        // the main command for this bot
        Command::ETCG => {
            log::info!("ETCG message received: {message:?}");

            let text = format!(
                "*__Congratulations\\!__*\n\
                \n\
                You have successfully used the ETCGBot to apply for the *ESTIEM Trading Card Game*\\. \
                Feel free to bother @erikviktor at CM if he has no cards for you, \
                but make sure to pay a lot of money to his PayPal account first \
                \\(unless you're turkish, swedish, finnish or serbian\\)\\.\n\
                \n\
                _This message was sent after actions made by {}\\. If you receive it by a mistake, \
                please submit a pull request [on GitHub](https://github.com/o1oo11oo/ETCGBot)\\._",
                escape(&sender)
            );

            bot.parse_mode(ParseMode::MarkdownV2)
                .send_message(message.chat.id, text)
                .await?;
        }

        // say goodbye!
        Command::Goodbye => {
            log::info!("GoodBye message received: {message:?}");

            // craft goodbye message
            let text = format!(
                "This joke has probably run its course\\.\n\
                \n\
                Thank you everyone for using the ETCGBot, \
                have fun with the *ESTIEM Trading Card Game*, \
                have a safe trip to Belgrade and see you there\\!\n\
                \n\
                _This message was sent after actions made by {}\\. If you receive it by a mistake, \
                please submit a pull request [on GitHub](https://github.com/o1oo11oo/ETCGBot)\\._",
                escape(&sender)
            );

            // send the message
            bot.clone()
                .parse_mode(ParseMode::MarkdownV2)
                .send_message(message.chat.id, text)
                .await?;

            // leave the group
            bot.leave_chat(message.chat.id).await?;
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();
    log::info!("Starting ETCGBot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
}
