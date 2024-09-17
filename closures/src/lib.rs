#[derive(Debug, Clone, Copy)]
pub enum ShirtColour {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColour>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
        // The closure captures an immutable reference to self
        // No type annotations necessary in most cases as compiler can infer
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColour {
        let mut red = 0;
        let mut blue = 0;
        for shirt in &self.shirts {
            match shirt {
                ShirtColour::Red => red += 1,
                ShirtColour::Blue => blue += 1,
            }
        };
        if red > blue {
            ShirtColour::Red
        } else {
            ShirtColour::Blue
        }
    }
}