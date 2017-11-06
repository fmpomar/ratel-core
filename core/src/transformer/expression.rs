use transformer::{Transformer, Transformable};
use ast::{Expression, ExpressionPtr, ExpressionList};

impl<'ast> Transformable<'ast> for ExpressionList<'ast> {
    #[inline]
    fn transform(&self, t: &Transformer) {
        for expression in self.ptr_iter() {
            expression.transform(t);
        }
    }
}

impl<'ast> Transformable<'ast> for ExpressionPtr<'ast> {
    fn transform(&self, t: &Transformer) {
        use self::Expression::*;

        match self.item {
            Error => panic!("Module contains errors"),
            Void => {},
            This => {},
            Identifier(ref ident) => unimplemented!(),
            Value(ref value) => unimplemented!(),
            Sequence {
                ref body
            } => {
                unimplemented!();
            },
            Array {
                ref body
            } => {
                unimplemented!();
            },
            Member {
                ref object,
                ref property,
            } => {
                unimplemented!();
            },
            ComputedMember {
                ref object,
                ref property,
            } => {
                unimplemented!();
            },
            Call {
                ref callee,
                ref arguments,
            } => {
                unimplemented!();
            },
            Binary {
                ref operator,
                ref left,
                ref right,
                ..
            } => {
                unimplemented!();
            },
            Prefix {
                ref operator,
                ref operand,
            } => {
                unimplemented!();
            },
            Postfix {
                ref operator,
                ref operand,
            } => {
                unimplemented!();
            },
            Conditional {
                ref test,
                ref consequent,
                ref alternate,
            } => {
                unimplemented!();
            },
            Template {
                ref tag,
                ref expressions,
                ref quasis,
            } => {
                unimplemented!();
            },
            Arrow {
                ref params,
                ref body,
            } => {
                unimplemented!();
            },
            Object {
                ref body,
            } => {
                unimplemented!();
            },
            Function {
                ref function,
            } => {
                unimplemented!();
            },
            Class {
                ref class,
            } => {
                unimplemented!();
            }
        }
    }
}
