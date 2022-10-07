use regex::{Captures, Regex};
use std::{collections::HashMap, fmt};

enum Data {
    Number(i32),
    Boolean(bool),
    Text(String),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Text(x) => write!(f, "{}", x),
            Self::Number(x) => write!(f, "{}", x),
            Self::Boolean(x) => write!(f, "{}", x),
        }
    }
}

fn render(mut template: String, mut data: HashMap<&str, Data>) -> String {
    let print_regex = Regex::new(r"\{\{(.*?)\}\}").unwrap();
    template = print_regex
        .replace_all(&template, |caps: &Captures| {
            let key = caps.get(1).unwrap().as_str().trim();
            data[key].to_string()
        })
        .to_string();

    let repeat_regex =
        Regex::new(r"\{% repeat (\d*?) times %\}((.|\n)*?)\{% endrepeat %\}").unwrap();
    template = repeat_regex
        .replace_all(&template, |caps: &Captures| {
            // Extract the number of times to repeat the code
            let times = caps.get(1).unwrap().as_str().trim();
            // Parse the code block to be repeated
            let code = caps.get(2).unwrap().as_str().trim();
            // Repeat the code `times` number of times
            code.repeat(times.parse::<usize>().unwrap())
        })
        .to_string();

    let if_else_regex = Regex::new(
        r"\{% if (.*?) %\}((.|\n)*?)(\{% else %\}((.|\n)*?)\{% endif %\}|\{% endif %\})",
    )
    .unwrap();
    template = if_else_regex
        .replace_all(&template, |caps: &Captures| {
            // Extract the name of the bool being tested
            let key = caps.get(1).unwrap().as_str().trim();
            // Parse the 'if' and (optional) 'else' code blocks
            let if_code = caps.get(2).unwrap().as_str().trim();
            let else_code = caps.get(5).map_or("", |m| m.as_str()).trim();
            // Find the corresponding key in the Data and return the value
            if let Data::Boolean(exp) = data[key] {
                if exp {
                    if_code.to_string()
                } else {
                    else_code.to_string()
                }
            } else {
                "ERROR PARSING KEY".to_string()
            }
        })
        .to_string();

    template = template.replace("{#", "<!--").replace("#}", "-->");

    template
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
