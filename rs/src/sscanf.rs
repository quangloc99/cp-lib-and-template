/// Example usage:
/// let (a, b) = sscanf!("10 + 20 =", (usize, " + ", usize, " ="));
/// println!("{}", a + b);
#[allow(unused_macros)]
macro_rules! sscanf {
    ($line: expr, ($($tail: tt),*)) => {
        {
            let line_storage = $line;
            #[allow(unused_mut)]
            let mut line_var = &line_storage[..];
            sscanf!(@parse line_var, line_var, (), ($($tail),*))
        }
    };

    (@parse $line_var: ident, $line_expr: expr, ($($res: tt),*), ($($last_sep: literal)?)) => {
        $($res),*
    };

    (@parse $line_var: ident, $line_expr: expr, ($($res: tt),*), ($last_var_type: ty)) => {
        ($($res, )* $line_expr.parse::<$last_var_type>().unwrap())
    };

    (@parse $line_var: ident, $line_expr: expr, ($($res: tt),*), ($last_var_type: ty, $last_sep: literal)) => {
        sscanf!(@parse $line_var, $line_expr.split_once($last_sep).unwrap().0, ($($res),*), ($last_var_type))
    };

    (@parse $line_var: ident, $line_expr: expr, ($($res: tt),*), ($sep: literal, $($tail: tt),*)) => {
        sscanf!(@parse $line_var, $line_expr.split_once($sep).unwrap().1, ($($res),*), ($($tail),*))
    };

    (@parse $line_var: ident, $line_expr: expr, ($($res: tt),*), ($var_type: ty, $sep: literal, $($tail: tt),*)) => {
        sscanf!(@parse $line_var, $line_var, ($($res, )* {
            let (var_str, rest) = $line_expr.split_once($sep).unwrap();
            $line_var = rest;
            var_str.parse::<$var_type>().unwrap()
        }), ($($tail),*))
    };
}
