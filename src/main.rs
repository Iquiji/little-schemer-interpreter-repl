use little_schemer::{Interpreter, ExpressionTypes};
use little_schemer::ExpressionTypes::*;
use simple_prompts::prompt;


fn main() {
    println!("Hello Scheme REPL! by Iquiji");
    let mut interpreter = Interpreter::new();

    loop {
        let our_input: String = prompt("> ");

        if our_input.is_empty(){
            continue;
        }
        if our_input == "exit" || our_input == "quit"{
            break;
        }
        if our_input == "load example"{
            println!("(define factorial (lambda (n) (cond ((eq? n 0) 1) (else (* n (factorial (- n 1)))))))");
            continue;
        }


        // (define factorial (lambda (n) (cond ((eq? n 0) 1) (else (* n (factorial (- n 1))))))) works
        let res = execute_form_with_ast_custom_interpreter(&mut interpreter, &our_input);

        println!("RESULT: {}",res);
    }
}

pub fn execute_form_with_ast_custom_interpreter(
    interpreter: &mut Interpreter,
    form: &str,
) -> ExpressionTypes {
    let ast = &interpreter.generate_abstract_syntax_tree(form);

    if let List(only_form) = &ast[0] {
        interpreter.execute_on_ast(only_form)
    } else {
        panic!(
            "Form can only have one item that has to be a list: {:?}",
            ast
        )
    }
}