use super::func::Func;
use super::Interpreter;
use super::native;
use super::value::Value;

impl Interpreter {
    pub fn build_native_method(&self, base: Value, method_name: String) -> Value {
        let func = match (base.clone(), &method_name[..]) {
            (_, "string") => Func::Native {
                func: native::string,
                params: Some(1),
                name: "string".to_string(),
                receiver: Some(Box::new(base)),
            },
            (_, "type") => Func::Native {
                func: native::typ,
                params: Some(1),
                name: "type".to_string(),
                receiver: Some(Box::new(base)),
            },
            (_, "toJson") => Func::Native {
                func: native::to_json,
                params: Some(1),
                name: "toJson".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "fromJson") => Func::Native {
                func: native::from_json,
                params: Some(1),
                name: "fromJson".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "strip") => Func::Native {
                func: native::strip,
                params: Some(1),
                name: "strip".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "contains") => Func::Native {
                func: native::string_contains,
                params: Some(2),
                name: "contains".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "lower") => Func::Native {
                func: native::lower,
                params: Some(1),
                name: "lower".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "upper") => Func::Native {
                func: native::upper,
                params: Some(1),
                name: "upper".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "bool") => Func::Native {
                func: native::bool,
                params: Some(1),
                name: "bool".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "num") => Func::Native {
                func: native::num,
                params: Some(1),
                name: "num".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "replace") => Func::Native {
                func: native::replace,
                params: Some(3),
                name: "replace".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "split") => Func::Native {
                func: native::split,
                params: Some(2),
                name: "split".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "join") => Func::Native {
                func: native::join,
                params: Some(2),
                name: "join".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "matches") => Func::Native {
                func: native::matches,
                params: Some(2),
                name: "matches".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "find") => Func::Native {
                func: native::find,
                params: Some(2),
                name: "find".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::String(_), "len") => Func::Native {
                func: native::string_len,
                params: Some(1),
                name: "len".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Vec(_), "len") => Func::Native {
                func: native::vec_len,
                params: Some(1),
                name: "len".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Dict(_), "len") => Func::Native {
                func: native::dict_len,
                params: Some(1),
                name: "len".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Vec(_), "map") => Func::Native {
                func: native::map,
                params: Some(2),
                name: "map".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Vec(_), "filter") => Func::Native {
                func: native::filter,
                params: Some(2),
                name: "filter".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Vec(_), "forEach") => Func::Native {
                func: native::for_each,
                params: Some(2),
                name: "forEach".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Vec(_), "clone") => Func::Native {
                func: native::clone_vec,
                params: Some(1),
                name: "clone".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Dict(_), "clone") => Func::Native {
                func: native::clone_dict,
                params: Some(1),
                name: "clone".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Vec(_), "dict") => Func::Native {
                func: native::vec_2_dict,
                params: Some(1),
                name: "dict".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Dict(_), "vec") => Func::Native {
                func: native::dict_2_vec,
                params: Some(1),
                name: "vec".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Vec(_), "contains") => Func::Native {
                func: native::vec_contains,
                params: Some(2),
                name: "contains".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Dict(_), "contains") => Func::Native {
                func: native::dict_contains,
                params: Some(2),
                name: "contains".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Vec(_), "remove") => Func::Native {
                func: native::vec_remove,
                params: Some(2),
                name: "remove".to_string(),
                receiver: Some(Box::new(base)),
            },
            (Value::Dict(_), "remove") => Func::Native {
                func: native::dict_remove,
                params: Some(2),
                name: "remove".to_string(),
                receiver: Some(Box::new(base)),
            },
            _ => panic!("no method found with this name"),
        };
        Value::Func(func)
    }
}
