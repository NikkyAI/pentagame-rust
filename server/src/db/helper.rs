/*
 removes trailing null characters from String
 Required for db operations, because Postgresql TEXT field doesn't support tailing NULL characters
*/
pub fn zero_trim(s: &String) -> String {
    s.trim_matches(char::from(0)).to_owned()
}
