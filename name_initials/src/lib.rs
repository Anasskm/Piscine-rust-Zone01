pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|&name| {
            name.split_whitespace()
                .map(|s| {
					let first_char = s.chars().next().unwrap();
					let initials = format!("{}.", first_char); 
					initials
				})
				.collect::<Vec<String>>()
				.join(" ")
                
        })
        .collect()
}

