pub fn is_empty(v: &str) -> bool {
    if v.len()==0{
        return true;
    }
    false
}

pub fn is_ascii(v: &str) -> bool {
    v.chars().all(|c| c.is_ascii())
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
   v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
  if let Some(i) = v.find(pat){
    return  i;
   }
   return 0;
}