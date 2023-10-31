use crate::Options;
use rug::{float::Constant::Pi, Float};
use std::collections::HashSet;
pub fn input_var(
    input: &str,
    vars: &[[String; 2]],
    dont_do: Option<String>,
    sumrec: &mut Vec<(i32, String)>,
    options: Options,
) -> String
{
    let chars = input
        .replace('[', "(car{")
        .replace(']', "})")
        .chars()
        .collect::<Vec<char>>();
    let mut output = String::new();
    let mut stack_end = Vec::new();
    let mut stack_start = Vec::new();
    for c in &chars
    {
        match c
        {
            '(' => stack_end.push(')'),
            '{' => stack_end.push('}'),
            ')' | '}' =>
            {
                if let Some(top) = stack_end.last()
                {
                    if top == c
                    {
                        stack_end.pop();
                    }
                }
                else
                {
                    match c
                    {
                        ')' => stack_start.push('('),
                        '}' => stack_start.push('{'),
                        _ =>
                        {}
                    }
                }
            }
            _ =>
            {}
        }
    }
    let mut input = String::new();
    while let Some(top) = stack_start.pop()
    {
        input.push(top);
    }
    for i in &chars
    {
        input.push(*i)
    }
    while let Some(top) = stack_end.pop()
    {
        input.push(top);
    }
    let chars = input.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut sum = (0, String::new());
    let mut bracket = 0;
    let functions = functions();
    'main: while i < chars.len()
    {
        let c = chars[i];
        if !c.is_alphabetic()
        {
            if c == '('
            {
                bracket += 1;
            }
            else if c == ')'
            {
                if sum.0 == bracket
                {
                    sum.0 = 0;
                    sum.1 = String::new();
                    sumrec.pop();
                }
                bracket -= 1;
            }
            output.push(c);
            i += 1;
            continue;
        }
        let count = chars[i..]
            .iter()
            .position(|x| !x.is_alphabetic())
            .unwrap_or(chars.len() - i);
        let mut word = chars[i..i + count].iter().collect::<String>();
        if (word.ends_with('x') && word != "max") || word.ends_with('y') || word.ends_with('z')
        {
            word.pop();
        }
        if matches!(
            word.as_str(),
            "sum" | "summation" | "prod" | "production" | "vec" | "mat" | "Σ" | "Π"
        ) && chars.len() > i + count + 1
        {
            let mut place = 0;
            let mut count2 = 0;
            for c in &chars[i + count + 1..]
            {
                if c == &',' && count2 == 0
                {
                    place += 1;
                }
                else if c == &'(' || c == &'{'
                {
                    count2 += 1;
                }
                else if c == &')' || c == &'}'
                {
                    if count2 == 0
                    {
                        break;
                    }
                    count2 -= 1;
                }
            }
            if place == 3
            {
                let count2 = chars[i + count + 1..]
                    .iter()
                    .position(|x| x == &',')
                    .unwrap_or(0);
                sum.0 = bracket + 1;
                if count2 != 0
                {
                    sum.1 = chars[i + count + 1..i + count + count2 + 1]
                        .iter()
                        .collect::<String>();
                }
                if !sum.1.is_empty()
                {
                    sumrec.push(sum.clone())
                }
            }
        }
        if functions.contains(word.as_str())
            && !vars.iter().any(|a| {
                if a[0].contains('(')
                {
                    a[0][..a[0].find('(').unwrap()] == word
                }
                else
                {
                    a[0] == word
                }
            })
        {
            i += word.len();
            output.push_str(&word)
        }
        else if sumrec.iter().any(|a| a.1 == word)
        {
            if matches!(chars[i - 1], '0'..='9' | ')' | '}' | ']' | 'x' | 'y' | 'z')
            {
                output.push('*')
            }
            i += word.len();
            output.push_str(&word);
            if matches!(chars[i], '0'..='9' | ')' | '}' | ']' | 'x' | 'y' | 'z')
            {
                output.push('*')
            }
        }
        else
        {
            for var in vars
            {
                let vl = var[0].chars().collect::<Vec<char>>().len();
                if var[0] != "e"
                    || (!options.small_e
                        || !(i != 0
                            && i + 1 != chars.len()
                            && chars[i - 1].is_numeric()
                            && (chars[i + 1].is_numeric() || chars[i + 1] == '-')))
                {
                    let j = i;
                    if var[0].contains('(')
                        && input.contains('(')
                        && match chars[i..].iter().position(|c| c == &'(')
                        {
                            Some(n) =>
                            {
                                chars[i..i + n].iter().collect::<String>()
                                    == var[0].split('(').next().unwrap()
                            }
                            _ => false,
                        }
                    {
                        let mut count = 0;
                        for (f, c) in chars[i..].iter().enumerate()
                        {
                            if *c == '('
                            {
                                count += 1;
                            }
                            else if *c == ')'
                            {
                                count -= 1;
                                if count == 0
                                {
                                    i += f;
                                    break;
                                }
                            }
                        }
                        if i == j
                        {
                            i = input.len() - 1
                        }
                        if chars[j..i + 1].iter().collect::<String>() == var[0]
                        {
                            output.push('(');
                            output.push_str(&input_var(&var[1], vars, None, sumrec, options));
                            output.push(')');
                            i += 1;
                            continue 'main;
                        }
                        else
                        {
                            let mut k = 0;
                            count = 0;
                            for (f, c) in chars[j + 2..].iter().enumerate()
                            {
                                if *c == ')' && count == 0
                                {
                                    k = f + j + 3;
                                    break;
                                }
                                else if *c == '('
                                {
                                    count += 1;
                                }
                                else if *c == ')'
                                {
                                    count -= 1;
                                }
                                else if f + j + 3 == chars.len()
                                {
                                    k = f + j + 4;
                                    break;
                                }
                            }
                            if k == 0
                            {
                                continue;
                            }
                            let v = var[0].chars().collect::<Vec<char>>();
                            if input.contains(',') && var[0].contains(',') && chars.len() > 4
                            {
                                output.push('(');
                                let mut temp = &chars
                                    [j + var[0].chars().position(|c| c == '(').unwrap() + 1..i + 1];
                                if temp.ends_with(&[')'])
                                {
                                    temp = &temp[..temp.len() - 1];
                                }
                                let mut commas = Vec::new();
                                count = 0;
                                for (f, c) in temp.iter().enumerate()
                                {
                                    if c == &'(' || c == &'{' || c == &'['
                                    {
                                        count += 1;
                                    }
                                    else if c == &')' || c == &'}' || c == &']'
                                    {
                                        count -= 1;
                                    }
                                    else if c == &',' && count == 0
                                    {
                                        commas.push(f);
                                    }
                                }
                                if commas.len() == var[0].matches(',').count()
                                {
                                    let mut start = 0;
                                    let mut split = Vec::new();
                                    for end in commas
                                    {
                                        split.push(&temp[start..end]);
                                        start = end + 1;
                                    }
                                    split.push(&temp[start..]);
                                    let mut vars = vars.to_vec();
                                    let mut func_vars: Vec<String> = Vec::new();
                                    start = 0;
                                    for (f, c) in v.iter().enumerate()
                                    {
                                        if c == &'(' || c == &'{' || c == &'['
                                        {
                                            if count == 0
                                            {
                                                start = f + 1;
                                            }
                                            count += 1;
                                        }
                                        else if c == &')' || c == &'}' || c == &']'
                                        {
                                            count -= 1;
                                            if count == 0
                                            {
                                                func_vars.push(v[start..f].iter().collect());
                                            }
                                        }
                                        else if c == &',' && count == 1
                                        {
                                            func_vars.push(v[start..f].iter().collect());
                                            start = f + 1;
                                        }
                                    }
                                    for (var, func_var) in split.iter().zip(func_vars)
                                    {
                                        for (i, j) in vars.iter().enumerate()
                                        {
                                            if j[0].chars().count() <= func_var.len()
                                            {
                                                vars.insert(
                                                    i,
                                                    [func_var, var.iter().collect::<String>()],
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    output.push_str(&input_var(
                                        &var[1], &vars, None, sumrec, options,
                                    ));
                                    output.push(')');
                                    i += 1;
                                    continue 'main;
                                }
                            }
                            else
                            {
                                output.push('(');
                                let mut temp = &chars[j
                                    + var[0].split('(').next().unwrap().chars().count()
                                    + 1
                                    ..i + 1];
                                if temp.ends_with(&[')'])
                                {
                                    temp = &temp[..temp.len() - 1];
                                }
                                let l = v[var[0].find('(').unwrap() + 1..v.len() - 1]
                                    .iter()
                                    .collect::<String>();
                                let mut vars = vars.to_vec();
                                for (i, j) in vars.iter().enumerate()
                                {
                                    if j[0].chars().count() <= l.len()
                                    {
                                        vars.insert(i, [l, temp.iter().collect::<String>()]);
                                        break;
                                    }
                                }
                                output.push_str(&input_var(&var[1], &vars, None, sumrec, options));
                                output.push(')');
                                i += 1;
                                continue 'main;
                            }
                        }
                    }
                    else if i + vl <= chars.len()
                        && chars[i..i + vl].iter().collect::<String>() == var[0]
                    {
                        if let Some(ref n) = dont_do
                        {
                            if &var[0] == n
                            {
                                return "".to_string();
                            }
                        }
                        i += vl;
                        output.push('(');
                        output.push_str(&input_var(
                            &var[1],
                            vars,
                            Some(var[0].clone()),
                            sumrec,
                            options,
                        ));
                        output.push(')');
                        continue 'main;
                    }
                }
            }
            if (i == 0 || chars[i - 1] != ' ' || c != ' ')
                && (if options.small_e
                {
                    matches!(c, 'x' | 'y' | 'i' | 'e')
                }
                else
                {
                    matches!(c, 'x' | 'y' | 'i' | 'E')
                } || !c.is_alphabetic())
            {
                output.push(c);
            }
            else if c == 'z'
            {
                output.push_str("(x+y*i)")
            }
            i += 1;
        }
    }
    if output.is_empty()
    {
        input.to_string()
    }
    else
    {
        output
    }
}
pub fn functions() -> HashSet<&'static str>
{
    [
        "i",
        "rnd",
        "inf",
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
        "grad",
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
        "sort",
        "Γ",
        "ζ",
        "Σ",
        "Π",
        "factor",
        "factors",
        "vec",
        "mat",
        "prime",
        "add",
        "reverse",
        "link",
        "flatten",
        "I",
        "P",
        "C",
        "split",
        "slog",
        "doublefact",
        "mean",
        "median",
        "mode",
        "quad",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<&str>>()
}
pub fn get_cli_vars(options: Options, args: &[String]) -> Vec<[String; 2]>
{
    let mut vars = vec![
        ["ec".to_string(), "1.602176634e-19".to_string()],
        ["kB".to_string(), "1.380649e-23".to_string()],
        ["me".to_string(), "9.1093837015e-31".to_string()],
        ["mn".to_string(), "1.67492749804e-27".to_string()],
        ["mp".to_string(), "1.67262192369e-27".to_string()],
        ["Na".to_string(), "6.02214076e23".to_string()],
        ["c".to_string(), "299792458".to_string()],
        ["G".to_string(), "6.67430e-11".to_string()],
        ["g".to_string(), "9.80665".to_string()],
        ["h".to_string(), "6.62607015e-34".to_string()],
        ["k".to_string(), "8.9875517923e9".to_string()],
        ["R".to_string(), "8.31446261815324".to_string()],
    ];
    for i in args
    {
        if i.contains("phi")
        {
            let phi: Float = (1 + Float::with_val(options.prec, 5).sqrt()) / 2;
            vars.push(["phi".to_string(), phi.to_string()])
        }
        if i.contains("tau")
        {
            let pi = Float::with_val(options.prec, Pi);
            let tau: Float = pi.clone() * 2;
            vars.push(["tau".to_string(), tau.to_string()])
        }
        if i.contains("pi")
        {
            let pi = Float::with_val(options.prec, Pi);
            vars.push(["pi".to_string(), pi.to_string()])
        }
        if i.contains('φ')
        {
            let phi: Float = (1 + Float::with_val(options.prec, 5).sqrt()) / 2;
            vars.push(["φ".to_string(), phi.to_string()])
        }
        if i.contains('π')
        {
            let pi = Float::with_val(options.prec, Pi);
            vars.push(['π'.to_string(), pi.to_string()])
        }
        if i.contains('τ')
        {
            let pi = Float::with_val(options.prec, Pi);
            let tau: Float = pi.clone() * 2;
            vars.push(['τ'.to_string(), tau.to_string()])
        }
        if i.contains('e')
        {
            let e = Float::with_val(options.prec, 1).exp();
            vars.push(["e".to_string(), e.to_string()])
        }
    }
    vars.iter()
        .map(|a| {
            if options.small_e
            {
                a.clone()
            }
            else
            {
                [a[0].clone(), a[1].replace('e', "E")]
            }
        })
        .collect()
}
pub fn get_vars(options: Options) -> Vec<[String; 2]>
{
    let pi = Float::with_val(options.prec, Pi);
    let tau: Float = pi.clone() * 2;
    let phi: Float = (1 + Float::with_val(options.prec, 5).sqrt()) / 2;
    let e = Float::with_val(options.prec, 1).exp();
    vec![
        ["phi".to_string(), phi.to_string()],
        ["tau".to_string(), tau.to_string()],
        ["ec".to_string(), "1.602176634e-19".to_string()],
        ["kB".to_string(), "1.380649e-23".to_string()],
        ["me".to_string(), "9.1093837015e-31".to_string()],
        ["mn".to_string(), "1.67492749804e-27".to_string()],
        ["mp".to_string(), "1.67262192369e-27".to_string()],
        ["Na".to_string(), "6.02214076e23".to_string()],
        ["pi".to_string(), pi.to_string()],
        ["c".to_string(), "299792458".to_string()],
        ["e".to_string(), e.to_string()],
        ["G".to_string(), "6.67430e-11".to_string()],
        ["g".to_string(), "9.80665".to_string()],
        ["h".to_string(), "6.62607015e-34".to_string()],
        ["k".to_string(), "8.9875517923e9".to_string()],
        ["R".to_string(), "8.31446261815324".to_string()],
        ["φ".to_string(), phi.to_string()],
        ["π".to_string(), pi.to_string()],
        ["τ".to_string(), tau.to_string()],
    ]
    .iter()
    .map(|a| {
        if options.small_e
        {
            a.clone()
        }
        else
        {
            [a[0].clone(), a[1].replace('e', "E")]
        }
    })
    .collect()
}