pub trait ExprVisitor<T, R> {
    fn visit_binary(expr: T) -> R;
    fn visit_grouping(expr: T) -> R;
    fn visit_literal(expr: T) -> R;
    fn visit_unary(expr: T) -> R;
}

pub trait Visitable<T, R, V: ExprVisitor<T, R>> {
    fn accept(v: V) -> R;
}
