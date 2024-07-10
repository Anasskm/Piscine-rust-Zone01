use json::JsonValue;
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut res = JsonValue::new_object();
    let mut cals: f64 = 0.0;
    let mut carbs: f64 = 0.0;
    let mut proteins: f64 = 0.0;
    let mut fats: f64 = 0.0;
	let round = 10_f64.powi(2);

    for food in foods {
        let x = food.calories[1]
            .chars()
            .filter(|&c| c.is_numeric()||c == '.')
            .collect::<String>()
            .parse::<f64>()
            .unwrap();
        cals = cals + x * food.nbr_of_portions;
        carbs = carbs + (food.carbs * food.nbr_of_portions);
        fats = fats + (food.fats * food.nbr_of_portions);
        proteins = proteins + (food.proteins * food.nbr_of_portions);
    }
    res.insert("cals", (cals*round).round()/round).unwrap();
    res.insert("carbs", (carbs*round).round()/round).unwrap();
    res.insert("proteins", (proteins*round).round()/round).unwrap();
    res.insert("fats", (fats*round).round()/round).unwrap();
    res
}