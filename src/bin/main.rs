#[cfg(test)]
mod tests {
    use fksjson::*;

    #[test]
    pub fn main() {
        let (mut cxt, root_arr) = JsonContext::new(false);
        let (root, root_obj) = cxt.val_obj();
        cxt.array_push(root_arr, root);
        
        cxt.set_val(root_obj, "asd".to_string(), cxt.val_int(-56));
        cxt.set_val(root_obj, "try".to_string(), cxt.val_int(560));

        let (obj_val, obj) = cxt.val_obj();
        cxt.array_push(root_arr, obj_val);

        cxt.set_val(obj, "a float".to_string(), cxt.val_float(2340.0000001f64));
        cxt.set_val(
            obj,
            "another float".to_string(),
            cxt.val_float(-1230.1293f64),
        );
        cxt.set_val(root_obj, "life".to_string(), cxt.val_bool(true));
        cxt.set_val(obj, "death".to_string(), cxt.val_bool(false));
        let str = cxt.val_string("Felix".to_string());
        cxt.set_val(obj, "name".to_string(), str);
        cxt.set_val(obj, "null object".to_string(), cxt.val_null());

        let (arr_val, arr_id) = cxt.val_array();
        cxt.set_val(root_obj, "array".to_string(), arr_val);
        let (arr_obj, arr_obj_i) = cxt.val_obj();
        cxt.set_val(arr_obj_i, "try".to_string(), cxt.val_int(56));
        cxt.set_val(arr_obj_i, "what".to_string(), cxt.val_float(-23f64));
        cxt.array_push(arr_id, arr_obj);
        cxt.array_insert(arr_id, 0, cxt.val_bool(false));
        cxt.array_remove(arr_id, 0);

        //root.root.erase("a float".to_string());
        cxt.erase(obj, "another float".to_string());

        for _ in 0..1000 {
            cxt.set_val(obj, "test".to_string(), cxt.val_int(-5));
            let (o, _) = cxt.val_obj();
            cxt.set_val(obj, "test_object_2".to_string(), o);
        }

        let output_json = cxt.to_string(true);
        println!(
            "{}\n{}\n{}\n{}",
            output_json,
            cxt.get_int(cxt.get_val(root_obj, &"asd".to_string())),
            cxt.get_float(cxt.get_val(obj, &"a float".to_string())),
            cxt.get_bool(cxt.get_val(obj, &"death".to_string())),
        );
    }

    #[test]
    fn parsing() {
        let json = std::fs::read_to_string("test.json").unwrap();
        let txt = parser::parse(&json)
                 .unwrap().0.to_string(false);
        
        println!("\n\n{}\n\n", parser::parse(&txt).unwrap().0.to_string(true));
    }
}

fn main() {}
