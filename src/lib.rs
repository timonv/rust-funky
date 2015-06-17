#![feature(trace_macros)]

use std::sync::mpsc::channel;
use std::thread;

// trace_macros!(true);

#[macro_export]
macro_rules! funky {
    ($func: ident) => {{
        let (tx, rx) = channel();
        thread::spawn(move || {
            let result = $func();
            tx.send(result).ok().expect("Funky: Could not send result to receiver");
        });
        rx
    }};
    ($func: ident, $($x: expr),*) => {{
        let (tx, rx) = channel();
        thread::spawn(move || {
            let result = $func($($x),*);
            tx.send(result).ok().expect("Funky: Could not send result to receiver");
        });
        rx
    }};
}

#[test]
fn test_simple_function() {
    let func = || -> String {
        "abc".to_string()
    };

    let rx = funky!(func);
    assert_eq!(rx.recv().unwrap(), "abc".to_string())
}

#[test]
fn test_function_with_argument() {
    let func = |string: String| -> String {
        string
    };

    let rx = funky!(func, "abc".to_string());
    assert_eq!(rx.recv().unwrap(), "abc".to_string())
}

#[test]
fn test_function_with_multiple_arguments() {
    let func = |a: String, b: String| -> String {
        (a + &b).to_string()
    };

    let rx = funky!(func, "a".to_string(), "b".to_string());
    assert_eq!(rx.recv().unwrap(), "ab".to_string())
}
