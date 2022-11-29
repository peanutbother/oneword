use crate::util::Error;

pub(crate) async fn handle() -> Result<(), Error> {
    log::info!("bot ready");
    Ok(())
}
