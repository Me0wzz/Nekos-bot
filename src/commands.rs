use crate::api::get_image;
use crate::types::*;

pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn neko(
    ctx: Context<'_>,
    #[description = "Selected Tag"] choice: NTags,
) -> Result<(), Error> {
    //let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let image_url = get_image(choice).await;
    match image_url {
        Some(url) => {
            let response = format!("{}", url);
            ctx.say(response).await?;
        }
        None => {
            let response = format!("Cannot parse URL");
            ctx.say(response).await?;
        }
    }
    Ok(())
}
