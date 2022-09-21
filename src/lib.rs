mod error;

use log::{debug, error, info, trace};
use syn::{Expr, ExprBinary, ExprLit, ExprUnary, UnOp};

use crate::error::ParseError;

/// 字面值解析为数值
fn parse_num(literal: &ExprLit) -> Option<i32> {
    trace!("字面值解析为数值");
    None
}

/// 二元表达式分解
fn parse_bin(binary: &ExprBinary) -> Option<i32> {
    trace!("二元表达式分解");
    None
}

/// 一元表达式分解
fn parse_una(unary: &ExprUnary) -> Option<i32> {
    trace!("一元表达式分解");
    None
}

/// 递归解析表达式树
fn cmp(ee: &Expr) -> Option<i32> {
    match ee {
        Expr::Binary(bb) => parse_bin(bb),
        Expr::Unary(uu) => parse_una(uu),
        Expr::Paren(pp) => cmp(&pp.expr),
        Expr::Lit(ll) => parse_num(ll),
        _ => None
    }
}

/// 表达式求值
pub fn eval(expr: &str) -> Result<i32, ParseError> {
    match syn::parse_str::<Expr>(expr) {
        Ok(ee) => {
            trace!("尝试计算表达式结果");
            if let Some(res) = cmp(&ee) {
                debug!("{} = {}", expr, res);
                Ok(res)
            } else {
                info!("计算失败，表达式不符合规则");
                Err(ParseError::new(""))
            }
        }
        Err(e) => {
            error!("表达式语法错误！");
            Err(ParseError::new("表达式语法错误！"))
        }
    }
}
