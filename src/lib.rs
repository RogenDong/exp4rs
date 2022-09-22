use log::{debug, info, trace};
use syn::{BinOp, Expr, ExprBinary, ExprLit, ExprUnary, Lit};

use crate::error::ParseError;

mod error;

/// 字面值解析为数值
fn parse_num(literal: &ExprLit) -> Option<f64> {
    trace!("字面值解析为数值");
    match &literal.lit {
        Lit::Int(ii) =>
            if ii.suffix().is_empty() {
                debug!("字面值: {}", ii.token());
                if let Ok(v) = ii.base10_parse::<f64>() {
                    return Some(v);
                }
            }
        Lit::Float(ff) =>
            if ff.suffix().is_empty() {
                debug!("字面值: {}", ff.token());
                if let Ok(v) = ff.base10_parse::<f64>() {
                    return Some(v);
                }
            }
        _ => debug!("字面值属性: {:?}", literal.lit),
    }
    None
}

/// 二元表达式解析
fn parse_bin(binary: &ExprBinary) -> Option<f64> {
    trace!("二元表达式分解");
    let l = parse_exp(&binary.left)?;
    debug!("左值: {}", l);
    let r = parse_exp(&binary.right)?;
    debug!("右值: {}", r);
    let c = match &binary.op {
        BinOp::Add(_) => l + r,
        BinOp::Sub(_) => l - r,
        BinOp::Mul(_) => l * r,
        BinOp::Div(_) => l / r,
        BinOp::Shl(_) => ((l as i64) << (r as i64)) as f64,
        BinOp::Shr(_) => ((l as i64) >> (r as i64)) as f64,
        BinOp::BitOr(_) => ((l as i64) | (r as i64)) as f64,
        BinOp::BitAnd(_) => ((l as i64) & (r as i64)) as f64,
        BinOp::BitXor(_) => ((l as i64) ^ (r as i64)) as f64,
        _ => return None,
    };
    trace!("计算值: {}", c);
    Some(c)
}

/// 一元表达式解析
fn parse_una(unary: &ExprUnary) -> Option<f64> {
    trace!("一元表达式分解");
    None
}

/// 递归解析表达式树
fn parse_exp(ee: &Expr) -> Option<f64> {
    match ee {
        Expr::Paren(pp) => parse_exp(&pp.expr),
        Expr::Binary(bb) => parse_bin(bb),
        Expr::Unary(uu) => parse_una(uu),
        Expr::Lit(ll) => parse_num(ll),
        _ => {
            debug!("表达式属性: {:?}", ee);
            None
        }
    }
}

/// 表达式求值
/// # Arguments
/// - `expr` 表达式文本
pub fn eval(expr: &str) -> Result<f64, ParseError> {
    match syn::parse_str::<Expr>(expr) {
        Ok(ee) => {
            info!("尝试计算表达式结果");
            if let Some(res) = parse_exp(&ee) {
                info!("解析、计算完成");
                trace!("{} = {}", expr, res);
                Ok(res)
            } else {
                info!("计算失败，表达式不符合规则");
                Err(ParseError::new("存在不支持的格式或字符；\
                    支持的字符: +,-,*,/,&,|,^,<<,>>,(,),\
                    空格,数值（整数,浮点数）".to_string()))
            }
        }
        Err(e) => {
            Err(ParseError::new(format!("表达式语法错误！{:#?}", e)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval;

    #[test]
    fn ts_1() {
        let aa = [
            "(  ((  13-10  ))  ) << (  7-4  )",
            "{  [(  13-10  )]  }",
            "{  [(",
        ];
        for ee in aa {
            match eval(ee) {
                Ok(c) => println!("{} = {}", ee, c),
                Err(e) => println!("err: {}", e),
            }
        }
    }
}
