use std::str::FromStr;
use rug::{Complex, Float, Integer};
use rug::float::Constant::Pi;
use crate::fraction::fraction;
use crate::math::{do_math, NumStr, to_polar};
use crate::math::NumStr::{Num, Str, Vector};
use crate::parse::get_func;
use crate::PrintOptions;
pub fn print_answer(input:&str, func:Vec<NumStr>, print_options:PrintOptions, prec:u32)
{
    if input.contains('#')
       || (input.contains('x') && !input.contains("exp") && !input.contains("}x{") && !input.contains("]x["))
       || input.contains('y')
       || (input.contains('z') && !input.contains("zeta"))
       || (input.contains('=') && !(input.contains("!=") || input.contains("==") || input.contains(">=") || input.contains("<=")))
    {
        return;
    }
    let num = match do_math(func, print_options.deg, prec)
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("0");
            return;
        }
    };
    if let Num(n) = num
    {
        let sign = if n.real() != &0.0 && n.imag().is_sign_positive() { "+" } else { "" }.to_owned();
        let a = get_output(&print_options, &n, sign);
        print!("{}{}{}", a.0, a.1, if print_options.color { "\x1b[0m" } else { "" });
    }
    else if let Vector(mut v) = num
    {
        if print_options.polar
        {
            v = to_polar(&v,
                         if print_options.deg
                         {
                             Complex::with_val(prec, 180.0) / Complex::with_val(prec, Pi)
                         }
                         else
                         {
                             Complex::with_val(prec, 1.0)
                         });
        }
        let mut output = if print_options.polar { "[" } else { "{" }.to_string();
        let mut out;
        let mut sign;
        for (k, i) in v.iter().enumerate()
        {
            sign = if i.real() != &0.0 && i.imag() != &0.0 { "+" } else { "" }.to_owned();
            out = get_output(&print_options, i, sign);
            output += out.0.as_str();
            output += out.1.as_str();
            if print_options.color
            {
                output += "\x1b[0m";
            }
            if k == v.len() - 1
            {
                output += if print_options.polar { "]" } else { "}" };
            }
            else
            {
                output += ",";
            }
        }
        print!("{}{}", output, if print_options.color { "\x1b[0m" } else { "" });
    }
}
pub fn print_concurrent(unmodified_input:&str, input:&str, print_options:PrintOptions, prec:u32, start:usize, end:usize) -> bool
{
    if input.contains('#')
       || (input.contains('x') && !input.contains("exp") && !input.contains("}x{") && !input.contains("]x["))
       || input.contains('y')
       || (input.contains('z') && !input.contains("zeta"))
       || (input.contains('=') && !(input.contains("!=") || input.contains("==") || input.contains(">=") || input.contains("<=")))
    {
        print!("\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}{}\x1b[0m",
               if print_options.prompt
               {
                   if print_options.color
                   {
                       "\x1b[94m> \x1b[96m"
                   }
                   else
                   {
                       "> "
                   }
               }
               else if print_options.color
               {
                   "\x1b[96m"
               }
               else
               {
                   ""
               },
               &unmodified_input[start..end]);
        return false;
    }
    let func = match get_func(input, prec, print_options.deg)
    {
        Ok(f) => f,
        Err(_) =>
        {
            print!("\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}{}\x1b[0m",
                   if print_options.prompt
                   {
                       if print_options.color
                       {
                           "\x1b[94m> \x1b[96m"
                       }
                       else
                       {
                           "> "
                       }
                   }
                   else if print_options.color
                   {
                       "\x1b[96m"
                   }
                   else
                   {
                       ""
                   },
                   &unmodified_input[start..end]);
            return false;
        }
    };
    let mut frac = false;
    let mut num = do_math(func, print_options.deg, prec).unwrap_or(Num(Complex::with_val(256, 0.0)));
    if let Str(_) = num
    {
        num = Num(Complex::with_val(256, 0.0));
    }
    if let Num(n) = num
    {
        let a = n.real().to_f64();
        let b = n.imag().to_f64();
        let fa = fraction(n.real().clone(), print_options.tau, print_options.decimal_places);
        let fb = fraction(n.imag().clone(), print_options.tau, print_options.decimal_places);
        let sign = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned();
        let (frac_a, frac_b) = match (!fa.is_empty(), !fb.is_empty())
        {
            (true, true) =>
            {
                frac = true;
                (if a == 0.0 && b != 0.0 { "".to_string() } else { fa },
                 if b == 0.0
                 {
                     "".to_string()
                 }
                 else
                 {
                     sign.clone() + fb.as_str() + if print_options.color { "\x1b[93mi" } else { "i" }
                 })
            }
            (true, _) =>
            {
                frac = true;
                (if a == 0.0 && b != 0.0 { "".to_string() } else { fa },
                 if b == 0.0
                 {
                     "".to_string()
                 }
                 else
                 {
                     sign.clone() + b.to_string().as_str() + if print_options.color { "\x1b[93mi" } else { "i" }
                 })
            }
            (_, true) =>
            {
                frac = true;
                (if a == 0.0 && b != 0.0 { "".to_string() } else { a.to_string() },
                 if b == 0.0
                 {
                     "".to_string()
                 }
                 else
                 {
                     sign.clone() + fb.as_str() + if print_options.color { "\x1b[93mi" } else { "i" }
                 })
            }
            _ => ("".to_string(), "".to_string()),
        };
        let output = get_output(&print_options, &n, sign);
        if frac && !print_options.frac
        {
            frac = false;
        }
        print!("{}\x1b[0m\n\x1B[2K\x1B[1G{}{}\x1b[A{}\x1B[2K\x1B[1G{}{}\x1b[0m",
               if frac
               {
                   format!("\x1b[0m\n\x1B[2K\x1B[1G{}{}", frac_a, frac_b)
               }
               else
               {
                   "\n\n\x1B[2K\x1B[1G\x1b[A\x1b[A".to_string()
               },
               output.0,
               output.1,
               if frac { "\x1b[A" } else { "" },
               if print_options.prompt
               {
                   if print_options.color
                   {
                       "\x1b[94m> \x1b[96m"
                   }
                   else
                   {
                       "> "
                   }
               }
               else if print_options.color
               {
                   "\x1b[96m"
               }
               else
               {
                   ""
               },
               &unmodified_input[start..end]);
    }
    else if let Vector(mut v) = num
    {
        if print_options.polar
        {
            v = to_polar(&v,
                         if print_options.deg
                         {
                             Complex::with_val(prec, 180.0) / Complex::with_val(prec, Pi)
                         }
                         else
                         {
                             Complex::with_val(prec, 1.0)
                         });
        }
        let mut output = if print_options.polar { "[" } else { "{" }.to_string();
        let mut frac_out = if print_options.polar { "[" } else { "{" }.to_string();
        let mut out;
        let mut sign;
        let mut frac_temp;
        for (k, i) in v.iter().enumerate()
        {
            sign = if i.real() != &0.0 && i.imag() != &0.0 { "+" } else { "" }.to_owned();
            out = get_output(&print_options, i, sign);
            frac_temp = fraction(i.real().clone(), print_options.tau, print_options.decimal_places);
            frac_out += if !frac_temp.is_empty() { &frac_temp } else { &out.0 };
            frac_temp = fraction(i.imag().clone(), print_options.tau, print_options.decimal_places);
            frac_out += if !frac_temp.is_empty() { &frac_temp } else { &out.1 };
            output += &out.0;
            output += &out.1;
            if print_options.color
            {
                output += "\x1b[0m";
                frac_out += "\x1b[0m";
            }
            if k == v.len() - 1
            {
                output += if print_options.polar { "]" } else { "}" };
                frac_out += if print_options.polar { "]" } else { "}" };
            }
            else
            {
                output += ",";
                frac_out += ",";
            }
        }
        if frac_out != output
        {
            frac = true;
        }
        if frac && !print_options.frac
        {
            frac = false;
        }
        print!("{}\x1b[0m\n\x1B[2K\x1B[1G{}\x1b[A{}\x1B[2K\x1B[1G{}{}\x1b[0m",
               if frac
               {
                   format!("\x1b[0m\n\x1B[2K\x1B[1G{}", frac_out)
               }
               else
               {
                   "\n\n\x1B[2K\x1B[1G\x1b[A\x1b[A".to_string()
               },
               output,
               if frac { "\x1b[A" } else { "" },
               if print_options.prompt
               {
                   if print_options.color
                   {
                       "\x1b[94m> \x1b[96m"
                   }
                   else
                   {
                       "> "
                   }
               }
               else if print_options.color
               {
                   "\x1b[96m"
               }
               else
               {
                   ""
               },
               &unmodified_input[start..end]);
    }
    frac
}
fn get_output(print_options:&PrintOptions, num:&Complex, sign:String) -> (String, String)
{
    let mut n;
    if print_options.base != 10
    {
        (if num.real() != &0.0
         {
             n = remove_trailing_zeros(&num.real().to_string_radix(print_options.base as i32, None));
             if n.contains('e')
             {
                 n
             }
             else
             {
                 n.trim_end_matches('0').trim_end_matches('.').to_owned()
             }
         }
         else if num.imag() == &0.0
         {
             "0".to_owned()
         }
         else
         {
             "".to_owned()
         },
         if num.imag() != &0.0
         {
             n = remove_trailing_zeros(&num.imag().to_string_radix(print_options.base as i32, None));
             sign + &if n.contains('e') { n } else { n.trim_end_matches('0').trim_end_matches('.').to_owned() } + if print_options.color { "\x1b[93mi" } else { "i" }
         }
         else
         {
             "".to_string()
         })
    }
    else if print_options.sci
    {
        let dec = if print_options.decimal_places == 0 { 1 } else { print_options.decimal_places };
        (if num.real() != &0.0
         {
             add_commas(&remove_trailing_zeros(&format!("{:.dec$e}{}", num.real(), if print_options.color { "\x1b[0m" } else { "" })),
                        print_options.comma).replace("e0", "")
                                            .replace('e', if print_options.color { "\x1b[92mE" } else { "E" })
         }
         else if num.imag() == &0.0
         {
             "0".to_owned()
         }
         else
         {
             "".to_owned()
         },
         if num.imag() != &0.0
         {
             add_commas(&remove_trailing_zeros(&format!("{}{:.dec$e}{}", sign, num.imag(), if print_options.color { "\x1b[93mi" } else { "i" })),
                        print_options.comma).replace("e0", "")
                                            .replace('e', if print_options.color { "\x1b[92mE" } else { "E" })
         }
         else
         {
             "".to_owned()
         })
    }
    else
    {
        n = add_commas(&to_string(num.real(), print_options.decimal_places), print_options.comma);
        let sign = if n == "0" { "".to_string() } else { sign };
        let im = add_commas(&to_string(num.imag(), print_options.decimal_places), print_options.comma);
        (if n == "0" && im != "0" { "".to_string() } else { n },
         if im == "0"
         {
             "".to_string()
         }
         else
         {
             sign + &im + if print_options.color { "\x1b[93mi" } else { "i" }
         })
    }
}
fn to_string(num:&Float, decimals:usize) -> String
{
    let (neg, mut str, exp) = num.to_sign_string_exp(10, None);
    let mut neg = if neg { "-" } else { "" };
    if exp.is_none()
    {
        return if str == "0" { "0".to_string() } else { format!("{}{}", neg, str) };
    }
    let exp = exp.unwrap();
    if str.len() as i32 == exp
    {
        return if str == "0" { "0".to_string() } else { format!("{}{}", neg, str) };
    }
    if exp > str.len() as i32
    {
        str.push_str(&"0".repeat(exp as usize - str.len()));
    }
    let mut zeros = String::new();
    if exp < 0
    {
        zeros = "0".repeat(-exp as usize);
        str.insert_str(0, &zeros);
        str.insert(1, '.');
    }
    else
    {
        str.insert(exp as usize, '.');
    }
    let mut split = str.split('.');
    let mut l = split.next().unwrap().to_string();
    let mut r = split.next().unwrap().to_string();
    if r.is_empty()
    {
        return if str == "0" { "0".to_string() } else { format!("{}{}", neg, l) };
    }
    if r.len() > decimals
    {
        r.insert(decimals, '.');
    }
    let mut d = Float::with_val(num.prec(), Float::parse(&r).unwrap()).to_integer().unwrap();
    if exp > 0
    {
        zeros = "0".repeat(r.to_string().len() - r.to_string().trim_start_matches('0').len());
        if d.to_string() == 10.0f64.powi(decimals as i32 - 1).to_string()
        {
            zeros.pop();
        }
    }
    if zeros.is_empty() && d.to_string().trim_end_matches('0') == "1" && r.starts_with('9')
    {
        let t:Float = Float::with_val(num.prec(), Float::parse(if l.is_empty() { "0" } else { &l }).unwrap()) + 1;
        l = t.to_integer().unwrap().to_string();
        d = Integer::new();
    }
    if d.to_string() == "0" && (l.is_empty() || l == "0")
    {
        neg = ""
    }
    if decimals == 0
    {
        if zeros.is_empty() && d.to_string().chars().next().unwrap().to_digit(10).unwrap() == 1
        {
            format!("{}{}", neg, Integer::from_str(&l).unwrap_or(Integer::new()) + 1)
        }
        else
        {
            format!("{}{}", neg, if l.is_empty() { "0" } else { &l })
        }
    }
    else
    {
        format!("{}{}.{}{}", neg, if l.is_empty() { "0" } else { &l }, zeros, d).trim_end_matches('0')
                                                                                .trim_end_matches('.')
                                                                                .to_string()
    }
}
fn add_commas(input:&str, commas:bool) -> String
{
    if !commas
    {
        return input.to_owned();
    }
    let mut split = input.split('.');
    let mut result = String::new();
    let mut count = 0;
    let mut exp = false;
    for c in split.next().unwrap().chars().rev()
    {
        if c == 'e'
        {
            exp = true;
        }
        if count == 3 && !exp
        {
            result.push(',');
            count = 0;
        }
        result.push(c);
        count += 1;
    }
    if split.clone().count() == 1
    {
        let mut result = result.chars().rev().collect::<String>();
        result.push('.');
        count = 0;
        for c in split.next().unwrap().chars()
        {
            if c == 'e'
            {
                exp = true;
            }
            if count == 3 && !exp
            {
                result.push(',');
                count = 0;
            }
            result.push(c);
            count += 1;
        }
        return result;
    }
    result.chars().rev().collect::<String>()
}
fn remove_trailing_zeros(input:&str) -> String
{
    let chars = input.chars();
    let mut result = Vec::new();
    let mut found = false;
    let mut non_zero = false;
    for c in chars.rev()
    {
        if !non_zero && found && (c == '0' || c == '.')
        {
            continue;
        }
        else
        {
            non_zero = true;
        }
        if c == 'e'
        {
            found = true;
            non_zero = false;
        }
        result.push(c);
    }
    result.iter().rev().collect::<String>()
}