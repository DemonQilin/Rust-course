fn main() {
    // Create
    // The variables are create with keyword "let", rust automatically infers type but you can specific one.
    let a = 5;

    // Mutability
    // The variables are inmutable by default. To make them mutable you should be add keyword "mut" beetwen "let" and variable name
    let mut b = 5;
    b = 10;

    // Shadowing
    // When you declare one variable that shares scope and same name wiht another, then the previous variable is shadowed
    // Allow change variable type without declaring another variable with different name
    let c = 20;
    let c = "hola";

    // Scope
    // The variables live in a scope that is an area between curly brackets

    let d = 40;

    {
        let d = 80; // shadowing of d
        let e = 1;
        print!("inner scope d is: {d}");
    }

    print!("e from outer scope: {e}");
    print!("outer scope d is: {d}");
}
