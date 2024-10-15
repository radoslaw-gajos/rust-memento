#[derive(Clone, Copy, PartialEq, Debug)]
enum Material {
    Straw,
    Sticks,
    Bricks,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Condition {
    Solid,
    BlownDown,
}

#[derive(Clone, PartialEq, Debug)]
struct House {
    resident: String,
    material: Material,
    condition: Condition,
}

impl House {
    fn new(resident: String, material: Material) -> House {
        House {
            resident,
            material,
            condition: Condition::Solid,
        }
    }

    fn blow_down(&mut self) {
        self.condition = match self.material {
            Material::Straw => Condition::BlownDown,
            Material::Sticks => Condition::BlownDown,
            Material::Bricks => self.condition,
        };
    }
}

struct VillageMemento {
    houses: Vec<House>,
}

#[derive(Clone, PartialEq, Debug)]
struct Village {
    houses: Vec<House>,
}

impl Village {
    fn new() -> Village {
        Village {
            houses: Vec::new(),
        }
    }

    fn memento(&self) -> VillageMemento {
        VillageMemento {
            houses: self.houses.clone(),
        }
    }

    fn restore(&mut self, memento: &VillageMemento) {
        self.houses = memento.houses.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_blow_down_straw_house() {
        // given
        let mut house = House::new("Straw Man".to_string(), Material::Straw);

        // when
        house.blow_down();

        // then
        assert_eq!(house.condition, Condition::BlownDown);
    }

    #[test]
    fn should_return_village_to_previous_state() {
        // given
        let mut village = Village::new();
        village.houses.push(House::new("Irresponsible Pig".to_string(), Material::Straw));
        village.houses.push(House::new("Regular Pig".to_string(), Material::Sticks));
        village.houses.push(House::new("Genius Pig".to_string(), Material::Bricks));
        let expected = village.clone();

        // when
        let village_memento = village.memento();
        for house in &mut village.houses {
            house.blow_down();
        }
        village.houses[1].resident = "Big Bad Wolf".to_string();
        village.restore(&village_memento);

        // then
        assert_eq!(village, expected);
    }
}
