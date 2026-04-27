struct Program {
    statements: Vec<Statement>,
}

enum Node {
    Statement(Statement),
    Expression(Expression),
}

enum Statement {}

enum Expression {}
