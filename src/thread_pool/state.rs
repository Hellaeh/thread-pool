use self::state_cell::StateCell;

#[derive(Default, Debug)]
pub struct State {
	pub panicking: StateCell,
	pub busy: StateCell,
}

pub mod state_cell;
