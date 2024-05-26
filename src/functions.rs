use std::collections::HashSet;
pub fn functions() -> HashSet<&'static str>
{
    [
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
        "ceil",
        "floor",
        "round",
        "recip",
        "exp",
        "log",
        "root",
        "bi",
        "binomial",
        "gamma",
        "max",
        "min",
        "sqrt",
        "abs",
        "norm",
        "re",
        "real",
        "im",
        "imag",
        "sgn",
        "sign",
        "arg",
        "cbrt",
        "frac",
        "fract",
        "int",
        "trunc",
        "square",
        "cube",
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
        "all",
        "any",
        "eigenvalues",
        "eigenvectors",
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
        "quadratic",
        "cubic",
        "standarddeviation",
        "variance",
        "tolist",
        "tofreq",
        "σ",
        "var",
        "quartiles",
        "percentile",
        "percentilerank",
        "normD",
        "normP",
        "piecewise",
        "pw",
        "is_prime",
        "isprime",
        "dice",
        "μ",
        "W",
        "productlog",
        "lambertw",
        "ssrt",
        "gcd",
        "gcf",
        "lcm",
        "multinomial",
        "Β",
        "B",
        "beta",
        "betaP",
        "betaC",
        "slope",
        "lim",
        "limit",
        "D",
        "area",
        "∫",
        "integrate",
        "arclength",
        "roll",
        "erfi",
        "polygamma",
        "trigamma",
        "pochhammer",
        "ph",
        "ψ",
        "next",
        "factorial",
        "doublefactorial",
        "subfactorial",
        "units",
        "An",
        "Ap",
        "eta",
        "η",
        "iden",
        "identity",
        "quartic",
        "solve",
        "inter",
        "interpolate",
        "unity",
        "iter",
        "weighted",
        "lobf",
        "lineofbestfit",
        "onlyreal",
        "onlyre",
        "ore",
        "onlyimag",
        "onlyim",
        "oim",
        "mod",
        "cossin",
        "sincos",
        "surfacearea",
        "sarea",
        "cov",
        "covariance",
        "extrema",
        "plane",
        "is_nan",
        "is_inf",
        "is_fin",
        "is_finite",
        "nth_prime",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<&str>>()
}
