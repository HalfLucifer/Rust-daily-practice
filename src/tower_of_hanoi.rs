pub struct HanoiTower {
    pillar: Vec<i32>,
}

impl HanoiTower {
    pub fn new() -> Self {
        Self { pillar: vec![] }
    }

    pub fn list(&self) -> Vec<i32> {
        self.pillar.clone()
    }

    pub fn add_disc(&mut self, value: i32) -> Result<(), i32> {
        if !self.pillar.is_empty() && self.pillar.last().unwrap() < &value {
            return Err(value);
        }

        self.pillar.push(value);

        Ok(())
    }

    pub fn move_all(&mut self, buffer: &mut HanoiTower, destination: &mut HanoiTower) {
        self.move_recursive(self.pillar.len() as i32, buffer, destination);
    }

    fn move_recursive(&mut self, num_of_discs: i32, buffer: &mut HanoiTower, destination: &mut HanoiTower) {
        if num_of_discs > 0 {
            self.move_recursive(num_of_discs - 1, destination, buffer);
            self.move_disc(destination);
            buffer.move_recursive(num_of_discs - 1, self, destination);
        }
    }

    fn move_disc(&mut self, destination: &mut HanoiTower) {
        if !self.pillar.is_empty() {
            let _result = destination.add_disc(self.pillar.pop().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut origin = HanoiTower::new();
        let mut buffer = HanoiTower::new();
        let mut destination = HanoiTower::new();
        let num_of_discs = 3;

        for i in (1..=num_of_discs).rev() {
            let _result = origin.add_disc(i);
        }

        assert_eq!(origin.list(), [3, 2, 1]);
        origin.move_all(&mut buffer, &mut destination);

        assert_eq!(origin.list(), []);
        assert_eq!(buffer.list(), []);
        assert_eq!(destination.list(), [3, 2, 1]);
    }

    #[test]
    fn test_edge_cases() {
        let mut origin = HanoiTower::new();
        let mut buffer = HanoiTower::new();
        let mut destination = HanoiTower::new();

        origin.move_all(&mut buffer, &mut destination);
        assert_eq!(destination.list(), []);

        let _result = origin.add_disc(0);
        origin.move_all(&mut buffer, &mut destination);
        assert_eq!(destination.list(), [0]);

        let mut origin = HanoiTower::new();
        let mut buffer = HanoiTower::new();
        let mut destination = HanoiTower::new();
        let _result = origin.add_disc(1);
        let _result = origin.add_disc(0);
        origin.move_all(&mut buffer, &mut destination);

        assert_eq!(origin.list(), []);
        assert_eq!(buffer.list(), []);
        assert_eq!(destination.list(), [1, 0]);
    }

    #[test]
    fn test_add_discs() {
        let mut tower = HanoiTower::new();
        assert_eq!(tower.add_disc(2), Ok(()));
        assert_eq!(tower.add_disc(1), Ok(()));
        assert_eq!(tower.add_disc(3), Err(3));
        assert_eq!(tower.add_disc(1), Ok(()));
        assert_eq!(tower.add_disc(0), Ok(()));
        assert_eq!(tower.add_disc(2), Err(2));
        assert_eq!(tower.add_disc(100), Err(100));
        assert_eq!(tower.add_disc(-1), Ok(()));
    }

    #[test]
    fn test_larger_cases() {
        let mut origin = HanoiTower::new();
        let mut buffer = HanoiTower::new();
        let mut destination = HanoiTower::new();
        let num_of_discs = 24;

        for i in (1..=num_of_discs * 2).rev() {
            if i % 2 == 0 {
                let _result = origin.add_disc(i);
            }
        }

        let expected = origin.list();
        origin.move_all(&mut buffer, &mut destination);

        assert_eq!(origin.list(), []);
        assert_eq!(buffer.list(), []);
        assert_eq!(destination.list(), expected);
    }
}
