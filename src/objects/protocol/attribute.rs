use super::Object;
use crate::ast::Expr;
use crate::eval::EvalContext;
use crate::Result;

type Method = Box<dyn Fn(&EvalContext, &[Expr]) -> Result<Object>>;
pub enum Attribute {
    Value(Object),
    Method(Method),
    Missing,
}

impl From<Object> for Attribute {
    fn from(value: Object) -> Self {
        Self::Value(value)
    }
}

impl From<Method> for Attribute {
    fn from(value: Method) -> Self {
        Self::Method(value)
    }
}

impl From<Option<Object>> for Attribute {
    fn from(value: Option<Object>) -> Self {
        match value {
            Some(value) => Self::Value(value),
            None => Self::Missing,
        }
    }
}

impl From<Option<Method>> for Attribute {
    fn from(value: Option<Method>) -> Self {
        match value {
            Some(value) => Self::Method(value),
            None => Self::Missing,
        }
    }
}

/// Convert a method to `Attribute`.
pub trait ToAttribute<F, Dummy> {
    fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute>;
}

pub(crate) mod to_attribute_impl {
    use super::super::FromObject;
    use super::super::FromObjectRef;
    use super::super::IntoObject;
    use super::*;
    use crate::eval;

    // Marker types.
    pub struct ByRef;
    pub struct ByValue;
    pub struct ManyArgs;
    pub struct RawObject;
    pub struct ReturnOk;
    pub struct ReturnResult;
    pub struct WithContext;
    pub struct ReturnOkSelf;

    // 1 argument - property on self.

