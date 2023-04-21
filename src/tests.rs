mod tests {
    use crate::{lexer::Lexer, syntax::Syntax};

    #[test]
    fn stmt_if() {
        let lexer = Lexer::new("if(a>b){hello+world;}");
    
        let mut syntax = Syntax::new(lexer);
        assert_eq!(syntax.check_validity(), true);
    }

    #[test]
    fn stmt_if_else() {
        let lexer = Lexer::new("if(a>b){hello+world;}else{bob+world;}");
    
        let mut syntax = Syntax::new(lexer);
        assert_eq!(syntax.check_validity(), true);
    }

    #[test]
    fn stmt_block() {
        let lexer = Lexer::new("{1230+1231;5+23213;hello+world;}");
    
        let mut syntax = Syntax::new(lexer);
        assert_eq!(syntax.check_validity(), true);
    }

    #[test]
    fn stmt_expr() {
        let lexer = Lexer::new("5+5*5%5/5");
    
        let mut syntax = Syntax::new(lexer);
        assert_eq!(syntax.check_validity(), true);
    }

    #[test]
    fn stmt_while() {
        let lexer = Lexer::new("while(true){hello+world;}");
    
        let mut syntax = Syntax::new(lexer);
        assert_eq!(syntax.check_validity(), true);
    }

    #[test]
    fn stmt_complex_0() {
        let lexer = Lexer::new("hello + world * 1249129.521512 if(bob>hello){hello-world;}");
    
        let mut syntax = Syntax::new(lexer);
        assert_eq!(syntax.check_validity(), true);
    }

    #[test]
    fn stmt_complex_1() {
        let lexer = Lexer::new("if(x 
            > 20 || y <= 40 && v >= z
        ) 
        { hello 
            + 
            world; 
        }
         if(
            x>20
        ){hello
            +world
            ;
        }else{bye+world;}{1230+
            1231;
            5+23213;hello+
            world;}while(true){bye+world;}{bob+bob;qwiorwqr+wijqijqf;{qiwjdiqjw+jiejf;};}55555555555*421");
    
        let mut syntax = Syntax::new(lexer);
        assert_eq!(syntax.check_validity(), true);
    }

    #[test]
    fn stmt_assign() {
        let lexer = Lexer::new("hello = 5 + 5");
    
        let mut syntax = Syntax::new(lexer);
        assert_eq!(syntax.check_validity(), true);
    }

    #[test]
    fn stmt_declare() {
        let lexer = Lexer::new("DataType hello");
    
        let mut syntax = Syntax::new(lexer);
        assert_eq!(syntax.check_validity(), true);
    }

    #[test]
    fn stmt_declare_complex() {
        let lexer = Lexer::new("DataType hello,test");
    
        let mut syntax = Syntax::new(lexer);
        assert_eq!(syntax.check_validity(), true);
    }
}