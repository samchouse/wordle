#![cfg(target_family = "unix")]
mod lib;

use std::io;

use crossterm::{
    self, cursor, execute,
    terminal::{self, ClearType},
};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use paris::{error, info};

fn run() -> crossterm::Result<()> {
    execute!(
        io::stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    let confirmed_letters: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Confirmed letters")
        .interact_text()?;
    let mut extra_letters = String::new();
    let mut schema = String::new();
    let mut multiple_letters = String::new();

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to add extra letters?")
        .default(false)
        .interact()?
    {
        extra_letters = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Extra letters")
            .interact_text()?;
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to add multiple letters?")
        .default(false)
        .interact()?
    {
        multiple_letters = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Multiple letters")
            .interact_text()?;
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to a schema?")
        .default(false)
        .interact()?
    {
        schema = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Schema")
            .interact_text()?;
    }

    println!();

    let mut found_words = false;
    lib::solver(
        &confirmed_letters,
        &extra_letters,
        &multiple_letters,
        &schema,
    )
    .split(",")
    .for_each(|word| {
        if word.trim() != "" {
            found_words = true;
            info!("{}", word);
        }
    });

    if !found_words {
        error!("No words found");
    }

    println!();

    Ok(())
}

fn main() -> crossterm::Result<()> {
    let mut first_run = true;
    loop {
        if first_run {
            run()?;
            first_run = false;
        }

        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to run again?")
            .default(true)
            .interact_opt()?
        {
            Some(true) => run()?,
            Some(false) => break,
            _ => (),
        }
    }

    execute!(
        io::stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    Ok(())
}
