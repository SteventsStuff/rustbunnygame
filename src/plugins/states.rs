use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    InLoadingLevel,
    InGame,
    InPause,
}

impl States for GameState {
    type Iter = std::array::IntoIter<GameState, 3>;

    fn variants() -> Self::Iter {
        [
            GameState::InLoadingLevel,
            GameState::InGame,
            GameState::InPause,
        ]
        .into_iter()
    }
}
