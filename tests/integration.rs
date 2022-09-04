use glc::{Grammar, Expression, nt, nt_seq_rule, t_or_rule};

fn grammar() -> Grammar {
    Grammar(
        nt!("E"),
        vec![
            nt_seq_rule!("E" => "T", "E'"),
            nt_seq_rule!("E'" => "PM", "T", "E'"),
            t_or_rule!("E'" => ""),
            t_or_rule!("E'" => ""), // increase prob. by duplicating rule
            t_or_rule!("E'" => ""), // increase prob. by duplicating rule
            t_or_rule!("PM" => "+", "-"),
            nt_seq_rule!("T" => "F", "T'"),
            nt_seq_rule!("T'" => "MD", "F", "T'"),
            t_or_rule!("MD" => "*", "/"),
            t_or_rule!("T'" => ""),
            t_or_rule!("T'" => ""), // increase prob.
            t_or_rule!("T'" => ""), // increase prob.
            nt_seq_rule!("F" => "LP", "E", "RP"),
            nt_seq_rule!("F" => "M", "LP", "E", "RP"),
            nt_seq_rule!("F" => "N"),
            nt_seq_rule!("F" => "M", "N"),
            t_or_rule!("LP" => "("),
            t_or_rule!("RP" => ")"),
            t_or_rule!("M" => "-"),

            nt_seq_rule!("N" => "D", "DEC"),
            nt_seq_rule!("D" => "D", "OD"),
            nt_seq_rule!("OD" => "D"),
            t_or_rule!("OD" => ""),
            t_or_rule!("OD" => ""), // increase probability
            t_or_rule!("OD" => ""), // increase probability
            t_or_rule!("D" => "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"),
            nt_seq_rule!("DEC" => "P", "D"),
            t_or_rule!("P" => "."),
        ]
    )
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
