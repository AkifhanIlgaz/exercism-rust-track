#[derive(Clone)]
pub struct Allergies {
    pub allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    pub fn from(score: u32) -> Option<Self> {
        match score {
            0 => Some(Allergen::Eggs),
            1 => Some(Allergen::Peanuts),
            2 => Some(Allergen::Shellfish),
            3 => Some(Allergen::Strawberries),
            4 => Some(Allergen::Tomatoes),
            5 => Some(Allergen::Chocolate),
            6 => Some(Allergen::Pollen),
            7 => Some(Allergen::Cats),
            _ => None,
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut score = score;
        let mut allergies = Vec::new();

        while score != 0 {
            let exp = f64::log2(score as f64) as u32;

            if let Some(allergen) = Allergen::from(exp) {
                allergies.push(allergen)
            }
            
            score -= 2_u32.pow(exp);
        }

        Self { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.iter().cloned().collect()
    }
}
