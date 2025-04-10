// spell-checker:ignore bigdecimal prec

use std::str::FromStr;

use bigdecimal::BigDecimal;

/// Compute bd**exp using exponentiation by squaring algorithm, while maintaining the
/// precision specified in ctx (the number of digits would otherwise explode).
// Algorithm comes from https://en.wikipedia.org/wiki/Exponentiation_by_squaring
fn pow_with_context(bd: BigDecimal, exp: u32, ctx: &bigdecimal::Context) -> BigDecimal {
    if exp == 0 {
        return 1.into();
    }

    // When performing a multiplication between 2 numbers, we may lose up to 2 digits
    // of precision.
    const MARGIN_PER_MUL: u64 = 2;

    fn trim_precision(bd: BigDecimal, ctx: &bigdecimal::Context, margin: u64) -> BigDecimal {
        let prec = ctx.precision().get() + margin;
        if bd.digits() > prec {
            bd.with_precision_round(prec.try_into().unwrap(), ctx.rounding_mode())
        } else {
            bd
        }
    }

    // Count the number of multiplications we're going to perform, one per "1" binary digit
    // in exp, and the number of times we can divide exp by 2.
    let mut margin = MARGIN_PER_MUL * (exp.count_ones() + exp.ilog2() - 1) as u64;

    let mut bd_y: BigDecimal = 1.into();
    let mut bd_x = bd;
    let mut n = exp;
    while n > 1 {
        if n % 2 == 1 {
            bd_y = trim_precision(&bd_x * bd_y, ctx, margin);
            margin -= MARGIN_PER_MUL;
            n -= 1;
        }
        bd_x = trim_precision(bd_x.square(), ctx, margin);
        margin -= MARGIN_PER_MUL;
        n /= 2;
    }
    debug_assert_eq!(margin, 0);

    return trim_precision(bd_x * bd_y, ctx, 0);
}

fn main() {
    println!("Start test");

    // Wolfram Alpha can get us (close to?) these values with a bit of log trickery:
    // 2**3000000000 = 10**log_10(2**3000000000) = 10**(3000000000 * log_10(2))

    let bd = pow_with_context(2.into(),3000, &bigdecimal::Context::default());
    let bd_good = BigDecimal::from_str("1.230231922161117176931558813276752514640713895736833715766118029160058800614672948775360067838593459582429649254051804908512884180898236823e903").unwrap();

    println!("100d  0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    println!("good  {bd_good}");
    println!("val   {bd}");
    println!("delta {}", (bd-&bd_good)/&bd_good);

    let bd = pow_with_context(2.into(),2048, &bigdecimal::Context::default());
    let bd_good = BigDecimal::from_str("3.231700607131100730071487668866995196044410266971548403213034542752465513886789089319720141152291346368871796092189801949411955915049092109e616").unwrap();

    println!("100d  0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    println!("good  {bd_good}");
    println!("val   {bd}");
    println!("delta {}", (bd-&bd_good)/&bd_good);

    let bd = pow_with_context(2.into(),2001, &bigdecimal::Context::default());
    let bd_good = BigDecimal::from_str("2.296261390548509048465666402355363968044635404177390400955285473651532522784740627713318972633012539836891929277974925546894237921726110662e602").unwrap();

    println!("100d  0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    println!("good  {bd_good}");
    println!("val   {bd}");
    println!("delta {}", (bd-&bd_good)/&bd_good);

    let bd = pow_with_context(2.into(),3000000000, &bigdecimal::Context::default());
    let bd_good = BigDecimal::from_str("9.8162042336235053508313854078782835648991393286913072670026492205522618203568834202759669215027003865712110468405800021098042607617495e903089986").unwrap();

    println!("100d  0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    println!("good  {bd_good}");
    println!("val   {bd}");
    println!("delta {}", (bd-&bd_good)/&bd_good);

    let bd = pow_with_context(BigDecimal::from(2).inverse(),30000000, &bigdecimal::Context::default());
    let bd_good = BigDecimal::from_str("1.34921314623699835510360889355448887159595110457423959780496317685705095413905406464421931122265203166201415504288117880522818881981650e-9030900").unwrap();

    println!("100d  0.123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    println!("good  {bd_good}");
    println!("val   {bd}");
    println!("delta {}", (bd-&bd_good)/&bd_good);
}
