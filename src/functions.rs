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
        "oproj",
        "project",
        "oproject",
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
        "reverse",
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
        "to_list",
        "to_freq",
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
        "taylor",
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
        "rand_weighted",
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
        "rand_norm",
        "rand_uniform",
        "rand_int",
        "geo_mean",
        "γ",
        "lower_gamma",
        "gamma_pdf",
        "gamma_cdf",
        "beta_cdf",
        "beta_pdf",
        "norm_cdf",
        "norm_pdf",
        "rand_gamma",
        "rand_beta",
        "skewness",
        "weighted_mean",
        "lognorm_cdf",
        "binomial_cdf",
        "geometric_cdf",
        "lognorm_pdf",
        "binomial_pmf",
        "geometric_pmf",
        "rand_lognorm",
        "rand_binomial",
        "rand_geometric",
        "rand_bernoulli",
        "sd",
        "skew",
        "kurtosis",
        "rand_poisson",
        "poisson_pmf",
        "poisson_cdf",
        "rand_neg_binomial",
        "neg_binomial_cdf",
        "neg_binomial_pmf",
        "hypergeometric_pmf",
        "hypergeometric_cdf",
        "rand_hypergeometric",
        "neg_hypergeometric_pmf",
        "neg_hypergeometric_cdf",
        "rand_neg_hypergeometric",
        "cyl",
        "cylinder",
        "prime_factors",
        "hsv_to_rgb",
        "domain_coloring_rgb",
        "set",
        "uniq",
        "union",
        "intersection",
        "set_difference",
        "symmetric_difference",
        "cartesian_product",
        "power_set",
        "set_fix",
        "subset",
        "element",
        "remove",
        "extend",
        "link",
        "rationalize",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<&str>>()
}
pub fn functions_with_args() -> HashSet<&'static str>
{
    [
        "sum(a,f(a),start,end)",
        "sum(vec)",
        "prod(a,f(a),start,end)",
        "prod(vec)",
        "Σ(a,f(a),start,end)",
        "Π(a,f(a),start,end)",
        "cofactor(mat)",
        "minor(mat)",
        "adjugate(mat)",
        "inverse(mat)",
        "transpose(mat)",
        "length(vec/mat)",
        "width(vec/mat)",
        "trace(mat)",
        "determinant(mat)",
        "part(mat,x(,y))",
        "part(vec,x)",
        "norm(num/vec/mat)",
        "abs(num/vec/mat)",
        "normalize(vec)",
        "cartesian({r,θ(,φ)})",
        "polar({x,y(,z)})",
        "angle(vec(,vec))",
        "cross(vec,vec)",
        "project(vec,vec)",
        "oproject(vec,vec)",
        "dot(vec,vec)",
        "rotate(θ)",
        "rotate(yaw,pitch,roll)",
        "sin(x)",
        "csc(x)",
        "cos(x)",
        "sec(x)",
        "tan(x)",
        "cot(x)",
        "asin(x)",
        "acsc(x)",
        "acos(x)",
        "asec(x)",
        "atan(x(,y))",
        "atan2(y,x)",
        "acot(x)",
        "sinh(x)",
        "csch(x)",
        "cosh(x)",
        "sech(x)",
        "tanh(x)",
        "coth(x)",
        "asinh(x)",
        "acsch(x)",
        "acosh(x)",
        "asech(x)",
        "atanh(x)",
        "acoth(x)",
        "cis(x)",
        "ln(x)",
        "ceil(x)",
        "floor(x)",
        "round(x)",
        "recip(x)",
        "exp(x(,y))",
        "log((b,)p)",
        "root(b(,p))",
        "bi(n,r)",
        "gamma(s(,x))",
        "max(vec/mat)",
        "min(vec/mat)",
        "sqrt(x)",
        "re(x)",
        "im(x)",
        "sgn(x)",
        "arg(x)",
        "cbrt(x)",
        "fract(x)",
        "int(x)",
        "trunc(x)",
        "fact(x)",
        "subfact(x)",
        "sinc(x)",
        "conj(x)",
        "erf(x)",
        "erfc(x)",
        "ai(x)",
        "digamma((m,)x)",
        "zeta(x)",
        "sort(vec/mat)",
        "Γ(x)",
        "ζ(x)",
        "factors(x)",
        "vec(a,f(a),start,end)",
        "all(vec)",
        "any(vec)",
        "eigenvalues(mat)",
        "eigenvectors(mat)",
        "mat(a,f(a),start,end)",
        "reverse(vec)",
        "flatten(mat)",
        "I(x,α,β(,z))",
        "P(n,r)",
        "C(n,r)",
        "split(x)",
        "slog(b,p)",
        "doublefact(x)",
        "mean(vec)",
        "weighted_mean(mat)",
        "median(vec)",
        "mode(vec)",
        "quadratic(a,b,c(,real))",
        "cubic(a,b,c,d(,real))",
        "standarddeviation(vec)",
        "variance(vec)",
        "to_list(mat)",
        "to_freq(vec)",
        "σ(vec)",
        "quartiles(vec)",
        "percentile(vec,nth)",
        "percentilerank(vec,x)",
        "norm_cdf(x,μ,σ)",
        "norm_pdf(x,μ,σ)",
        "piecewise({f(x),bool},{g(x),bool}...)",
        "is_prime(x)",
        "dice(vec/mat)",
        "μ(vec)",
        "W((k,),x)",
        "productlog((k,),x)",
        "lambertw((k,),x)",
        "ssrt((k,),x)",
        "gcd(x,y)",
        "lcm(x,y)",
        "multinomial(a,b,c...)",
        "Β((z,)a,b)",
        "B((z,)a,b)",
        "beta((z,)a,b)",
        "beta_pdf(x,α,β)",
        "beta_cdf(x,α,β)",
        "slope(a,f(a),x(,nth)(,combine))",
        "taylor(a,f(a),p,n(,x))",
        "lim(a,f(a),x(,side))",
        "area(a,f(a),start,end(,nth)(,combine)",
        "arclength(a,f(a),start,end)",
        "roll(vec/mat)",
        "erfi(x)",
        "pochhammer(x,n)",
        "next(x(,to))",
        "units(x)",
        "An(n,k)",
        "Ap(n,t)",
        "eta(x)",
        "η(x)",
        "identity(n)",
        "quartic(a,b,c,d,e(,real))",
        "solve(a,f(a)(,start))",
        "interpolate(mat,x)",
        "unity(b,p)",
        "iter(a,f(a),x,n(,steps))",
        "rand_weighted(mat)",
        "lineofbestfit(mat(,x))",
        "onlyreal(x)",
        "onlyimag(x)",
        "mod(a,b)",
        "cossin(x)",
        "sincos(x)",
        "surfacearea(a,b,z(a,b),start_b,end_b,start_a,end_a)",
        "covariance(vec)",
        "extrema(a,f(a)(,start))",
        "plane(mat(,x,y))",
        "is_nan(x)",
        "is_inf(x)",
        "is_finite(x)",
        "nth_prime(n)",
        "rand_norm(μ,σ)",
        "rand_int(a,b)",
        "rand_uniform(a,b)",
        "geo_mean(vec)",
        "γ(s(,x))",
        "lower_gamma(s,x)",
        "gamma_pdf(k,θ)",
        "gamma_cdf(k,θ)",
        "rand_gamma(k,θ)",
        "rand_beta(k,θ)",
        "skewness(vec)",
        "lognorm_cdf(x,μ,σ)",
        "binomial_cdf(k,n,p)",
        "geometric_cdf(k,p)",
        "lognorm_pdf(x,μ,σ)",
        "binomial_pmf(k,n,p)",
        "geometric_pmf(k,p)",
        "rand_lognorm(μ,σ)",
        "rand_binomial(n,p)",
        "rand_geometric(k,p)",
        "rand_bernoulli(p)",
        "rand_poisson(λ)",
        "poisson_pmf(x,λ)",
        "poisson_cdf(x,λ)",
        "kurtosis(vec)",
        "rand_neg_binomial(r,p)",
        "neg_binomial_cdf(k,r,p)",
        "neg_binomial_pmf(k,r,p)",
        "hypergeometric_pmf(k,N,K,n)",
        "hypergeometric_cdf(k,N,K,n)",
        "rand_hypergeometric(N,K,n)",
        "neg_hypergeometric_pmf(k,N,K,r)",
        "neg_hypergeometric_cdf(k,N,K,r)",
        "rand_neg_hypergeometric(N,K,r)",
        "cylinder({x,y,z})",
        "prime_factors(n)",
        "hsv_to_rgb({h,s,v})",
        "domain_coloring_rgb(x)",
        "set(a,f(a),val)",
        "uniq({vec})",
        "union(A,B)",
        "intersection(A,B)",
        "set_difference(A,B)",
        "symmetric_difference(A,B)",
        "cartesian_product(A,B)",
        "power_set(A)",
        "set_fix(A)",
        "subset(A,B)",
        "element(A,b)",
        "remove(vec,num/vec)",
        "extend(vec,num/vec)",
        "rand",
        "epoch",
        "inf",
        "true",
        "false",
        "nan",
        "and",
        "or",
        "not",
        "xor",
        "nand",
        "implies",
        "nor",
        "converse",
        "rationalize(x)",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<&str>>()
}
pub fn units_list() -> HashSet<&'static str>
{
    [
        "eV",
        "eC",
        "eM",
        "pM",
        "nM",
        "ke",
        "Na",
        "R",
        "boltzmann",
        "gravity",
        "G",
        "planck",
        "reduced_planck",
        "c",
        "meter",
        "second",
        "ampere",
        "kelvin",
        "mole",
        "candela",
        "gram",
        "joule",
        "mph",
        "mile",
        "yard",
        "foot",
        "inch",
        "pound",
        "litre",
        "hertz",
        "voltage",
        "celsius",
        "fahrenheit",
        "Wh",
        "Ah",
        "year",
        "month",
        "ly",
        "kph",
        "tesla",
        "henry",
        "weber",
        "siemens",
        "farad",
        "watt",
        "pascal",
        "ohm",
        "minute",
        "hour",
        "day",
        "week",
        "newton",
        "coulomb",
        "degrees",
        "arcsec",
        "arcmin",
        "radians",
        "gradians",
        "lumen",
        "lux",
        "nit",
        "byte",
        "gray",
        "sievert",
        "katal",
        "bit",
        "steradian",
        "atm",
        "psi",
        "bar",
        "tonne",
        "hectare",
        "acre",
        "ton",
        "oz",
        "gallon",
        "lbf",
        "parsec",
        "au",
        "floz",
        "AUD",
        "CAD",
        "CNY",
        "EUR",
        "GBP",
        "HKD",
        "IDR",
        "INR",
        "JPY",
        "KRW",
        "MYR",
        "NZD",
        "PHP",
        "SGD",
        "THB",
        "TWD",
        "VND",
        "BGN",
        "BRL",
        "CHF",
        "CLP",
        "CZK",
        "DKK",
        "HUF",
        "ILS",
        "ISK",
        "MXN",
        "NOK",
        "PLN",
        "RON",
        "SEK",
        "TRY",
        "UAH",
        "ZAR",
        "EGP",
        "JOD",
        "LBP",
        "AED",
        "MDL",
        "RSD",
        "RUB",
        "AMD",
        "AZN",
        "BDT",
        "DOP",
        "DZD",
        "GEL",
        "IQD",
        "IRR",
        "KGS",
        "KZT",
        "LYD",
        "MAD",
        "PKR",
        "SAR",
        "TJS",
        "TMT",
        "TND",
        "UZS",
        "XAF",
        "XOF",
        "BYN",
        "PEN",
        "VES",
        "ARS",
        "BOB",
        "COP",
        "CRC",
        "HTG",
        "PAB",
        "PYG",
        "UYU",
        "NGN",
        "AFN",
        "ALL",
        "ANG",
        "AOA",
        "AWG",
        "BAM",
        "BBD",
        "BHD",
        "BIF",
        "BND",
        "BSD",
        "BWP",
        "BZD",
        "CDF",
        "CUP",
        "CVE",
        "DJF",
        "ERN",
        "ETB",
        "FJD",
        "GHS",
        "GIP",
        "GMD",
        "GNF",
        "GTQ",
        "GYD",
        "HNL",
        "JMD",
        "KES",
        "KHR",
        "KMF",
        "KWD",
        "LAK",
        "LKR",
        "LRD",
        "LSL",
        "MGA",
        "MKD",
        "MMK",
        "MNT",
        "MOP",
        "MRU",
        "MUR",
        "MVR",
        "MWK",
        "MZN",
        "NAD",
        "NIO",
        "NPR",
        "OMR",
        "PGK",
        "QAR",
        "RWF",
        "SBD",
        "SCR",
        "SDG",
        "SOS",
        "SRD",
        "SSP",
        "STN",
        "SVC",
        "SYP",
        "SZL",
        "TOP",
        "TTD",
        "TZS",
        "UGX",
        "VUV",
        "WST",
        "XCD",
        "XPF",
        "YER",
        "ZMW",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<&str>>()
}
pub fn options_list() -> HashSet<&'static str>
{
    [
        "graphcli=",
        "slowcheck=",
        "interactive=",
        "prompt=",
        "progress=",
        "surface=",
        "rt=",
        "siunits=",
        "keepzeros=",
        "polar=",
        "frac=",
        "fractions=",
        "fractionsv=",
        "fractionsm=",
        "multi=",
        "tabbed=",
        "comma=",
        "units=",
        "scalegraph=",
        "debug=",
        "vars=",
        "onaxis=",
        "base=",
        "ticks=",
        "decimal=",
        "deci=",
        "decimals=",
        "graphprec=",
        "graphprecision=",
        "prec=",
        "windowsize=",
        "precision=",
        "range=",
        "xr=",
        "yr=",
        "zr=",
        "vrange=",
        "vxr=",
        "vyr=",
        "vzr=",
        "color=",
        "lines=",
        "angle=",
        "notation=",
        "graph=",
        "saveto=",
        "recol=",
        "imcol=",
        "textc=",
        "promptc=",
        "imagc=",
        "scic=",
        "unitsc=",
        "bracketc=",
        "label=",
        "point=",
        "default_units=",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<&str>>()
}
