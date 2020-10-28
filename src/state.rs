use amethyst::prelude::*;
use amethyst::input::{VirtualKeyCode, is_key_down};

struct GameplayState {
    player_count: u8
}

struct PauseState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_,'_>>) {
        println!("number of players: {}", self.player_count);
    }

    fn handle_event(&mut self, _data: StateData<()>, event: StateEvent) -> EmptyState {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PauseState));
            }
        }
    }
}

impl EmptyState for PauseState {
    fn handle_event(&mut self, _data: StateData<()>, event: StateEvent) -> EmptyState {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }
    }
}
