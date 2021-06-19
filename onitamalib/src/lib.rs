pub mod cards;
pub mod board;
pub mod models;
pub mod singleplayer;
mod game;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use crate::models::{GameState, GameView, Move, GameError};
pub use singleplayer::*;

#[wasm_bindgen]
pub struct Game {
    state: GameState,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            state: GameState::new(),
        }
    }
    #[wasm_bindgen(js_name = getState)]
    pub fn get_state(&self) -> JsValue {
        let state: GameView = GameView::from(&self.state);
        JsValue::from_serde(&state).unwrap()
    }
    fn play_move_inner(&mut self, game_move: &JsValue) -> Result<GameView, GameError> {
        let game_move: Move = match game_move.into_serde() {
            Ok(game_move) => game_move,
            Err(err) => {
                return Err(GameError::Error {
                    message:  err.to_string(),
                });
            }
        };
        let board = match &self.state {
            GameState::Playing { board } => board,
            GameState::Finished { .. } => {
                return Err(GameError::Error {
                    message: "Game Finished".to_string(),
                });
            },
        };
        let game_state = match board.make_move(game_move) {
            Ok(game_state) => game_state,
            Err(message) => {
                return Err(GameError::Error {
                    message,
                });
            },
        };
        self.state = game_state;
        Ok(GameView::from(&self.state))
    }
    #[wasm_bindgen(js_name = move)]
    pub fn play_move(&mut self, game_move: &JsValue) -> JsValue {
        match self.play_move_inner(game_move) {
            Ok(game_view) => {
                log::info!("Played move: {:?}", &game_move);
                JsValue::from_serde(&game_view).unwrap()
            },
            Err(err) => {
                log::error!("Error in play move: {:?}", &err);
                JsValue::from_serde(&err).unwrap()
            },
        }
    }
    fn import_inner(&mut self, state: &JsValue) -> Result<GameView, GameError> {
        let state: GameState = match state.into_serde() {
            Ok(state) => state,
            Err(err) => {
                let err = GameError::Error { message: err.to_string() };
                return Err(err);
            },
        };
        self.state = state;
        let view = GameView::from(&self.state);
        Ok(view)
    }
    #[wasm_bindgen(js_name = importState)]
    pub fn import(&mut self, state: &JsValue) -> JsValue {
        match self.import_inner(state) {
            Ok(game_view) => {
                log::info!("Imported state: {:?}", &game_view);
                JsValue::from_serde(&game_view).unwrap()
            },
            Err(err) => {
                log::error!("Error in import: {:?}", &err);
                JsValue::from_serde(&err).unwrap()
            },
        }
    }
    #[wasm_bindgen(js_name = exportState)]
    pub fn export(&self, state: &JsValue) -> JsValue {
        JsValue::from_serde(&self.state).unwrap()
    }
}

#[wasm_bindgen(start)]
pub fn init(){
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Test");
}
