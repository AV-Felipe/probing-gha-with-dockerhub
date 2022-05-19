fn main() {
    println!("Hello, world!");

    let felipe_allergie: Allergies = Allergies::new(125);

    println!("{:?}", felipe_allergie.is_allergic_to(&Allergen::Cats));

    println!("{:?}", felipe_allergie.allergies());
}


#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
    NA,
}

impl Allergen {
    pub fn allergen_value(&self) -> u32{
        match &self {
            Allergen::Eggs => {
                println!("1");
                1
            },
            Allergen::Peanuts => {
                println!("2");
                2
            },
            Allergen::Shellfish => {
                println!("4");
                4
            },
            Allergen::Strawberries => {
                println!("8");
                8
            },
            Allergen::Tomatoes => {
                println!("16");
                16
            },
            Allergen::Chocolate => {
                println!("32");
                32
            },
            Allergen::Pollen => {
                println!("64");
                64
            },
            Allergen::Cats => {
                println!("128");
                128
            },
            Allergen::NA => {
                println!("not listed");
                0
            }
        }
    }

}



fn allergen_by_value (value: u32) -> Result<Allergen, String>{
    match value {
        1 => Ok(Allergen::Eggs) ,
        2 => Ok(Allergen::Peanuts),
        4 => Ok(Allergen::Shellfish),
        8 => Ok(Allergen::Strawberries),
        16 => Ok(Allergen::Tomatoes),
        32 => Ok(Allergen::Chocolate),
        64 => Ok(Allergen::Pollen),
        128 => Ok(Allergen::Cats),
        _ => Err("Not_listed".to_string()),
    }
}


#[derive(Debug)]
pub struct Allergies{
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        
        let all_allergens = Self::allergies(&self);

        all_allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        
        let mut positive_alergens = vec![];

        let mut possible_range = vec![];

        let mut confirmed_range = vec![];

        let allergy_score: &u32 = &self.score;
        
        let mut current_value: u32 = 1;

        let mut current_sum: u32 = 0;

        // create range of possibilities
        loop {
            
            // check if allergy score is plain match with current value
            if *allergy_score == current_value {
                // positive_alergens.push(allergen_by_value(current_value).unwrap_or(Allergen::NA));
                confirmed_range.push(current_value);
                break;
            }

            if *allergy_score > current_sum {
                possible_range.push(current_value);
                current_sum = current_sum + current_value;
                current_value = current_value * 2;

            } else {
                //possible_range.push(current_value);
                confirmed_range.push(current_value / 2);
                break;
            }
        }

        current_value = confirmed_range[0].clone();
        current_sum = current_value.clone();

        let mut range_index: usize = 0;

        // filter confirmed values
        if current_value != *allergy_score {
        
            loop {

                if *allergy_score > current_sum {
                    current_sum = current_sum + possible_range[range_index];

                    range_index = range_index + 1;

                } else {
                    confirmed_range.push(possible_range[range_index -1]);

                    // current_sum = current_sum + possible_range[range_index];

                    current_value = current_value + possible_range[range_index -1];
                    current_sum = current_value.clone();
    
                    if *allergy_score == current_value {

                        break;
                    } else {
                        range_index = 0;
                    }
                }
                // test_flag = test_flag + 1;
                // if test_flag == 3 {break;} 
            }
        }


        for value in confirmed_range {
            positive_alergens.push(allergen_by_value(value).unwrap_or(Allergen::NA));
        }


        //positive_alergens.push(Allergen::Cats);

        positive_alergens.retain(|x| *x != Allergen::NA);


        positive_alergens

    }
}