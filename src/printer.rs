use crate::mal_types::MalType;

pub fn pr_str(ast: &MalType) -> String {
    match ast {
        MalType::Symbol(s) => s.clone(),
        MalType::Number(n) => n.to_string(),
        MalType::Nil => String::from("nil"),
        MalType::MalList(l) => {
            let str_list = l
                .to_vec()
                .into_iter()
                .map(|s| pr_str(&s))
                .collect::<Vec<String>>()
                .join(" ");
            format!("{}{}{}", "(", str_list, ")")
        }
        MalType::Vector(l) => {
            let str_list = l
                .to_vec()
                .into_iter()
                .map(|s| pr_str(&s))
                .collect::<Vec<String>>()
                .join(" ");
            format!("{}{}{}", "[", str_list, "]")
        }
        MalType::Str(s) => format!("\"{s}\""),
        MalType::Keyword(k) => format!(":{k}"),
        MalType::Bool(a) => a.to_string(),
    }
}