    impl<F, S, R> ToAttribute<F, (ByRef, ReturnResult)> for S
    where
        R: IntoObject + 'static,
        F: Fn(&S) -> Result<R>,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let _ = func_name;
            let value = func(self)?;
            Ok(Attribute::Value(value.into_object()))
        }
    }

    impl<F, S, R> ToAttribute<F, (ByRef, ReturnOk)> for S
    where
        R: IntoObject + 'static,
        F: Fn(&S) -> R,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let _ = func_name;
            let value = func(self);
            Ok(Attribute::Value(value.into_object()))
        }
    }

    // 2 arguments - binary operations.

    impl<F, S, O, R> ToAttribute<F, (ByRef, ByRef, ReturnResult, O)> for S
    where
        S: FromObjectRef + 'static,
        O: FromObjectRef + 'static,
        R: IntoObject + 'static,
        F: Fn(&S, &O) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                eval::ensure_arg_count(func_name, args, 2..=2)?;
                let lhs = context.eval_expr(&args[0])?;
                let rhs = context.eval_expr(&args[1])?;
                let lhs = S::from_object(&lhs)?;
                let rhs = O::from_object(&rhs)?;
                Ok(func(lhs, rhs)?.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    impl<F, S, O, R> ToAttribute<F, (ByRef, Option<ByRef>, ReturnResult, O)> for S
    where
        S: FromObjectRef + 'static,
        O: FromObjectRef + 'static,
        R: IntoObject + 'static,
        F: Fn(&S, Option<&O>) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                eval::ensure_arg_count(func_name, args, 1..=2)?;
                let lhs = context.eval_expr(&args[0])?;
                let rhs = match args.get(1) {
                    Some(arg) => Some(context.eval_expr(arg)?),
                    None => None,
                };
                let lhs = S::from_object(&lhs)?;
                let result = match rhs {
                    Some(rhs) => {
                        let rhs = O::from_object(&rhs)?;
                        func(lhs, Some(rhs))?
                    }
                    None => func(lhs, None)?,
                };
                Ok(result.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    impl<F, S, O, R> ToAttribute<F, (WithContext, ByRef, ByRef, ReturnResult, O)> for S
    where
        S: FromObjectRef + 'static,
        O: FromObjectRef + 'static,
        R: IntoObject + 'static,
        F: Fn(&S, &eval::EvalContext, &O) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                eval::ensure_arg_count(func_name, args, 2..=2)?;
                let lhs = context.eval_expr(&args[0])?;
                let rhs = context.eval_expr(&args[1])?;
                let lhs = S::from_object(&lhs)?;
                let rhs = O::from_object(&rhs)?;
                Ok(func(lhs, context, rhs)?.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    impl<F, S, R> ToAttribute<F, (WithContext, ByRef, RawObject, ReturnResult)> for S
    where
        S: FromObjectRef + 'static,
        R: IntoObject + 'static,
        F: Fn(&S, &eval::EvalContext, Object) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                eval::ensure_arg_count(func_name, args, 2..=2)?;
                let lhs = context.eval_expr(&args[0])?;
                let rhs = context.eval_expr(&args[1])?;
                let lhs = S::from_object(&lhs)?;
                Ok(func(lhs, context, rhs)?.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    impl<F, S, R> ToAttribute<F, (ByRef, RawObject, ReturnResult)> for S
    where
        S: FromObjectRef + 'static,
        R: IntoObject + 'static,
        F: Fn(&S, Object) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                eval::ensure_arg_count(func_name, args, 2..=2)?;
                let lhs = context.eval_expr(&args[0])?;
                let rhs = context.eval_expr(&args[1])?;
                let lhs = S::from_object(&lhs)?;
                Ok(func(lhs, rhs)?.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    impl<F, S, R> ToAttribute<F, (WithContext, ByRef, Expr, ReturnResult)> for S
    where
        S: FromObjectRef + 'static,
        R: IntoObject + 'static,
        F: Fn(&S, Expr) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                eval::ensure_arg_count(func_name, args, 2..=2)?;
                let lhs = context.eval_expr(&args[0])?;
                let lhs = S::from_object(&lhs)?;
                Ok(func(lhs, args[1].clone())?.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    impl<F, S, O, R> ToAttribute<F, (ByRef, ByRef, ReturnOk, O)> for S
    where
        S: FromObjectRef + 'static,
        O: FromObjectRef + 'static,
        R: IntoObject + 'static,
        F: Fn(&S, &O) -> R,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                eval::ensure_arg_count(func_name, args, 2..=2)?;
                let lhs = context.eval_expr(&args[0])?;
                let rhs = context.eval_expr(&args[1])?;
                let lhs = S::from_object(&lhs)?;
                let rhs = O::from_object(&rhs)?;
                Ok(func(lhs, rhs).into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    // 2 or 3 arguments

    impl<F, T1, T2, T3, R> ToAttribute<F, (ByRef, ByRef, Option<ByRef>, ReturnResult, T2, T3)> for T1
    where
        T1: FromObjectRef + 'static,
        T2: FromObjectRef + 'static,
        T3: FromObjectRef + 'static,
        R: IntoObject + 'static,
        F: Fn(&T1, &T2, Option<&T3>) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                eval::ensure_arg_count(func_name, args, 2..=3)?;
                let v1 = context.eval_expr(&args[0])?;
                let v1 = T1::from_object(&v1)?;
                let v2 = context.eval_expr(&args[1])?;
                let v2 = T2::from_object(&v2)?;
                let v3 = match args.get(2) {
                    Some(arg) => Some(context.eval_expr(arg)?),
                    None => None,
                };
                let v3 = match &v3 {
                    Some(v) => Some(T3::from_object(v)?),
                    None => None,
                };
                Ok(func(v1, v2, v3)?.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    // 3 arguments

    impl<F, T1, R> ToAttribute<F, (ByRef, Object, Object, ReturnResult)> for T1
    where
        T1: FromObjectRef + 'static,
        R: IntoObject + 'static,
        F: Fn(&T1, Object, Object) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                eval::ensure_arg_count(func_name, args, 3..=3)?;
                let v1 = context.eval_expr(&args[0])?;
                let v1 = T1::from_object(&v1)?;
                let v2 = context.eval_expr(&args[1])?;
                let v3 = context.eval_expr(&args[2])?;
                Ok(func(v1, v2, v3)?.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    impl<F, T1, T2, R> ToAttribute<F, (ByRef, ByRef, Object, ReturnResult, T2)> for T1
    where
        T1: FromObjectRef + 'static,
        T2: FromObjectRef + 'static,
        R: IntoObject + 'static,
        F: Fn(&T1, &T2, Object) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                eval::ensure_arg_count(func_name, args, 3..=3)?;
                let v1 = context.eval_expr(&args[0])?;
                let v1 = T1::from_object(&v1)?;
                let v2 = context.eval_expr(&args[1])?;
                let v2 = T2::from_object(&v2)?;
                let v3 = context.eval_expr(&args[2])?;
                Ok(func(v1, v2, v3)?.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    // n arguments of a same type.

    impl<F, S, R> ToAttribute<F, (ManyArgs, ByRef, ReturnOk, R)> for S
    where
        S: FromObjectRef,
        R: IntoObject + 'static,
        F: Fn(&[&S]) -> R,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let _ = func_name;
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                let args: Vec<Object> = args
                    .iter()
                    .map(|arg| context.eval_expr(arg))
                    .collect::<Result<Vec<_>>>()?;
                let args: Vec<&S> = args
                    .iter()
                    .map(|v| S::from_object(v))
                    .collect::<Result<Vec<_>>>()?;
                Ok(func(&args).into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    impl<F, S, R, X> ToAttribute<F, (ManyArgs, ByValue, ReturnOk, S, R)> for X
    where
        S: FromObject,
        R: IntoObject + 'static,
        F: Fn(&[S]) -> R,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let _ = func_name;
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                let args: Vec<Object> = args
                    .iter()
                    .map(|arg| context.eval_expr(arg))
                    .collect::<Result<Vec<_>>>()?;
                let args: Vec<S> = args
                    .iter()
                    .map(|v| S::from_object(v))
                    .collect::<Result<Vec<_>>>()?;
                Ok(func(&args).into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    impl<F, S, R> ToAttribute<F, (ManyArgs, ByRef, ReturnResult, R)> for S
    where
        S: FromObjectRef,
        R: IntoObject + 'static,
        F: Fn(&[&S]) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let _ = func_name;
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                let args: Vec<Object> = args
                    .iter()
                    .map(|arg| context.eval_expr(arg))
                    .collect::<Result<Vec<_>>>()?;
                let args: Vec<&S> = args
                    .iter()
                    .map(|v| S::from_object(v))
                    .collect::<Result<Vec<_>>>()?;
                Ok(func(&args)?.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    impl<F, S, R, X> ToAttribute<F, (ManyArgs, ByValue, ReturnResult, S, R)> for X
    where
        S: FromObject,
        R: IntoObject + 'static,
        F: Fn(&[S]) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let _ = func_name;
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                let args: Vec<Object> = args
                    .iter()
                    .map(|arg| context.eval_expr(arg))
                    .collect::<Result<Vec<_>>>()?;
                let args: Vec<S> = args
                    .iter()
                    .map(|v| S::from_object(v))
                    .collect::<Result<Vec<_>>>()?;
                Ok(func(&args)?.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }

    // General form.

    impl<F, S, R> ToAttribute<F, (ManyArgs, RawObject, ReturnResult)> for S
    where
        S: 'static,
        R: IntoObject + 'static,
        F: Fn(&eval::EvalContext, &[Expr]) -> Result<R>,
        F: 'static,
    {
        fn to_attribute(&self, func: F, func_name: &'static str) -> Result<Attribute> {
            let wrapper = move |context: &eval::EvalContext, args: &[Expr]| -> Result<Object> {
                let _ = func_name;
                func(context, args).map(|o| o.into_object())
            };
            Ok(Attribute::Method(Box::new(wrapper)))
        }
    }
}
