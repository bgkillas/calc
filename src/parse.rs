use std::f64::consts::{E, PI, TAU};
use crate::math::Complex;
use crate::math::Complex::{Num, Str};
pub fn get_func(input:&str) -> Result<Vec<Complex>, ()>
{
    let mut count:i32 = 0;
    let mut exp = 0;
    let mut func:Vec<Complex> = Vec::new();
    let mut word = String::new();
    let mut find_word = false;
    let mut abs = true;
    let mut neg = false;
    let mut i = 0;
    let chars = input.chars().collect::<Vec<char>>();
    let (mut c, mut deci);
    while i < input.len()
    {
        c = chars[i];
        if c.is_numeric()
        {
            if !word.is_empty()
            {
                find_word = false;
                func.push(Str(word.clone()));
                word.clear();
            }
            place_multiplier(&mut func, &find_word);
            if neg
            {
                word.push('-');
                neg = false;
            }
            deci = false;
            for c in chars[i..].iter()
            {
                match c
                {
                    '0'..='9' =>
                    {
                        word.push(*c);
                    }
                    '.' if !deci =>
                    {
                        deci = true;
                        word.push(*c);
                    }
                    _ => break,
                }
                i += 1;
            }
            func.push(Num((word.parse::<f64>().unwrap_or(0.0), 0.0)));
            word.clear();
            continue;
        }
        else if c.is_ascii_alphabetic()
        {
            if find_word && (!(c == 'x' || c == 'y') || (chars.len() - 1 != i && chars[i + 1] == 'p'))
            {
                word.push(c);
            }
            else
            {
                match c
                {
                    'E' =>
                    {
                        place_multiplier(&mut func, &find_word);
                        if neg
                        {
                            func.push(Num((-1.0, 0.0)));
                            func.push(Str('*'.to_string()));
                            neg = false;
                        }
                        func.push(Num((10.0, 0.0)));
                        func.push(Str('^'.to_string()));
                    }
                    'x' | 'y' =>
                    {
                        place_multiplier(&mut func, &find_word);
                        if neg
                        {
                            func.push(Num((-1.0, 0.0)));
                            func.push(Str('*'.to_string()));
                            neg = false;
                        }
                        if !word.is_empty()
                        {
                            find_word = false;
                            func.push(Str(word.clone()));
                            word.clear();
                        }
                        func.push(Str(c.to_string()));
                    }
                    'i' =>
                    {
                        place_multiplier(&mut func, &find_word);
                        if i != 0 && chars[i - 1] == 'p'
                        {
                            func.push(Num((if neg { -PI } else { PI }, 0.0)));
                            neg = false;
                        }
                        else if i + 1 != chars.len() && (chars[i + 1] == 'n' || chars[i + 1] == 'm')
                        {
                            word.push(c);
                            find_word = true;
                        }
                        else
                        {
                            func.push(Num((0.0, if neg { -1.0 } else { 1.0 })));
                            neg = false;
                        }
                    }
                    'e' =>
                    {
                        place_multiplier(&mut func, &find_word);
                        if i + 2 < chars.len() && (chars[i + 1] == 'x' || chars[i + 1] == 'y') && chars[i + 2] == 'p'
                        {
                            word.push(c);
                            find_word = true;
                        }
                        else
                        {
                            func.push(Num((if neg { -E } else { E }, 0.0)));
                            neg = false;
                        }
                    }
                    'p' =>
                    {
                        if find_word
                        {
                            word.push(c);
                        }
                    }
                    't' =>
                    {
                        if find_word || (i + 2 < chars.len() && chars[i + 2] != 'u')
                        {
                            place_multiplier(&mut func, &find_word);
                            word.push(c);
                            find_word = true;
                        }
                    }
                    'a' =>
                    {
                        if find_word || (i + 1 != chars.len() && chars[i + 1] != 'u')
                        {
                            place_multiplier(&mut func, &find_word);
                            word.push(c);
                            find_word = true;
                        }
                    }
                    'u' =>
                    {
                        func.push(Num((if neg { -TAU } else { TAU }, 0.0)));
                        neg = false;
                    }
                    _ =>
                    {
                        place_multiplier(&mut func, &find_word);
                        word.push(c);
                        find_word = true;
                    }
                }
            }
        }
        else
        {
            if !word.is_empty()
            {
                find_word = false;
                if i + 4 < chars.len() && chars[i] == '^' && chars[i + 1] == '(' && chars[i + 2] == '-' && chars[i + 3] == '1' && chars[i + 4] == ')'
                {
                    word.insert(0, 'a');
                    func.push(Str(word.clone()));
                    word.clear();
                    i += 5;
                    continue;
                }
                if i + 2 < chars.len() && chars[i] == '^' && chars[i + 1] == '-' && chars[i + 2] == '1'
                {
                    word.insert(0, 'a');
                    func.push(Str(word.clone()));
                    word.clear();
                    i += 3;
                    continue;
                }
                if i + 1 < chars.len() && chars[i] == '^' && chars[i + 1].is_ascii_digit()
                {
                    func.push(Str(word.clone()));
                    word.clear();
                    exp = chars[i + 1].to_string().parse::<u8>().unwrap();
                    i += 2;
                    continue;
                }
                func.push(Str(word.clone()));
                word.clear();
            }
            if exp != 0 && c != '(' && c != ')'
            {
                func.push(Str("^".to_string()));
                func.push(Num((exp.into(), 0.0)));
                exp = 0;
            }
            match c
            {
                '*' if i != 0 && i != chars.len() =>
                {
                    if chars.len() != i + 1 && chars[i + 1] == '*'
                    {
                        func.push(Str("^".to_string()));
                        i += 1;
                    }
                    else
                    {
                        func.push(Str('*'.to_string()));
                    }
                }
                '/' if i != 0 && i != chars.len() => func.push(Str('/'.to_string())),
                '+' if i != 0 && i != chars.len() => func.push(Str('+'.to_string())),
                '-' if i != chars.len() =>
                {
                    if i == 0 || !(chars[i - 1].is_ascii_alphanumeric() || chars[i - 1] == ')')
                    {
                        if i + 1 != chars.len() && (chars[i + 1] == '(' || chars[i + 1] == '-')
                        {
                            func.push(Num((-1.0, 0.0)));
                            func.push(Str("*".to_string()));
                        }
                        else
                        {
                            neg = true;
                        }
                    }
                    else
                    {
                        func.push(Str('-'.to_string()));
                    }
                }
                '^' if i != 0 && i != chars.len() => func.push(Str('^'.to_string())),
                '(' =>
                {
                    count += 1;
                    place_multiplier(&mut func, &find_word);
                    func.push(Str("(".to_string()))
                }
                ')' =>
                {
                    count -= 1;
                    func.push(Str(")".to_string()))
                }
                '|' =>
                {
                    if abs
                    {
                        func.push(Str("abs".to_string()));
                        func.push(Str("(".to_string()));
                    }
                    else
                    {
                        func.push(Str(")".to_string()));
                    }
                    abs = !abs;
                }
                '!' =>
                {
                    if i != 0 && (chars[i - 1].is_ascii_digit() || chars[i - 1] == 'x' || chars[i - 1] == 'y')
                    {
                        func.insert(func.len() - 1, Str("fact".to_string()));
                        func.insert(func.len() - 1, Str("(".to_string()));
                        func.push(Str(")".to_string()));
                    }
                    else if i != chars.len() - 1 && (chars[i + 1].is_ascii_digit() || chars[i + 1] == 'x' || chars[i + 1] == 'y')
                    {
                        func.push(Str("subfact".to_string()));
                        func.push(Str("(".to_string()));
                        count += 1;
                    }
                }
                'π' =>
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Num((if neg { -PI } else { PI }, 0.0)));
                    neg = false;
                }
                'τ' =>
                {
                    place_multiplier(&mut func, &find_word);
                    func.push(Num((if neg { -TAU } else { TAU }, 0.0)));
                    neg = false;
                }
                ',' => func.push(Str(','.to_string())),
                '%' if i != 0 && i != chars.len() => func.push(Str('%'.to_string())),
                _ => (),
            }
        }
        i += 1;
    }
    let bracket = Str(if count > 0 { ")".to_string() } else { "(".to_string() });
    for _ in 0..count.abs()
    {
        func.push(bracket.to_owned());
    }
    if !abs
    {
        func.push(Str(")".to_string()))
    }
    if exp != 0
    {
        func.push(Str("^".to_string()));
        func.push(Num((exp.into(), 0.0)));
    }
    if !word.is_empty()
    {
        func.push(Str(word.clone()));
        word.clear();
    }
    if func.is_empty()
    {
        return Err(());
    }
    Ok(func)
}
// noinspection RsTypeCheck
fn place_multiplier(func:&mut Vec<Complex>, find_word:&bool)
{
    if let Some(Str(s)) = func.last()
    {
        if !find_word && (s == ")" || s == "x" || s == "y")
        {
            func.push(Str('*'.to_string()))
        }
    }
    else if let Num(_) = func.last().unwrap_or(&Str("".to_string()))
    {
        func.push(Str('*'.to_string()))
    }
}
pub fn input_var(input:&str, vars:&[Vec<char>]) -> String
{
    let chars = input.chars().collect::<Vec<char>>();
    let mut output = String::new();
    let (mut split, mut var_name, mut not_pushed, mut var_value, mut v, mut c, mut k, mut f);
    let mut i = 0;
    while i < chars.len()
    {
        c = chars[i];
        not_pushed = true;
        for var in vars
        {
            if c != var[0]
            {
                continue;
            }
            if var[1] == '('
            {
                v = var.iter().collect::<String>();
                split = v.split('=');
                var_name = split.next().unwrap().to_string();
                var_value = split.next().unwrap().to_string();
                if !input.contains('=') && input.len() > i + var_name.len() && input[i..i + var_name.len()] == var_name
                {
                    not_pushed = false;
                    output.push('(');
                    output.push_str(&var_value);
                    output.push(')');
                    i += var_name.len() - 1;
                }
                if (i == 0 || !chars[i - 1].is_ascii_alphabetic()) && i + 2 < chars.len() && chars[i + 1] == '('
                {
                    k = 0;
                    for (j, c) in chars[i + 2..].iter().enumerate()
                    {
                        if *c == ')'
                        {
                            k = j + i + 3;
                            break;
                        }
                        else if j + i + 3 == chars.len()
                        {
                            k = j + i + 4;
                            break;
                        }
                    }
                    if k == 0
                    {
                        continue;
                    }
                    f = 0;
                    for (j, c) in chars[i + 2..].iter().enumerate()
                    {
                        if *c == ','
                        {
                            f = j + i + 2;
                            break;
                        }
                    }
                    if f != 0 && k > f && chars.len() > 4
                    {
                        not_pushed = false;
                        output.push('(');
                        output.push_str(&var_value.replace(var[2], &format!("({})", chars[i + 2..f].iter().collect::<String>()))
                                                  .replace(var[4], &format!("({})", chars[f + 1..k - 1].iter().collect::<String>())));
                        output.push(')');
                        i += k - 1;
                    }
                    else
                    {
                        not_pushed = false;
                        output.push('(');
                        output.push_str(&var_value.replace(var[2], &format!("({})", chars[i + 2..k - 1].iter().collect::<String>())));
                        output.push(')');
                        i += k - 1;
                    }
                }
            }
            else if (i == 0 || !chars[i - 1].is_ascii_alphabetic()) && (i == chars.len() - 1 || !chars[i + 1].is_ascii_alphabetic())
            {
                not_pushed = false;
                output.push('(');
                output.push_str(&var[2..].iter().collect::<String>());
                output.push(')');
            }
        }
        if not_pushed
        {
            output.push(c);
        }
        i += 1;
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