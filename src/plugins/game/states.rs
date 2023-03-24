use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Setup,
    Finished,
}

impl States for AppState {
    type Iter = std::array::IntoIter<AppState, 2>;

    fn variants() -> Self::Iter {
        [AppState::Setup, AppState::Finished].into_iter()
    }
}
