// spell-checker:ignore bigdecimal

use std::str::FromStr;

use bigdecimal::BigDecimal;

/// Compute bd**exp using exponentiation by squaring algorithm, while maintaining the
/// precision specified in ctx (the number of digits would otherwise explode).
// TODO: Upstream this to bigdecimal-rs.
fn pow_with_context(bd: BigDecimal, exp: u32, ctx: &bigdecimal::Context) -> BigDecimal {
    if exp == 0 {
        return 1.into();
    }

    fn trim_precision(bd: BigDecimal, ctx: &bigdecimal::Context, factor: u64) -> BigDecimal {
        let prec = ctx.precision().get() + factor;
        if bd.digits() > prec {
            bd.with_precision_round(prec.try_into().unwrap(), ctx.rounding_mode())
        } else {
            bd
        }
    }
    println!("it {bd} {exp}");
    let sq = trim_precision(bd.square(), ctx, 64);
    let ret = if exp % 2 == 0 {
        pow_with_context(sq, exp / 2, ctx)
    } else {
        trim_precision(&bd * pow_with_context(sq, (exp - 1) / 2, ctx), ctx, 3)
    };
    println!("ret {ret}");
    ret
}

fn main() {
    println!("Start test");

    // Wolfram Alpha can get us (close to?) these values with a bit of log trickery:
    // 2**3000000000 = 10**log_10(2**3000000000) = 10**(3000000000 * log_10(2))
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

/*
        assert_eq!(
            Ok(ExtendedBigDecimal::BigDecimal(
                // Wolfram Alpha says 1.3492131462369983551036088935544888715959511045742395978049631768570509541390540646442193112226520316... × 10^-9030900
                BigDecimal::from_str("1.349213146236998355103608893554488871595951104574239597804963176857050954139054064644219311222656999e-9030900").unwrap()
            )),
            // Couldn't get a answer from Wolfram Alpha for smaller negative exponents
            ExtendedBigDecimal::extended_parse("0x1p-30000000")
        );*/
}
