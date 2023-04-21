# Notes
    I've added tests so you can run `cargo test` and it'll syntactically check each expression.
    If you want to input your own values, you can edit lexer.txt. The current lexer.txt has 60-70 tokens.

# LR Grammar
    STMT' -> STMT
    STMT -> IFSTMT
    STMT -> BLOCK
    STMT -> ASSIGN
    STMT -> DECLARE
    STMT -> WHILE_LOOP
    
    BLOCK -> { STMT_LIST }
    
    STMT_LIST -> STMT_LIST STMT ;
    STMT_LIST -> ''
    
    IFSTMT -> if ( BOOL_EXPR ) BLOCK
    IFSTMT -> if ( BOOL_EXPR ) BLOCK else BLOCK
    
    ASSIGN -> ID = EXPR
    
    DECLARE -> DECLARE , ID
    DECLARE -> DataType ID
    
    WHILE_LOOP -> while ( BOOL_EXPR ) BLOCK
    WHILE_LOOP -> while ( BOOL_EXPR ) BLOCK else BLOCK
    
    BOOL_EXPR -> BOOL_EXPR > BTERM
    BOOL_EXPR -> BOOL_EXPR >= BTERM
    BOOL_EXPR -> BOOL_EXPR < BTERM
    BOOL_EXPR -> BOOL_EXPR <= BTERM
    BOOL_EXPR -> BTERM
    
    BTERM -> BTERM == BAND
    BTERM -> BTERM != BAND
    BTERM -> BAND
    
    BAND -> BAND && BOR
    BAND -> BOR
    
    BOR -> BOR || EXPR
    BOR -> EXPR
    
    EXPR -> EXPR + TERM
    EXPR -> EXPR - TERM
    EXPR -> TERM
    
    TERM -> TERM * FACT
    TERM -> TERM / FACT
    TERM -> TERM % FACT
    TERM -> FACT
    
    FACT -> ID
    FACT -> INT_LIT
    FACT -> FLOAT_LIT
    FACT -> ( EXPR )

# LR Table
    Located in lr_table.html

# Denotational Semantics
    Located in denotational_semantics.txt