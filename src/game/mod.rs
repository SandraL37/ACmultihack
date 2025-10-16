use std::{cell::RefCell, sync::LazyLock};

use crate::{game::game::Game, hacks};
pub mod game;
pub mod state;

pub static GAME: LazyLock<Game> = LazyLock::new(|| {
    let game = Game::new().unwrap();
    game
});

thread_local! {
    pub static STATE: LazyLock<RefCell<state::State>> = LazyLock::new(|| {
        RefCell::new(
            state::State {
                aimbot: false,
                crosshair: false,
                esp: false,
                no_clip: false,
                trace: false,
                triggerbot: false,
                wallbang: false,
                maphack: hacks::maphack(),
                show_menu: false,
            }
        )
    });
}