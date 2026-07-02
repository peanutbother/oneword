use crate::{
    database, defer_ephemeral, require_admin,
    util::{check_permissions, guild_id, guild_safe, Context, Error},
};
use entity::sea_orm::ActiveModelTrait;
use poise::serenity_prelude::{
    ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, EditInteractionResponse, Message,
};
use std::time::Duration;

/// Configure Summary Footer
#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    category = "setup",
    rename = "footer",
    // required_permissions = "ADMINISTRATOR"
)]
pub async fn command(ctx: Context<'_>) -> Result<(), Error> {
    require_admin!(ctx);
    defer_ephemeral!(ctx);

    let db = database!(ctx);
    let guild_id = guild_id(&ctx);
    let guild = guild_safe(db, guild_id).await?;
    let uuid = ctx.id();

    let embed = create_embed(&guild);
    let components = create_components(&guild, uuid);
    let message = ctx
        .interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new()
                .embed(embed.clone())
                .components(components.clone()),
        )
        .await?;

    await_interaction(ctx, guild_id, guild, uuid, message).await?;

    Ok(())
}

fn create_embed(guild: &entity::guild::Model) -> CreateEmbed {
    CreateEmbed::new()
        .title("Configure Endgame Footer")
        // .description(
        //     "This allows you to customize the embed footer of when a game ends.".to_owned(),
        // )
        .description("Click on one of the buttons to change the settings.")
        .fields(vec![
            (
                "Display users in the Footer",
                if guild.hide_user { "❎ " } else { "✅" },
                false,
            ),
            (
                "Display if original messages will be deleted",
                if guild.hide_deletion_info {
                    "❎"
                } else {
                    "✅"
                },
                false,
            ),
        ])
}

fn create_components(guild: &entity::guild::Model, uuid: u64) -> Vec<CreateActionRow> {
    vec![CreateActionRow::Buttons(vec![
        CreateButton::new(format!("toggle_user_{uuid}"))
            .style(ButtonStyle::Primary)
            .label(if !&guild.hide_user {
                "Hide User"
            } else {
                "Show User"
            }),
        CreateButton::new(format!("toggle_delete_{uuid}"))
            .style(ButtonStyle::Primary)
            .label(if !&guild.hide_deletion_info {
                "Hide Deletion Info"
            } else {
                "Show Deletion Info"
            }),
    ])]
}

async fn await_interaction<'a>(
    ctx: Context<'a>,
    guild_id: u64,
    guild: entity::guild::Model,
    uuid: u64,
    message: Message,
) -> Result<(), Error> {
    let db = database!(ctx);
    let mut guild = guild;

    while let Some(mci) = message
        .await_component_interactions(ctx.serenity_context)
        .timeout(Duration::from_secs(120))
        .filter(move |i| {
            i.data.custom_id == format!("toggle_user_{uuid}")
                || i.data.custom_id == format!("toggle_delete_{uuid}")
        })
        .await
    {
        mci.defer(ctx.serenity_context).await?;

        if mci.data.custom_id == format!("toggle_user_{uuid}") {
            guild.hide_user = !guild.hide_user;
        } else if mci.data.custom_id == format!("toggle_delete_{uuid}") {
            guild.hide_deletion_info = !guild.hide_deletion_info;
        }

        entity::guild::ActiveModel::update_guild(
            guild_id,
            guild.active,
            guild.retain_messages,
            guild.oauth,
            guild.hide_user,
            guild.hide_deletion_info,
        )
        .update(db)
        .await?;

        mci.edit_response(
            ctx,
            EditInteractionResponse::new()
                .embed(create_embed(&guild))
                .components(create_components(&guild, uuid)),
        )
        .await?;
    }

    Ok(())
}
