use crate::{AngleType, Number, Options, Units};
use rug::{float::Constant::Pi, ops::Pow, Complex};
use std::collections::HashSet;
impl Units
{
    pub fn mul(&self, b: &Self) -> Self
    {
        Self {
            second: self.second + b.second,
            meter: self.meter + b.meter,
            kilogram: self.kilogram + b.kilogram,
            ampere: self.ampere + b.ampere,
            kelvin: self.kelvin + b.kelvin,
            mole: self.mole + b.mole,
            candela: self.candela + b.candela,
            angle: self.angle + b.angle,
            steradian: self.steradian + b.steradian,
            byte: self.byte + b.byte,
        }
    }
    pub fn div(&self, b: &Self) -> Self
    {
        Self {
            second: self.second - b.second,
            meter: self.meter - b.meter,
            kilogram: self.kilogram - b.kilogram,
            ampere: self.ampere - b.ampere,
            kelvin: self.kelvin - b.kelvin,
            mole: self.mole - b.mole,
            candela: self.candela - b.candela,
            angle: self.angle - b.angle,
            steradian: self.steradian - b.steradian,
            byte: self.byte - b.byte,
        }
    }
    pub fn pow(&self, b: f64) -> Self
    {
        Self {
            second: self.second * b,
            meter: self.meter * b,
            kilogram: self.kilogram * b,
            ampere: self.ampere * b,
            kelvin: self.kelvin * b,
            mole: self.mole * b,
            candela: self.candela * b,
            angle: self.angle * b,
            steradian: self.steradian * b,
            byte: self.byte * b,
        }
    }
    pub fn root(&self, b: f64) -> Self
    {
        Self {
            second: self.second / b,
            meter: self.meter / b,
            kilogram: self.kilogram / b,
            ampere: self.ampere / b,
            kelvin: self.kelvin / b,
            mole: self.mole / b,
            candela: self.candela / b,
            angle: self.angle / b,
            steradian: self.steradian / b,
            byte: self.byte / b,
        }
    }
    pub fn to_string(&self, options: Options) -> String
    {
        format!(
            "{}{}{}{}{}{}{}{}{}{}",
            if self.meter != 0.0
            {
                " m".to_owned()
                    + &if self.meter != 1.0
                    {
                        "^".to_owned() + &self.meter.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.second != 0.0
            {
                " s".to_owned()
                    + &if self.second != 1.0
                    {
                        "^".to_owned() + &self.second.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.kilogram != 0.0
            {
                " kg".to_owned()
                    + &if self.kilogram != 1.0
                    {
                        "^".to_owned() + &self.kilogram.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.ampere != 0.0
            {
                " A".to_owned()
                    + &if self.ampere != 1.0
                    {
                        "^".to_owned() + &self.ampere.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.kelvin != 0.0
            {
                " K".to_owned()
                    + &if self.kelvin != 1.0
                    {
                        "^".to_owned() + &self.kelvin.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.mole != 0.0
            {
                " mol".to_owned()
                    + &if self.mole != 1.0
                    {
                        "^".to_owned() + &self.mole.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.candela != 0.0
            {
                " cd".to_owned()
                    + &if self.candela != 1.0
                    {
                        "^".to_owned() + &self.candela.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.angle != 0.0
            {
                match options.deg
                {
                    AngleType::Degrees => " deg",
                    AngleType::Radians => " rad",
                    AngleType::Gradians => " grad",
                }
                .to_owned()
                    + &if self.angle != 1.0
                    {
                        "^".to_owned() + &self.angle.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.steradian != 0.0
            {
                " sr".to_owned()
                    + &if self.steradian != 1.0
                    {
                        "^".to_owned() + &self.steradian.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            },
            if self.byte != 0.0
            {
                " B".to_owned()
                    + &if self.byte != 1.0
                    {
                        "^".to_owned() + &self.byte.to_string()
                    }
                    else
                    {
                        String::new()
                    }
            }
            else
            {
                String::new()
            }
        )
    }
}
impl Default for Units
{
    fn default() -> Self
    {
        Self {
            second: 0.0,
            meter: 0.0,
            kilogram: 0.0,
            ampere: 0.0,
            kelvin: 0.0,
            mole: 0.0,
            candela: 0.0,
            angle: 0.0,
            steradian: 0.0,
            byte: 0.0,
        }
    }
}
pub fn prefixes(mut unit: String, prec: (u32, u32)) -> (String, Complex)
{
    if units().contains(unit.as_str())
    {
        return (unit, Complex::with_val(prec, 1));
    }
    let bak = unit.clone();
    let mut word = String::new();
    while !unit.is_empty() && word.len() < 7
    {
        word.push(unit.remove(0));
        match word.as_str()
        {
            "quetta" | "Q" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(30))
            }
            "ronna" | "R" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(27))
            }
            "yotta" | "Y" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(24))
            }
            "zetta" | "Z" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(21))
            }
            "exa" | "E" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(18))
            }
            "peta" | "P" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(15))
            }
            "tera" | "T" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(12))
            }
            "giga" | "G" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(9))
            }
            "mega" | "M" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(6))
            }
            "kilo" | "k" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(3))
            }
            "hecto" | "h" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(2))
            }
            "deca" | "da" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(1))
            }
            "deci" | "d" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-1))
            }
            "centi" | "c" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-2))
            }
            "milli" | "m" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-3))
            }
            "micro" | "μ" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-6))
            }
            "nano" | "n" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-9))
            }
            "pico" | "p" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-12))
            }
            "femto" | "f" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-15))
            }
            "atto" | "a" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-18))
            }
            "zepto" | "z" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-21))
            }
            "yocto" | "y" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-24))
            }
            "ronto" | "r" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-27))
            }
            "qecto" | "q" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 10).pow(-30))
            }
            "kibi" | "Ki" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(10))
            }
            "mebi" | "Mi" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(20))
            }
            "gibi" | "Gi" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(30))
            }
            "tebi" | "Ti" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(40))
            }
            "pebi" | "Pi" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(50))
            }
            "exbi" | "Ei" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(60))
            }
            "zebi" | "Zi" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(70))
            }
            "yobi" | "Yi" if units().contains(unit.as_str()) =>
            {
                return (unit, Complex::with_val(prec, 2).pow(80))
            }
            _ =>
            {}
        }
    }
    (bak, Complex::with_val(prec, 1))
}
pub fn units() -> HashSet<&'static str>
{
    [
        "m",
        "meter",
        "s",
        "second",
        "A",
        "ampere",
        "K",
        "kelvin",
        "mol",
        "mole",
        "cd",
        "month",
        "candela",
        "J",
        "joule",
        "min",
        "minute",
        "C",
        "coulomb",
        "N",
        "newton",
        "°",
        "deg",
        "degrees",
        "rad",
        "radians",
        "grad",
        "gradians",
        "hour",
        "day",
        "week",
        "Ω",
        "ohm",
        "V",
        "volt",
        "voltage",
        "Hz",
        "hertz",
        "Pa",
        "pascal",
        "W",
        "watt",
        "farad",
        "F",
        "siemens",
        "S",
        "weber",
        "Wb",
        "T",
        "tesla",
        "H",
        "henry",
        "°C",
        "°F",
        "Wh",
        "Ah",
        "celsius",
        "fahrenheit",
        "litre",
        "L",
        "lb",
        "pound",
        "inch",
        "in",
        "ft",
        "yd",
        "yard",
        "mi",
        "mile",
        "mph",
        "gram",
        "g",
        "h",
        "d",
        "lumen",
        "lm",
        "lux",
        "lx",
        "byte",
        "B",
        "gray",
        "Gy",
        "sievert",
        "Sv",
        "katal",
        "kat",
        "bit",
        "b",
        "steradian",
        "sr",
        "kph",
        "year",
        "ly",
        "nit",
        "nt",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<&str>>()
}
pub fn to_unit(unit: String, mut num: Complex, options: Options) -> (Number, Option<Number>)
{
    let mut units = Units::default();
    let mut add = None;
    match unit.as_str()
    {
        "m" | "meter" => units.meter = 1.0,
        "s" | "second" => units.second = 1.0,
        "A" | "ampere" => units.ampere = 1.0,
        "K" | "kelvin" => units.kelvin = 1.0,
        "mol" | "mole" => units.mole = 1.0,
        "cd" | "candela" => units.candela = 1.0,
        "byte" | "B" => units.byte = 1.0,
        "steradian" | "sr" => units.steradian = 1.0,
        "bit" | "b" =>
        {
            num /= 8;
            units.byte = 1.0;
        }
        "g" | "gram" =>
        {
            num *= Complex::with_val(options.prec, 10).pow(-3);
            units.kilogram = 1.0
        }
        "nit" | "nt" =>
        {
            units.candela = 1.0;
            units.meter = -2.0
        }
        "gray" | "Gy" =>
        {
            units.second = -2.0;
            units.meter = 2.0;
        }
        "sievert" | "Sv" =>
        {
            units.second = -2.0;
            units.meter = 2.0;
        }
        "katal" | "kat" =>
        {
            units.second = -1.0;
            units.mole = 1.0;
        }
        "lumen" | "lm" =>
        {
            units.steradian = 1.0;
            units.candela = 1.0;
        }
        "lux" | "lx" =>
        {
            units.steradian = 1.0;
            units.candela = 1.0;
            units.meter = -2.0;
        }
        "J" | "joule" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -2.0;
        }
        "mph" =>
        {
            num *= 201168;
            num /= 125;
            num /= 3600;
            units.meter = 1.0;
            units.second = -1.0;
        }
        "kph" =>
        {
            num /= 3.6;
            units.meter = 1.0;
        }
        "mi" | "mile" =>
        {
            num *= 201168;
            num /= 125;
            units.meter = 1.0;
        }
        "yd" | "yard" =>
        {
            num *= 1143;
            num /= 1250;
            units.meter = 1.0;
        }
        "ft" | "foot" =>
        {
            units.meter = 1.0;
            num *= 381;
            num /= 1250;
        }
        "in" | "inch" =>
        {
            units.meter = 1.0;
            num *= 127;
            num /= 5000;
        }
        "lb" | "pound" =>
        {
            units.kilogram = 1.0;
            num *= 45359237;
            num /= 100000000;
        }
        "L" | "litre" =>
        {
            num *= Complex::with_val(options.prec, 10).pow(-3);
            units.meter = 3.0;
        }
        "Hz" | "hertz" => units.second = -1.0,
        "V" | "volt" | "voltage" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -3.0;
            units.ampere = -1.0;
        }
        "°C" | "celsius" =>
        {
            units.kelvin = 1.0;
            let unit = Units {
                kelvin: 1.0,
                ..Units::default()
            };
            add = Some(Number::from(
                Complex::with_val(options.prec, 5463) / 20,
                Some(unit),
            ));
        }
        "°F" | "fahrenheit" =>
        {
            num *= 5;
            num /= 9;
            units.kelvin = 1.0;
            let unit = Units {
                kelvin: 1.0,
                ..Units::default()
            };
            add = Some(Number::from(
                Complex::with_val(options.prec, 45967) / 180,
                Some(unit),
            ));
        }
        "Wh" =>
        {
            num *= 3600;
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -2.0;
        }
        "Ah" =>
        {
            num *= 3600;
            units.ampere = 1.0;
            units.second = 1.0;
        }
        "T" | "tesla" =>
        {
            units.kilogram = 1.0;
            units.second = -2.0;
            units.ampere = -1.0;
        }
        "H" | "henry" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -2.0;
            units.ampere = -2.0;
        }
        "weber" | "Wb" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -2.0;
            units.ampere = -1.0;
        }
        "siemens" | "S" =>
        {
            units.kilogram = -1.0;
            units.meter = -2.0;
            units.second = 3.0;
            units.ampere = 2.0;
        }
        "F" | "farad" =>
        {
            units.kilogram = -1.0;
            units.meter = -2.0;
            units.second = 4.0;
            units.ampere = 2.0;
        }
        "W" | "watt" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -3.0;
        }
        "Pa" | "pascal" =>
        {
            units.kilogram = 1.0;
            units.meter = -1.0;
            units.second = -2.0;
        }
        "Ω" | "ohm" =>
        {
            units.kilogram = 1.0;
            units.meter = 2.0;
            units.second = -3.0;
            units.ampere = -2.0;
        }
        "min" | "minute" =>
        {
            units.second = 1.0;
            num *= 60;
        }
        "h" | "hour" =>
        {
            units.second = 1.0;
            num *= 3600;
        }
        "d" | "day" =>
        {
            units.second = 1.0;
            num *= 86400;
        }
        "week" =>
        {
            units.second = 1.0;
            num *= 604800;
        }
        "month" =>
        {
            num *= 2629800;
            units.second = 1.0
        }
        "year" =>
        {
            num *= 31557600;
            units.second = 1.0;
        }
        "ly" =>
        {
            num *= 9460730472580800u128;
            units.second = 1.0;
        }
        "N" | "newton" =>
        {
            units.kilogram = 1.0;
            units.meter = 1.0;
            units.second = -2.0;
        }
        "C" | "coulomb" =>
        {
            units.ampere = 1.0;
            units.second = 1.0;
        }
        "°" | "deg" | "degrees" =>
        {
            match options.deg
            {
                AngleType::Degrees =>
                {}
                AngleType::Gradians =>
                {
                    num *= 200;
                    num /= 180
                }
                AngleType::Radians => num *= Complex::with_val(options.prec, Pi) / 180,
            };
            units.angle = 1.0;
        }
        "rad" | "radians" =>
        {
            match options.deg
            {
                AngleType::Degrees => num *= 180 / Complex::with_val(options.prec, Pi),
                AngleType::Gradians => num *= 200 / Complex::with_val(options.prec, Pi),
                AngleType::Radians =>
                {}
            };
            units.angle = 1.0
        }
        "grad" | "gradians" =>
        {
            match options.deg
            {
                AngleType::Degrees =>
                {
                    num *= 180;
                    num /= 200
                }
                AngleType::Gradians =>
                {}
                AngleType::Radians => num *= Complex::with_val(options.prec, Pi) / 200,
            };
            units.angle = 1.0;
        }
        _ =>
        {}
    }
    (Number::from(num, Some(units)), add)
}
