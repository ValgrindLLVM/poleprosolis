use ppl_game::{
    game::{Game, GameAction},
    ui::Context,
    ui_impls::tui,
};
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = tui::Context::init()?.map(Game::new);

    game.handle.ui.draw_borders()?;
    game.redraw_all()?;
    game.handle.draw_player_info()?;
    game.handle.ui.apply()?;
    let buff = &mut [0];

    while stdin().read_exact(buff).is_ok() {
        match buff[0] {
            b'w' => game.do_action(GameAction::MoveUp)?,
            b'a' => game.do_action(GameAction::MoveLeft)?,
            b's' => game.do_action(GameAction::MoveDown)?,
            b'd' => game.do_action(GameAction::MoveRight)?,

            b'e' => game.do_action(GameAction::Interact)?,
            b'i' => game.handle.toggle_inventory(),

            b'q' => break,

            _ => continue,
        }

        game.handle.draw_lore()?;
        game.handle.draw_player_info()?;
        game.handle.ui.apply()?;
    }

    Ok(())
}
