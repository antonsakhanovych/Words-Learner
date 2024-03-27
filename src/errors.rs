use color_eyre::{config::HookBuilder, eyre};

use crate::tui;
use std::panic;

const ERR_MESSAGE: &'static str = "ERROR: Couldn't restore to normal";

pub fn install_hooks() -> color_eyre::Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        tui::restore().expect(ERR_MESSAGE);
        panic_hook(panic_info);
    }));

    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |err: &(dyn std::error::Error + 'static)| {
        tui::restore().expect(ERR_MESSAGE);
        eyre_hook(err)
    }))?;
    Ok(())
}
