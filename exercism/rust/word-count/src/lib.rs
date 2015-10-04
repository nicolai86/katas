use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
  let mut map = HashMap::new();
  for word in s.to_lowercase().split( |c:char| !c.is_alphanumeric() ).filter( |p| p.len() > 0 ) {
    let counter = map.entry(word.to_string()).or_insert(0u32);
    *counter += 1;
  };
  return map
}
