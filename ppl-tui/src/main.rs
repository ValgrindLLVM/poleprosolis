use ppl_game::{
    game::{Game, GameAction},
    ui::Context,
    ui_impls::tui,
};
use std::{
    ffi::c_int,
    io::{self, stdin, Read},
};

// consts: STDOUT_FD, F_SETFL, O_NONBLOCK
include!(concat!(env!("OUT_DIR"), "/_consts.rs"));

extern "C" {
    /// See `fcntl(2)`
    #[must_use]
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: error handling (-1 is a error)
    // FIXME: STDOUT_FD here
    // SAFETY: see fcntl(2)
    _ = unsafe { fcntl(0, F_SETFL, O_NONBLOCK) };

    let mut game = tui::Context::init()?.map(Game::new);

    game.handle.ui.draw_borders()?;
    game.redraw_all()?;
    game.handle.draw_player_info()?;
    game.handle.ui.apply()?;
    let buff = &mut [0];
    let mut counter = 0u16;

    loop {
        match stdin().read_exact(buff) {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                counter = counter.saturating_add(1);
                continue;
            }
            Err(e) => {
                // TODO: fix this smol iq moment
                drop(game);
                panic!("{e}");
            }
        };

        match buff[0] {
            b'w' => game.do_action(GameAction::MoveUp)?,
            b'a' => game.do_action(GameAction::MoveLeft)?,
            b's' => game.do_action(GameAction::MoveDown)?,
            b'd' => game.do_action(GameAction::MoveRight)?,

            b'e' => game.do_action(GameAction::Interact)?,
            b'i' => game.handle.toggle_inventory(),

            b'q' => break,

            _ => {}
        }

        if counter == u16::MAX {
            counter = 0;
            game.do_random_tick()?;
        }

        game.handle.draw_lore()?;
        game.handle.draw_player_info()?;
        game.handle.ui.apply()?;
    }

    Ok(())
}
