use crate::{
    complex::{
        NumStr,
        NumStr::{Num, Str, Vector},
    },
    Options,
};
use rug::{ops::Pow, Complex};
use std::collections::HashSet;
pub fn get_func(input: &str, options: Options) -> Result<Vec<NumStr>, &'static str> {
    let mut count: i32 = 0;
    let mut exp = String::new();
    let mut func: Vec<NumStr> = Vec::new();
    let mut word = String::new();
    let mut find_word = false;
    let mut abs = true;
    let mut neg = false;
    let mut i = 1;
    let mut chars = input.chars().collect::<Vec<char>>();
    while i < chars.len() - 1 {
        if chars[i].is_whitespace() {
            if chars[i - 1].is_numeric() && chars[i + 1].is_numeric() {
                chars[i] = '*'
            } else {
                chars.remove(i);
            }
        }
        i += 1;
    }
    i = 0;
    let (mut c, mut deci);
    let n1 = Complex::with_val(options.prec, -1);
    let mut pow = String::new();
    let mut sum = 0;
    'outer: while i < chars.len() {
        c = chars[i];
        if !matches!(
            c,
            '⁰' | '₀'
                | '⁹'
                | '₉'
                | '⁸'
                | '₈'
                | '⁷'
                | '₇'
                | '⁶'
                | '₆'
                | '⁵'
                | '₅'
                | '⁴'
                | '₄'
                | '³'
                | '₃'
                | '²'
                | '₂'
                | '¹'
                | '₁'
                | '⁻'
                | 'ⁱ'
        ) && !pow.is_empty()
        {
            let i = pow.matches('i').count() % 4;
            pow = pow.replace('i', "");
            if pow.is_empty() {
                pow = "1".to_string();
            }
            if !func.is_empty()
                && (func.last().unwrap().num().is_ok() || func.last().unwrap().str_is(")"))
            {
                func.push(Str("^".to_string()));
            }
            func.push(Num(Complex::with_val(
                options.prec,
                match Complex::parse(pow.as_bytes()) {
                    Ok(n) => n,
                    Err(_) => return Err("exponent error"),
                },
            ) * Complex::with_val(options.prec, (0, 1))
                .pow(Complex::with_val(options.prec, i))));
            pow = String::new();
        }
        if c.is_ascii_digit() {
            if !word.is_empty() && word != "0." {
                find_word = false;
                if is_func(&word) || sum != 0 {
                    place_multiplier(&mut func, &find_word);
                    func.push(Str(word.clone()));
                    if word == "sum" && sum == 0 {
                        sum = count + 1;
                    }
                }
                word.clear();
            }
            place_multiplier(&mut func, &find_word);
            deci = false;
            for c in chars[i..].iter() {
                match c {
                    '0'..='9' => {
                        word.push(*c);
                    }
                    '.' => {
                        if deci {
                            return Err("cant have multiple '.'");
                        }
                        deci = true;
                        word.push(*c);
                    }
                    _ => break,
                }
                i += 1;
            }
            if neg {
                if chars.len() > i && chars[i] == '^' {
                    func.push(Num(n1.clone()));
                    func.push(Str('*'.to_string()));
                } else {
                    word.insert(0, '-');
                }
                neg = false;
            }
            func.push(Num(Complex::with_val(
                options.prec,
                Complex::parse(word.as_bytes()).unwrap(),
            )));
            word.clear();
            continue;
        } else if c.is_alphabetic() {
            if find_word
                && (!(c == 'x' || c == 'y')
                    || (chars.len() - 1 != i && chars[i + 1] == 'p' && word == "e")
                    || word == "ma")
            {
                word.push(c);
            } else {
                if neg {
                    func.push(Num(n1.clone()));
                    func.push(Str('*'.to_string()));
                    neg = false;
                }
                match c {
                    'ⁱ' => pow.push('i'),
                    'E' | 'e'
                        if (options.small_e && c == 'e') || (!options.small_e && c == 'E') =>
                    {
                        place_multiplier(&mut func, &find_word);
                        func.push(Num(Complex::with_val(options.prec, 10)));
                        if i + 1 != chars.len()
                            && (chars[i + 1].is_alphanumeric()
                                || chars[i + 1] == '-'
                                || chars[i + 1] == '+'
                                || chars[i + 1] == '('
                                || chars[i + 1] == '{'
                                || chars[i + 1] == '[')
                        {
                            func.push(Str('^'.to_string()));
                            func.push(Str('('.to_string()));
                            count += 1;
                        }
                    }
                    'x' | 'y' => {
                        if !word.is_empty() {
                            find_word = false;
                            if is_func(&word) || sum != 0 {
                                place_multiplier(&mut func, &find_word);
                                func.push(Str(word.clone()));
                                if word == "sum" && sum == 0 {
                                    sum = count + 1;
                                }
                            }
                            word.clear();
                        }
                        place_multiplier(&mut func, &find_word);
                        func.push(Str(c.to_string()));
                    }
                    'i' => {
                        if i + 1 != chars.len() && (chars[i + 1] == 'n' || chars[i + 1] == 'm') {
                            word.push(c);
                            find_word = true;
                        } else {
                            place_multiplier(&mut func, &find_word);
                            func.push(Num(Complex::with_val(options.prec, (0, 1))));
                        }
                    }
                    _ => {
                        word.push(c);
                        find_word = true;
                    }
                }
            }
        } else {
            if !word.is_empty() {
                find_word = false;
                if i + 4 < chars.len()
                    && chars[i] == '^'
                    && chars[i + 1] == '('
                    && chars[i + 2] == '-'
                    && chars[i + 3] == '1'
                    && chars[i + 4] == ')'
                {
                    place_multiplier(&mut func, &find_word);
                    word.insert(0, 'a');
                    func.push(Str(word.clone()));
                    word.clear();
                    i += 5;
                    continue;
                }
                if i + 2 < chars.len()
                    && chars[i] == '^'
                    && chars[i + 1] == '-'
                    && chars[i + 2] == '1'
                {
                    place_multiplier(&mut func, &find_word);
                    word.insert(0, 'a');
                    func.push(Str(word.clone()));
                    word.clear();
                    i += 3;
                    continue;
                }
                if i + 1 < chars.len()
                    && chars[i] == '^'
                    && (chars[i + 1].is_ascii_digit() || chars[i + 1] == '-')
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Str(word.clone()));
                    word.clear();
                    let pos = chars.iter().skip(i + 1).position(|&c| c == '(' || c == ')');
                    if pos.is_none() {
                        continue;
                    }
                    exp = chars[i + 1..i + 1 + pos.unwrap()].iter().collect();
                    if exp == "-" {
                        exp = "-1".to_string();
                    }
                    i += pos.unwrap() + 1;
                    continue;
                }
                if is_func(&word) || sum != 0 {
                    place_multiplier(&mut func, &find_word);
                    func.push(Str(word.clone()));
                    if word == "sum" && sum == 0 {
                        sum = count + 1;
                    }
                    word.clear();
                }
            }
            if !exp.is_empty() && c != '(' && c != ')' {
                func.push(Str("^".to_string()));
                func.push(Num(Complex::with_val(
                    options.prec,
                    match Complex::parse(exp.as_bytes()) {
                        Ok(n) => n,
                        Err(_) => return Err("exponent error"),
                    },
                )));
                exp = String::new();
            }
            match c {
                '√' => func.push(Str("sqrt".to_string())),
                '∛' => func.push(Str("cbrt".to_string())),
                '¼' => func.push(Num(Complex::with_val(options.prec, 0.25))),
                '½' => func.push(Num(Complex::with_val(options.prec, 0.5))),
                '¾' => func.push(Num(Complex::with_val(options.prec, 0.75))),
                '⅒' => func.push(Num(Complex::with_val(options.prec, 0.1))),
                '⅕' => func.push(Num(Complex::with_val(options.prec, 0.2))),
                '⅖' => func.push(Num(Complex::with_val(options.prec, 0.4))),
                '⅗' => func.push(Num(Complex::with_val(options.prec, 0.6))),
                '⅘' => func.push(Num(Complex::with_val(options.prec, 0.8))),
                '⅐' => func.push(Num(Complex::with_val(options.prec, 7.0).recip())),
                '⅑' => func.push(Num(Complex::with_val(options.prec, 9.0).recip())),
                '⅓' => func.push(Num(Complex::with_val(options.prec, 3.0).recip())),
                '⅔' => func.push(Num(Complex::with_val(options.prec, 1.5).recip())),
                '⅙' => func.push(Num(Complex::with_val(options.prec, 6.0).recip())),
                '⅚' => func.push(Num(Complex::with_val(options.prec, 1.2).recip())),
                '⅛' => func.push(Num(Complex::with_val(options.prec, 0.125))),
                '⅜' => func.push(Num(Complex::with_val(options.prec, 0.375))),
                '⅝' => func.push(Num(Complex::with_val(options.prec, 0.625))),
                '⅞' => func.push(Num(Complex::with_val(options.prec, 0.875))),
                '⅟' => {
                    func.push(Num(Complex::with_val(options.prec, 1)));
                    func.push(Str("/".to_string()))
                }
                '↉' => func.push(Num(Complex::new(options.prec))),
                '⁰' | '₀' => pow.push('0'),
                '⁹' | '₉' => pow.push('9'),
                '⁸' | '₈' => pow.push('8'),
                '⁷' | '₇' => pow.push('7'),
                '⁶' | '₆' => pow.push('6'),
                '⁵' | '₅' => pow.push('5'),
                '⁴' | '₄' => pow.push('4'),
                '³' | '₃' => pow.push('3'),
                '²' | '₂' => pow.push('2'),
                '¹' | '₁' => pow.push('1'),
                '⁻' => pow.push('-'),
                '.' => word.push_str("0."),
                '&' if i != 0 && i + 1 < chars.len() && chars[i + 1] == '&' => {
                    func.push(Str("&&".to_string()));
                }
                '*' if i != 0 && i + 1 != chars.len() => {
                    if i + 1 != chars.len() && chars[i + 1] == '*' {
                        func.push(Str("^".to_string()));
                        i += 1;
                    } else {
                        func.push(Str('*'.to_string()));
                    }
                }
                '=' if i != 0 && i + 1 < chars.len() => {
                    if chars[i + 1] == '=' {
                        func.push(Str("==".to_string()));
                        i += 1;
                    } else if chars[i - 1] == '>' {
                        func.push(Str(">=".to_string()));
                    } else if chars[i - 1] == '<' {
                        func.push(Str("<=".to_string()));
                    }
                }
                '{' => {
                    place_multiplier(&mut func, &find_word);
                    if neg {
                        func.push(Num(n1.clone()));
                        func.push(Str('*'.to_string()));
                        neg = false;
                    }
                    func.push(Str("{".to_string()));
                }
                '}' => {
                    func.push(Str("}".to_string()));
                }
                '±' if i + 1 != chars.len() => {
                    if func.is_empty()
                        || matches!(func.last().unwrap(), Str(s) if !(s == ")" || s == "*"))
                    {
                        func.push(Num(Complex::new(options.prec)))
                    }
                    func.push(Str("±".to_string()))
                }
                '/' if i != 0 && i + 1 != chars.len() => func.push(Str('/'.to_string())),
                '+' if i != 0
                    && i + 1 != chars.len()
                    && (chars[i - 1].is_alphanumeric()
                        || (!func.is_empty() && func.last().unwrap().str_is(")"))
                        || chars[i - 1] == '}'
                        || chars[i - 1] == ']')
                    && chars[i - 1] != if options.small_e { 'e' } else { 'E' } =>
                {
                    func.push(Str('+'.to_string()))
                }
                '<' if i != 0 && i + 1 < chars.len() && chars[i + 1] != '=' => {
                    if chars[i + 1] == '<' {
                        func.push(Str("<<".to_string()));
                        i += 1;
                    } else {
                        func.push(Str('<'.to_string()));
                    }
                }
                '>' if i != 0 && i + 1 < chars.len() && chars[i + 1] != '=' => {
                    if chars[i + 1] == '>' {
                        func.push(Str(">>".to_string()));
                        i += 1;
                    } else {
                        func.push(Str('>'.to_string()));
                    }
                }
                '-' => {
                    if i != 0 && chars[i - 1] == '^' {
                        func.push(Str("(".to_string()));
                        func.push(Num(n1.clone()));
                        count += 1;
                    } else if i == 0
                        || !(chars[i - 1] != if options.small_e { 'e' } else { 'E' }
                            && (chars[i - 1].is_alphanumeric()
                                || func.last().unwrap().str_is(")")
                                || chars[i - 1] == '}'
                                || chars[i - 1] == ']'))
                    {
                        if i + 1 != chars.len() && (chars[i + 1] == '(' || chars[i + 1] == '-') {
                            func.push(Num(n1.clone()));
                            func.push(Str("*".to_string()));
                        } else {
                            neg = true;
                        }
                    } else {
                        func.push(Str('-'.to_string()));
                    }
                }
                '^' if i != 0 && i + 1 != chars.len() => func.push(Str('^'.to_string())),
                '(' if i + 1 != chars.len() && chars[i + 1] != ')' => {
                    count += 1;
                    place_multiplier(&mut func, &find_word);
                    func.push(Str("(".to_string()))
                }
                ')' if i != 0 && chars[i - 1] != '(' => {
                    if sum == count {
                        sum = 0;
                    }
                    count -= 1;
                    func.push(Str(")".to_string()))
                }
                '|' => {
                    if i + 1 != chars.len() && chars[i + 1] == '|' && abs {
                        func.push(Str("||".to_string()));
                        i += 2;
                        continue;
                    } else if abs {
                        place_multiplier(&mut func, &find_word);
                        func.push(Str("norm".to_string()));
                        func.push(Str("(".to_string()));
                        abs = false;
                    } else {
                        func.push(Str(")".to_string()));
                        abs = true;
                    }
                }
                '!' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        func.push(Str("!=".to_string()));
                    } else if i != 0
                        && (chars[i - 1].is_alphanumeric()
                            || (!func.is_empty() && func.last().unwrap().str_is(")")
                                || func.last().unwrap().str_is("}")))
                    {
                        if let Num(a) = func.clone().last().unwrap() {
                            if a.real() < &0.0 {
                                func.pop();
                                func.push(Num(Complex::with_val(
                                    options.prec,
                                    (-a.real(), a.imag()),
                                )));
                                func.insert(func.len() - 1, Num(n1.clone()));
                                func.insert(func.len() - 1, Str("*".to_string()));
                            }
                        }
                        if func.clone().last().unwrap().str_is(")")
                            || func.last().unwrap().str_is("}")
                        {
                            let mut count = 0;
                            for (j, c) in func.iter().enumerate().rev() {
                                if let Str(s) = c {
                                    if s == "(" || s == "{" {
                                        count -= 1;
                                    } else if s == ")" || s == "}" {
                                        count += 1;
                                    }
                                }
                                if count == 0 {
                                    if j != 0 {
                                        if let Str(s) = &func[j - 1] {
                                            if s != "subfact" && s != "(" {
                                                func.insert(j - 1, Str("(".to_string()));
                                                func.insert(j - 1, Str("fact".to_string()));
                                                func.push(Str(")".to_string()));
                                                i += 1;
                                                continue 'outer;
                                            }
                                        }
                                    }
                                    func.insert(j, Str("(".to_string()));
                                    func.insert(j, Str("fact".to_string()));
                                    func.push(Str(")".to_string()));
                                    i += 1;
                                    continue 'outer;
                                }
                            }
                        }
                        func.insert(func.len() - 1, Str("fact".to_string()));
                        func.insert(func.len() - 1, Str("(".to_string()));
                        func.push(Str(")".to_string()));
                    } else if i != chars.len() - 1
                        && (chars[i + 1].is_alphanumeric()
                            || chars[i + 1] == '('
                            || chars[i + 1] == '{'
                            || chars[i + 1] == '|'
                            || chars[i + 1] == '-'
                            || chars[i + 1] == '!')
                    {
                        if neg {
                            func.push(Num(n1.clone()));
                            func.push(Str("*".to_string()));
                            neg = false;
                        }
                        func.push(Str("subfact".to_string()));
                        func.push(Str("(".to_string()));
                        count += 1;
                    }
                }
                ',' if i != 0 && i + 1 != chars.len() => func.push(Str(','.to_string())),
                '%' if i != 0 && i + 1 != chars.len() => func.push(Str('%'.to_string())),
                _ => (),
            }
        }
        i += 1;
    }
    func.extend(vec![Str(")".to_string()); count as usize]);
    if !pow.is_empty() {
        let i = pow.matches('i').count() % 4;
        pow = pow.replace('i', "");
        if pow.is_empty() {
            pow = "1".to_string();
        }
        if !func.is_empty()
            && (func.last().unwrap().num().is_ok() || func.last().unwrap().str_is(")"))
        {
            func.push(Str("^".to_string()));
        }
        func.push(Num(Complex::with_val(
            options.prec,
            match Complex::parse(pow.as_bytes()) {
                Ok(n) => n,
                Err(_) => return Err("exponent error"),
            },
        ) * Complex::with_val(options.prec, (0, 1))
            .pow(Complex::with_val(options.prec, i))));
    }
    if !exp.is_empty() {
        func.push(Str("^".to_string()));
        func.push(Num(Complex::with_val(
            options.prec,
            match Complex::parse(exp.as_bytes()) {
                Ok(n) => n,
                Err(_) => return Err("exponent error"),
            },
        )));
    }
    if !abs {
        func.push(Str(")".to_string()));
    }
    if neg {
        func.push(Num(n1));
    }
    if func.is_empty() {
        return Err("no function");
    }
    // for i in &func
    // {
    //     match i
    //     {
    //         Str(s) => println!("{}", s),
    //         Num(n) => println!("{}", n),
    //         Vector(v) => println!("{:?}", v),
    //         Matrix(m) => println!("{:?}", m),
    //     }
    // }
    Ok(func)
}
fn place_multiplier(func: &mut Vec<NumStr>, find_word: &bool) {
    if let Some(Str(s)) = func.last() {
        if !find_word && (s == ")" || s == "x" || s == "y" || s == "]" || s == "}") {
            func.push(Str('*'.to_string()))
        }
    } else if let Num(_) = func.last().unwrap_or(&Str("".to_string())) {
        func.push(Str('*'.to_string()))
    } else if let Vector(_) = func.last().unwrap_or(&Str("".to_string())) {
        func.push(Str('*'.to_string()))
    }
}
pub fn is_func(word: &str) -> bool {
    let functions: HashSet<_> = [
        "sum",
        "product",
        "prod",
        "summation",
        "cofactor",
        "cofactors",
        "cof",
        "minor",
        "minors",
        "adjugate",
        "adj",
        "inv",
        "inverse",
        "transpose",
        "trans",
        "len",
        "length",
        "wid",
        "width",
        "tr",
        "trace",
        "det",
        "determinant",
        "part",
        "norm",
        "abs",
        "normalize",
        "car",
        "cartesian",
        "polar",
        "pol",
        "angle",
        "cross",
        "proj",
        "project",
        "dot",
        "rotate",
        "sin",
        "csc",
        "cos",
        "sec",
        "tan",
        "cot",
        "asin",
        "arcsin",
        "acsc",
        "arccsc",
        "acos",
        "arccos",
        "asec",
        "arcsec",
        "atan",
        "arctan",
        "atan2",
        "acot",
        "arccot",
        "sinh",
        "csch",
        "cosh",
        "sech",
        "tanh",
        "coth",
        "asinh",
        "arcsinh",
        "acsch",
        "arccsch",
        "acosh",
        "arccosh",
        "asech",
        "arcsech",
        "atanh",
        "arctanh",
        "acoth",
        "arccoth",
        "cis",
        "ln",
        "aexp",
        "ceil",
        "floor",
        "round",
        "recip",
        "exp",
        "aln",
        "log",
        "root",
        "bi",
        "binomial",
        "gamma",
        "max",
        "min",
        "sqrt",
        "asquare",
        "abs",
        "norm",
        "deg",
        "degree",
        "rad",
        "radian",
        "gradian",
        "re",
        "real",
        "im",
        "imag",
        "sgn",
        "sign",
        "arg",
        "cbrt",
        "acube",
        "frac",
        "fract",
        "int",
        "trunc",
        "square",
        "asqrt",
        "cube",
        "acbrt",
        "fact",
        "subfact",
        "sinc",
        "conj",
        "conjugate",
        "erf",
        "erfc",
        "ai",
        "digamma",
        "zeta",
        "Γ",
        "ζ",
        "Σ",
        "Π",
    ]
    .iter()
    .cloned()
    .collect();
    functions.contains(word)
}
