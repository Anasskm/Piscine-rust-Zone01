pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val)
}

pub fn at_index(vec: &Vec<String>, index: usize) ->String{
   let res =&vec[index];
   res.to_string()
}
