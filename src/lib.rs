
use std::fmt;

#[derive(Clone)]
pub struct StringFactory{
    //Public for tests only
    pub(crate) string: String
}

impl fmt::Display for StringFactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl StringFactory{

    //--------- Creation ----------

    pub fn new() -> StringFactory{
        StringFactory {string: String::new()}
    }

    pub fn from_str(val: &str) -> StringFactory{
        StringFactory {string: val.into()}
    }

    pub fn from_string(string: String) -> StringFactory{
        StringFactory {string: string}
    }

    //---------- Appending ----------

    pub fn append<T: fmt::Display>(&mut self, val: T){
        self.string = format!("{}{}", self.string, val);
    }

    pub fn append_debug<T: fmt::Debug>(&mut self, val: T){
        self.string = format!("{}{:?}", self.string, val);
    }

    pub fn append_front<T: fmt::Display>(&mut self, val: T){
        self.string = format!("{}{}", val,  self.string);
    }

    pub fn append_debug_front<T: fmt::Debug>(&mut self, val: T){
        self.string = format!("{:?}{}", val, self.string);
    }

    //---------- Utilities ----------

    pub fn to_char_vec(&self) -> Vec<char>{
        let mut vec = Vec::new();

        for c in self.string.chars(){
            vec.push(c);
        }

        vec
    }

    pub fn index_of(&self, val: char) -> Option<usize>{
        for (index, c) in self.string.chars().enumerate() {
            if c == val{
                return Some(index);
            }
        }
        None
    }

    pub fn indexes_of(&self, val: char) -> Vec<usize>{
        let mut vec = Vec::new();

        for (index, c) in self.string.chars().enumerate() {
            if c == val{
                vec.push(index);
            }
        }
        vec
    }

    pub fn replace(&mut self, from: char, to: char){
        let mut chars = self.to_char_vec();
        let indexes = self.indexes_of(from);

        for index in indexes{
            chars[index] = to;
        }

        self.string = chars.iter().collect();
    }
}

#[cfg(test)]
mod string_factory_tests {
    use super::StringFactory;

    #[test]
    fn test_creation() {
        assert_eq!("".to_string(), StringFactory::new().string);
        assert_eq!("Test".to_string(), StringFactory::from_str("Test").string);
        assert_eq!("Test".to_string(),
            StringFactory::from_string("Test".to_string()).string);
    }

    #[test]
    fn test_append() {
        let mut val = StringFactory::new();

        val.append(5);
        assert_eq!("5".to_string(), val.string);
        val.append_front(20);
        assert_eq!("205".to_string(), val.string);

        let mut val = StringFactory::new();

        val.append_debug(vec![0]);
        assert_eq!(format!("{:?}", vec![0]), val.string);
        val.append_debug_front(vec![5]);
        assert_eq!(format!("{:?}{:?}", vec![5], vec![0]), val.string);
    }

    #[test]
    fn test_index(){
        //index_of
        assert!(StringFactory::from_str("ABC").index_of('B') == Some(1));
        assert!(StringFactory::from_str("ABC").index_of('b') == None);
        assert!(StringFactory::from_str("Hello").index_of('o') == Some(4));

        //indexes_of
        assert!(StringFactory::from_str("bob").indexes_of('b') == vec![0, 2]);
        assert!(StringFactory::from_str("Tatoo").indexes_of('o') == vec![3, 4]);
        assert!(StringFactory::from_str("Test").indexes_of('e') != vec![0]);
    }

    #[test]
    fn test_replace(){
        let mut fac = StringFactory::from_str("Bob");
        fac.replace('o', '0');
        println!("{}", fac.string);
        assert!(fac.string == "B0b".to_string());

        let mut fac = StringFactory::from_str("Book");
        fac.replace('o', '0');
        assert!(fac.string == "B00k");
    }

}
