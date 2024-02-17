// oint algorithms (Rust)

use {
    meval::{eval_str_with_context, Context},
    scan_fmt::{scan_fmt, scanln_fmt},
    std::{
        io::{self, Write},
        sync::{
            mpsc::{self, SendError},
            Arc,
        },
        thread,
    },
};

fn f(x: f64, expr: &str) -> f64 {
    let mut context = Context::new();
    context.var("x", x);
    eval_str_with_context(expr, &context).expect("wrong variable!")
}

fn lrect(a: f64, b: f64, n: f64, expr: &str) -> f64 {
    let mut s: f64 = 0.0;
    for i in 0..=(n - 1.0) as u64 {
        s += f(a + i as f64 * (b - a) / n, expr);
    }
    s * ((b - a) / n)
}

fn rrect(a: f64, b: f64, n: f64, expr: &str) -> f64 {
    let mut s: f64 = 0.0;
    for i in 1..=n as u64 {
        s += f(a + i as f64 * (b - a) / n, expr);
    }
    s * ((b - a) / n)
}

fn mrect(a: f64, b: f64, n: f64, expr: &str) -> f64 {
    let mut s: f64 = 0.0;
    for i in 0..=(n - 1.0) as u64 {
        s += f(a + (b - a) * (2.0 * i as f64 + 1.0) / (2.0 * n), expr);
    }
    s * ((b - a) / n)
}

fn trapezoid(a: f64, b: f64, n: f64, expr: &str) -> f64 {
    let mut s: f64 = 0.0;
    for i in 0..=(n - 1.0) as u64 {
        s +=
            (f(a + i as f64 * (b - a) / n, expr) + f(a + (i + 1) as f64 * (b - a) / n, expr)) / 2.0;
    }
    s * ((b - a) / n)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "first, write lower bound, upper bound, then epsilon respectively in this format\n[a;b],e"
    );
    let (a, b, eps) = scanln_fmt!("[{};{}],{}", f64, f64, f64)?;

    println!("then write your function with `x` variable");
    let mut expr = String::new();
    io::stdin().read_line(&mut expr)?;

    let (txl, rx) = mpsc::channel();
    let (txr, txm, txt) = (txl.clone(), txl.clone(), txl.clone());

    let expr = Arc::new(expr);
    let (exprr, exprm, exprt) = (Arc::clone(&expr), Arc::clone(&expr), Arc::clone(&expr));

    thread::spawn(move || -> Result<(), SendError<_>> {
        let (mut n, mut s1, mut s2) = (1.0, 0.0, f(a, &expr) * (b - a));
        while (s2 - s1).abs() > eps {
            n *= 2.0;
            (s1, s2) = (s2, lrect(a, b, n, &expr));
        }
        txl.send(format!("left: {s2}\n"))?;
        Ok(())
    });

    thread::spawn(move || -> Result<(), SendError<_>> {
        let (mut n, mut s1, mut s2) = (1.0, 0.0, f(a, &exprr) * (b - a));
        while (s2 - s1).abs() > eps {
            n *= 2.0;
            (s1, s2) = (s2, rrect(a, b, n, &exprr));
        }
        txr.send(format!("right: {s2}\n"))?;
        Ok(())
    });

    thread::spawn(move || -> Result<(), SendError<_>> {
        let (mut n, mut s1, mut s2) = (1.0, 0.0, f((a + b) / 2.0, &exprm) * (b - a));
        while (s2 - s1).abs() > eps {
            n *= 2.0;
            (s1, s2) = (s2, mrect(a, b, n, &exprm));
        }
        txm.send(format!("middle: {s2}\n"))?;
        Ok(())
    });

    thread::spawn(move || -> Result<(), SendError<_>> {
        let (mut n, mut s1, mut s2) = (1.0, 0.0, (b - a) * (f(a, &exprt) + f(b, &exprt)) / 2.0);
        while (s2 - s1).abs() > eps {
            n *= 2.0;
            (s1, s2) = (s2, trapezoid(a, b, n, &exprt));
        }
        txt.send(format!("trapezoid: {s2}\n"))?;
        Ok(())
    });

    let mut handle = io::BufWriter::new(io::stdout());
    handle.write_all(b"\n")?;
    for res in rx {
        handle.write_all(res.as_bytes())?;
    }
    handle.flush()?;
    Ok(())
}
