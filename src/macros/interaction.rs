#[macro_export]
macro_rules! require_admin {
    ($ctx: ident) => {
        check_permissions($ctx, poise::serenity_prelude::Permissions::ADMINISTRATOR)?
    };
}

#[macro_export]
macro_rules! require_mod {
    ($ctx: ident) => {
        check_permissions($ctx, poise::serenity_prelude::Permissions::MANAGE_MESSAGES)?
    };
}

#[macro_export]
macro_rules! defer_ephemeral {
    ($ctx: ident) => {
        $ctx.defer_response(true).await?
    };
}
