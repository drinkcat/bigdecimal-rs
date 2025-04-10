// spell-checker:ignore bigdecimal prec

use std::str::FromStr;

use bigdecimal::BigDecimal;

fn trim_precision(bd: BigDecimal, ctx: &bigdecimal::Context, factor: u64) -> BigDecimal {
    let prec = ctx.precision().get() * factor;
    if bd.digits() > prec {
        bd.with_precision_round(prec.try_into().unwrap(), ctx.rounding_mode())
    } else {
        bd
    }
}

/// Compute bd**exp using exponentiation by squaring algorithm, while maintaining the
/// precision specified in ctx (the number of digits would otherwise explode).
// TODO: Upstream this to bigdecimal-rs.
fn pow_with_context(bd: BigDecimal, exp: u32, ctx: &bigdecimal::Context) -> BigDecimal {
    if exp == 0 {
        return 1.into();
    }

    println!("it {bd} {exp}");
    let sq = trim_precision(bd.square(), ctx, 2);
    let ret = if exp % 2 == 0 {
        pow_with_context(sq, exp / 2, ctx)
    } else {
        trim_precision(&bd * pow_with_context(sq, (exp - 1) / 2, ctx), ctx, 2)
    };
    println!("ret {ret}");
    ret
}

fn pow_with_context_iterative(bd: BigDecimal, exp: u32, ctx: &bigdecimal::Context) -> BigDecimal {
    if exp == 0 {
        return 1.into();
    }

    let mut bd_y: BigDecimal = 1.into();
    let mut bd_x = bd;
    let mut n = exp;
    while n > 1 {
        println!("itx {bd_x}");
        println!("ity {bd_y}");
        if n % 2 == 1 {
            bd_y = trim_precision(&bd_x * bd_y, ctx, 2);
            n -= 1;
        }
        bd_x = trim_precision(bd_x.square(), ctx, 2);
        n /= 2;
    }

    return trim_precision(bd_x * bd_y, ctx, 1);
}

fn main() {
    println!("Start test");

    let bd = pow_with_context_iterative(2.into(),3000, &bigdecimal::Context::default());
    let bd_good = BigDecimal::from_str("1.230231922161117176931558813276752514640713895736833715766118029160058800614672948775360067838593459582429649254051804908512884180898236823e903").unwrap();

    println!("100d  0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    println!("good  {bd_good}");
    println!("val   {bd}");
    println!("delta {}", (bd-&bd_good)/&bd_good);

    // Wolfram Alpha can get us (close to?) these values with a bit of log trickery:
    // 2**3000000000 = 10**log_10(2**3000000000) = 10**(3000000000 * log_10(2))
    let bd = pow_with_context_iterative(2.into(),3000000000, &bigdecimal::Context::default());
    let bd_good = BigDecimal::from_str("9.8162042336235053508313854078782835648991393286913072670026492205522618203568834202759669215027003865712110468405800021098042607617495e903089986").unwrap();

    println!("100d  0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    println!("good  {bd_good}");
    println!("val   {bd}");
    println!("delta {}", (bd-&bd_good)/&bd_good);

    if false {
    
    let bd = pow_with_context(BigDecimal::from(2).inverse(),30000000, &bigdecimal::Context::default());
    let bd_good = BigDecimal::from_str("1.34921314623699835510360889355448887159595110457423959780496317685705095413905406464421931122265203166201415504288117880522818881981650e-9030900").unwrap();

    println!("100d  0.123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    println!("good  {bd_good}");
    println!("val   {bd}");
    println!("delta {}", (bd-&bd_good)/&bd_good);
    }
}
