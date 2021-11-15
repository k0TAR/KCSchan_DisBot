use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("role一覧")]
async fn view_roles(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild_id.unwrap();
    let roles = guild.roles(&ctx.http()).await?;

    for (role_id, role) in roles.iter() {
        let name = if &role.name == "@everyone" {
            "everyone"
        } else {
            &role.name
        };
        msg.channel_id
            .say(
                &ctx.http,
                format!("id: `{}`, role_name: {}", &role_id, name),
            )
            .await?;
    }
    Ok(())
}

#[command]
// #[num_args(1)]
#[description("role作成")]
async fn create_role(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        &msg.channel_id
            .say(&ctx.http, format!("引数が1つ必要です",))
            .await;
    }
    let role_names = args
        .iter::<String>()
        .map(|s| s.unwrap_or("".to_string()))
        .collect::<Vec<_>>();

    let guild_id = msg.guild_id.unwrap();
    for role_name in role_names {
        match guild_id.create_role(&ctx.http, |r| r.name(role_name)).await {
            Ok(role) => {
                &msg.channel_id
                    .say(&ctx.http, format!("{}", role.name))
                    .await;
            }
            Err(error) => {
                &msg.channel_id
                    .say(
                        &ctx.http,
                        format! {"Error: {}\nロールを作成することができませんでした。", error},
                    )
                    .await;
            }
        }
    }
    Ok(())
}