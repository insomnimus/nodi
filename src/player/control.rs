pub struct PausablePlayer<T: Timer, C: Connection> {
	pub con: C,
	timer: T,
	pause: Receiver<bool>,
}

impl<T: Timer, C: Connection> PausablePlayer<T,C> {
	pub fn play_sheet(&mut self, slice: &[Moment]) {
		
	}
}