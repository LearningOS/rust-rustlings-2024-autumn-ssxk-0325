// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html

// 这与 from_into.rs 类似，但这次我们将实现 FromStr 并返回错误，而不是回退到默认值。此外，
// 在实现 FromStr 后，您可以使用字符串上的 parse 方法生成实现类型的对象。您可以在 这里 阅读更多信息。
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
// 我们将在“FromStr”实现中使用此错误类型。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}


// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object

// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

// 步骤：

// 如果提供的字符串长度为 0，则应返回一个错误。
// 根据字符串中的逗号拆分给定的字符串。
// 只有两个元素应该从拆分中返回，否则返回一个错误。
// 从拆分操作中提取第一个元素，并将其用作姓名。
// 从拆分操作中提取另一个元素，并使用类似 "4".parse::<usize>() 的方法将其解析为 usize 类型的年龄。
// 如果在提取姓名和年龄时出现问题，则应返回一个错误。
// 如果一切顺利，则返回一个 Person 对象的 Result。

// 作为补充：Box<dyn Error> 实现了 From<&'_ str>。
// 这意味着如果您想返回一个字符串错误消息，您可以通过使用 
// return Err("my error message".into()) 来做到这一点。

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        let parts:Vec<&str> = s.split(',').collect();

        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }
        let name = parts[0].trim();
        if name.is_empty(){
            return Err(ParsePersonError::NoName);
        }

        let age = parts[1].trim().parse::<usize>().map_err(ParsePersonError::ParseInt)?;

        Ok(Person{
            name:name.to_string(),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
