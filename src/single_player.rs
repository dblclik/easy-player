pub struct SinglePlayer {
	score: usize,
	lives: usize,
	credits: usize
}

impl SinglePlayer {
	pub fn add_score(&mut self, increment: usize) {
		self.score = usize::saturating_add(self.score, increment);
	}

	pub fn reduce_score(&mut self, increment: usize) {
		self.score = usize::saturating_sub(self.score, increment);
	}

	pub fn is_active(&self) -> bool {
		return self.lives > 0
	}

	pub fn add_lives(&mut self, increment: usize) {
		self.lives = usize::saturating_add(self.lives, increment);
	}

	pub fn reduce_lives(&mut self, increment: usize) {
		self.lives = usize::saturating_sub(self.lives, increment);
	}

	pub fn add_a_life(&mut self) {
		self.add_lives(1);
	}

	pub fn remove_a_life(&mut self) {
		self.reduce_lives(1);
	}

	pub fn add_credits(&mut self, increment: usize) {
		self.credits = usize::saturating_add(self.credits, increment);
	}

	pub fn reduce_credits(&mut self, increment: usize) {
		self.credits = usize::saturating_sub(self.credits, increment);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_score() {
        let mut single_player = SinglePlayer{
            score: 0,
            lives: 5,
            credits: 1
        };
        single_player.add_score(100);
        assert_eq!(single_player.score, 100);
        single_player.reduce_score(50);
        assert_eq!(single_player.score, 50);
        single_player.reduce_score(500);
        assert_eq!(single_player.score, 0);
		single_player.add_score(1);
		single_player.add_score(usize::MAX);
		assert_eq!(single_player.score, usize::MAX);
		single_player.reduce_score(1);
		single_player.reduce_score(usize::MAX);
		assert_eq!(single_player.score, usize::MIN);
    }

	#[test]
	fn test_lives() {
		let mut single_player = SinglePlayer{
            score: 0,
            lives: 5,
            credits: 1
        };
		single_player.remove_a_life();
		assert_eq!(single_player.lives, 4);
		single_player.add_a_life();
		assert_eq!(single_player.lives, 5);
		single_player.add_lives(5);
		assert_eq!(single_player.lives, 10);
		assert_eq!(true, single_player.is_active());
		single_player.reduce_lives(15);
		assert_eq!(single_player.lives, 0);
		assert_eq!(false, single_player.is_active());
	}

	#[test]
	fn test_credits() {
        let mut single_player = SinglePlayer{
            score: 0,
            lives: 5,
            credits: 1
        };
        single_player.add_credits(100);
        assert_eq!(single_player.credits, 101);
        single_player.reduce_credits(50);
        assert_eq!(single_player.credits, 51);
        single_player.reduce_credits(500);
        assert_eq!(single_player.credits, 0);
		single_player.add_credits(1);
		single_player.add_credits(usize::MAX);
		assert_eq!(single_player.credits, usize::MAX);
		single_player.reduce_credits(1);
		single_player.reduce_credits(usize::MAX);
		assert_eq!(single_player.credits, usize::MIN);
    }
}