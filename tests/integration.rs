#![recursion_limit = "256"]
use glc::{grammar, Expression, Grammar};

fn grammar() -> Grammar {
    grammar!{
        E => T E_;
        E_ => PM T E_;
        E_ => "";
        E_ => "";
        E_ => "";
        PM => "+", "-";
        T => F T_;
        T_ => MD F T_;
        MD => "*", "/";
        T_ => "";
        T_ => "";
        T_ => "";
        F => LP E RP;
        F => M LP E RP;
        F => N;
        F => M N;
        LP => "(";
        RP => ")";
        M => "-";
        N => D DEC;
        D => D OD;
        OD => D;
        OD => "";
        OD => "";
        OD => "";
        D => "0", "1", "2", "3", "4", "5", "6", "7", "8", "9";
        DEC => P D;
        P => "."
    }
}

#[test]
#[ignore]
fn without_bounds() {
    random_exprs(u64::MAX);
}

#[test]
fn with_bounds() {
    random_exprs(1_000);
}

fn random_exprs(bound: u64) {
    let grammar = grammar();
    let mut exprs = 0;

    'der: while exprs < bound {
        exprs += 1;
        let mut d = grammar.start_derivation();
        let mut count = 0;

        while !d.is_done() {
            count += 1;
            d.derive_step(&grammar);

            if count > 10_000 {
                println!("skipped");
                continue 'der;
            }
        }

        let e: Expression = d.into();
        let mexe = mexe::eval(e.to_string()).unwrap();
        let meval = meval::eval_str(e.to_string()).unwrap();
        println!("{}\nval: {}, steps: {}\n", &e, mexe, count);

        if !mexe.is_nan() || !meval.is_nan() {
            assert_eq!(mexe, meval);
        }
    }
}
