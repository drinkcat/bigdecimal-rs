// spell-checker:ignore bigdecimal prec

use std::{fmt::Display, str::FromStr};

use bigdecimal::{BigDecimal, RoundingMode};

fn test_one<T>(start: T, exp: i64, str: &str) where T: Display, BigDecimal: From<T> {
    println!("Compute {start}**{exp}");

    let ctx = bigdecimal::Context::default();
    let bd_start = BigDecimal::from(start);
    let bd = bd_start.pow_with_context(exp, &ctx);
    let bd_good = BigDecimal::from_str(str).unwrap();
    let bd_good_round = bd_good.with_precision_round(ctx.precision(), ctx.rounding_mode());

    println!("100d  0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
    println!("good  {bd_good}");
    println!("val   {bd}");
    println!("good2 {bd_good_round}");

    assert_eq!(bd, bd_good_round);
}

fn test_two<T>(start: T, exp: i64) where T: Display, BigDecimal: From<T> {
    println!("Compute/compare {start}**{exp}");

    let context = bigdecimal::Context::new(50.try_into().unwrap(), bigdecimal::RoundingMode::HalfEven);
    let context_good = bigdecimal::Context::new(500.try_into().unwrap(), bigdecimal::RoundingMode::HalfEven);

    let bd_start = BigDecimal::from(start);
    let bd = bd_start.pow_with_context(exp, &context);
    let bd_good = bd_start.pow_with_context(exp, &context_good);
    let bd_good_round = bd_good.with_precision_round(context.precision(), context.rounding_mode());

    if bd != bd_good_round {
        println!("100d  0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
        println!("good  {bd_good}");
        println!("val   {bd}");
        println!("good2 {bd_good_round}");
    }
    assert_eq!(bd, bd_good_round);
}

fn main() {
    println!("Start test");

    // Wolfram Alpha can get us (close to?) these values with a bit of log trickery:
    // 2**3000000000 = 10**log_10(2**3000000000) = 10**(3000000000 * log_10(2))

    test_one(2, 3000, "1.230231922161117176931558813276752514640713895736833715766118029160058800614672948775360067838593459582429649254051804908512884180898236823e903");
    test_one(2, 2048, "3.231700607131100730071487668866995196044410266971548403213034542752465513886789089319720141152291346368871796092189801949411955915049092109e616");
    test_one(2, 2001, "2.296261390548509048465666402355363968044635404177390400955285473651532522784740627713318972633012539836891929277974925546894237921726110662e602");
    test_one(2, 3000000000, "9.8162042336235053508313854078782835648991393286913072670026492205522618203568834202759669215027003865712110468405800021098042607617495e903089986");
    test_one(BigDecimal::from(2).inverse(), 30000000, "1.34921314623699835510360889355448887159595110457423959780496317685705095413905406464421931122265203166201415504288117880522818881981650e-9030900");
    test_one(2, -30000000, "1.34921314623699835510360889355448887159595110457423959780496317685705095413905406464421931122265203166201415504288117880522818881981650e-9030900");
    test_one(BigDecimal::from(3).inverse_with_context(&bigdecimal::Context::new(1000.try_into().unwrap(), RoundingMode::HalfEven)), 30000000, "2.2824965348198962029744520058679746159742143842721452620663907608967745444344346503448190515521985159162206416095535917875712100566195e-14313638");
    test_one(3, -30000000, "2.2824965348198962029744520058679746159742143842721452620663907608967745444344346503448190515521985159162206416095535917875712100566195e-14313638");

    for _ in 0..100000000 {
        test_two(BigDecimal::try_from(rand::random_range(-1e9..=1e9)).unwrap(), rand::random_range(-1e9..=1e9) as i64);
    }
}
