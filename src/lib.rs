use indexmap::IndexMap;
use std::collections::HashMap;

pub mod parser;

#[derive(Debug, PartialEq)]
pub enum JsonType {
    Int,
    Float,
    Boolean,
    String,
    Object,
    Array,
    Null,
}

#[derive(Debug)]
pub struct JsonValue {
    val: u64,
    typ: JsonType,
}

#[derive(Debug)]
pub struct JsonObject {
    values: IndexMap<String, JsonValue>,
}

#[derive(Debug)]
pub struct JsonContext {
    strings: HashMap<u64, String>,
    string_id: u64,
    objects: HashMap<u64, JsonObject>,
    object_id: u64,
    arrays: HashMap<u64, Vec<JsonValue>>,
    array_id: u64,
    initial_obj: bool,
}

impl JsonValue {
    pub fn get_type(&self) -> &JsonType {
        return &self.typ;
    }
}

impl JsonContext {
    pub fn new(initial_obj: bool) -> (Self, u64) {
        let mut objs = HashMap::<u64, JsonObject>::new();
        let mut arrs = HashMap::<u64, Vec<JsonValue>>::new();

        let mut obj_id = 0;
        let mut arr_id = 0;

        if initial_obj {
            objs.insert(
                0,
                JsonObject {
                    values: IndexMap::new(),
                },
            );
            obj_id += 1;
        } else {
            arrs.insert(0, Vec::new());
            arr_id += 1;
        }

        (
            Self {
                strings: HashMap::new(),
                string_id: 0,
                objects: objs,
                object_id: obj_id,
                arrays: arrs,
                array_id: arr_id,
                initial_obj,
            },
            0,
        )
    }

    pub fn to_string(&self, beautify: bool) -> String {
        let mut str = String::new();
        if self.initial_obj {
            let root = self.objects.get(&0u64).unwrap();
            str.push_str(root.to_string(&self, 0, beautify).as_str());
        } else {
            let arr_val = JsonValue {
                val: 0u64,
                typ: JsonType::Array,
            };
            JsonObject::str_push_value(&self, &mut str, &arr_val, 0, beautify);
        }
        return str;
    }

    fn obtain_object(&self, obj_id: u64) -> &JsonObject {
        self.objects.get(&obj_id).unwrap()
    }

    fn obtain_object_mut(&mut self, obj_id: u64) -> &mut JsonObject {
        self.objects.get_mut(&obj_id).unwrap()
    }

    fn validate_contains(&self, obj_id: u64, key: &String) {
        if !self.contains(obj_id, &key) {
            panic!("fksjson: erasing key '{key}' that does not exist.");
        }
    }

    fn clear_obj_vals(&mut self, values: &mut IndexMap<String, JsonValue>) {
        for (key, _) in values.iter() {
            let val = values.get(key).unwrap();
            match val.typ {
                JsonType::String => {
                    self.strings.remove(&val.val);
                }
                JsonType::Object => {
                    let mut obj_rem = self.objects.remove(&val.val).unwrap();
                    self.clear_obj_vals(&mut obj_rem.values);
                }
                _ => {}
            }
        }

        values.clear();
    }

    fn erase_no_panic(&mut self, obj_id: u64, key: &String) {
        if !self.objects.contains_key(&obj_id) || !self.contains(obj_id, key) {
            return;
        }

        let mut value_op: Option<JsonValue> = None;

        {
            let obj = self.objects.get_mut(&obj_id);
            match obj {
                Some(x) => {
                    value_op.replace(x.values.shift_remove(key).unwrap());
                }
                None => {
                    return;
                }
            }
        }

        let value = value_op.unwrap();

        match value.typ {
            JsonType::String => {
                self.strings.remove(&value.val);
            }
            JsonType::Object => {
                let obj_rem_id = value.val;
                let mut obj_rem = self.objects.remove(&obj_rem_id).unwrap();

                self.clear_obj_vals(&mut obj_rem.values);
            }
            _ => {}
        }
    }

    /*fn validate_not_contains(&self, key: &String) {
        if self.contains(&key) {
            panic!("fksjson: adding key '{key}' that already exist.");
        }
    }*/

    pub fn contains(&self, obj_id: u64, key: &String) -> bool {
        self.obtain_object(obj_id).values.contains_key(key)
    }

    pub fn contains_str(&self, obj_id: u64, key: &str) -> bool {
        self.obtain_object(obj_id)
            .values
            .contains_key(&key.to_string())
    }

    pub fn val_int(&self, val: i64) -> JsonValue {
        JsonValue {
            val: val as u64,
            typ: JsonType::Int,
        }
    }

    pub fn val_float(&self, val: f64) -> JsonValue {
        JsonValue {
            val: f64::to_bits(val),
            typ: JsonType::Float,
        }
    }

    pub fn val_bool(&self, val: bool) -> JsonValue {
        JsonValue {
            val: if val { 1 } else { 0 },
            typ: JsonType::Boolean,
        }
    }

    pub fn val_null(&self) -> JsonValue {
        JsonValue {
            val: 0,
            typ: JsonType::Null,
        }
    }

    pub fn val_string(&mut self, val: String) -> JsonValue {
        let str_id = self.string_id;
        let value = JsonValue {
            val: str_id,
            typ: JsonType::String,
        };

        self.strings.insert(self.string_id, val);
        self.string_id += 1;

        value
    }

    pub fn val_obj(&mut self) -> (JsonValue, u64) {
        let object_id = self.object_id;
        let value = JsonValue {
            val: object_id,
            typ: JsonType::Object,
        };

        let idx = self.object_id;
        self.objects.insert(
            self.object_id,
            JsonObject {
                values: IndexMap::new(),
            },
        );
        self.object_id += 1;

        (value, idx)
    }

    pub fn val_array(&mut self) -> (JsonValue, u64) {
        let arr_id = self.array_id;
        let value = JsonValue {
            val: arr_id,
            typ: JsonType::Array,
        };

        let idx = self.array_id;
        self.arrays.insert(self.array_id, Vec::new());
        self.array_id += 1;

        (value, idx)
    }

    pub fn set_val(&mut self, obj_id: u64, key: String, val: JsonValue) {
        self.erase_no_panic(obj_id, &key);

        self.obtain_object_mut(obj_id).values.insert(key, val);
    }

    pub fn get_int(&self, val: &JsonValue) -> i64 {
        if val.typ != JsonType::Int {
            panic!("fksjson: expected integer value.");
        }

        val.val as i64
    }

    pub fn get_float(&self, val: &JsonValue) -> f64 {
        if val.typ != JsonType::Float {
            panic!("fksjson: expected floating point value.");
        }

        f64::from_bits(val.val)
    }

    pub fn get_bool(&self, val: &JsonValue) -> bool {
        if val.typ != JsonType::Boolean {
            panic!("fksjson: expected boolean value.");
        }

        val.val != 0
    }

    pub fn get_string(&self, val: &JsonValue) -> &String {
        if val.typ != JsonType::String {
            panic!("fksjson: expected string value.");
        }

        self.strings.get(&val.val).unwrap()
    }

    pub fn get_obj(&self, val: &JsonValue) -> &JsonObject {
        if val.typ != JsonType::Object {
            panic!("fksjson: expected object value.");
        }

        self.objects.get(&val.val).unwrap()
    }

    pub fn get_val(&self, obj_id: u64, key: &String) -> &JsonValue {
        let val = self.objects.get(&obj_id).unwrap().values.get(key);
        return match val {
            Some(v) => v,
            None => panic!("fksjson: key '{key}' does not exist [get]."),
        };
    }

    fn array_vec_get(&self, arr_id: u64) -> &Vec<JsonValue> {
        let vec_op = self.arrays.get(&arr_id);
        match vec_op {
            Some(x) => x,
            None => panic!("fksjson: reading array that does not exist. (id={arr_id})"),
        }
    }

    fn array_vec_get_mut(&mut self, arr_id: u64) -> &mut Vec<JsonValue> {
        let vec_op = self.arrays.get_mut(&arr_id);
        match vec_op {
            Some(x) => x,
            None => panic!("fksjson: reading array that does not exist. (id={arr_id})"),
        }
    }

    pub fn array_push(&mut self, arr_id: u64, val: JsonValue) {
        self.array_vec_get_mut(arr_id).push(val);
    }

    pub fn array_insert(&mut self, arr_id: u64, idx: usize, val: JsonValue) {
        self.array_vec_get_mut(arr_id).insert(idx, val);
    }

    pub fn array_pop(&mut self, arr_id: u64) -> JsonValue {
        let arr = self.array_vec_get_mut(arr_id);
        if arr.len() == 0 {
            panic!("fksjson: array is empty [pop]. (id={arr_id})");
        }

        arr.pop().unwrap()
    }

    pub fn array_remove(&mut self, arr_id: u64, idx: usize) -> JsonValue {
        let arr = self.array_vec_get_mut(arr_id);
        if idx >= arr.len() {
            panic!(
                "fksjson: array index out of bounds of length {} [remove]. (id={arr_id}, idx={idx})",
                arr.len()
            );
        }

        arr.remove(idx)
    }

    pub fn array_top(&self, arr_id: u64) -> &JsonValue {
        let arr = self.array_vec_get(arr_id);
        if arr.len() == 0 {
            panic!("fksjson: array is empty [top]. (id={arr_id})");
        }

        arr.last().unwrap()
    }

    pub fn array_at(&self, arr_id: u64, idx: usize) -> &JsonValue {
        let arr = self.array_vec_get(arr_id);
        if idx >= arr.len() {
            panic!(
                "fksjson: array index out of bounds of length {} [at]. (id={arr_id}, idx={idx})",
                arr.len()
            );
        }

        &arr[idx]
    }

    pub fn is_null(&self, obj_id: u64, key: String) -> bool {
        self.validate_contains(obj_id, &key);
        let val = self.get_val(obj_id, &key);

        val.typ == JsonType::Null
    }

    pub fn erase(&mut self, obj_id: u64, key: String) {
        self.validate_contains(obj_id, &key);
        self.erase_no_panic(obj_id, &key);
    }
}

impl JsonObject {    
    fn push_string(str: &mut String, val: &String) {
        str.push('\"');

        for c in val.chars() {
            match c {
                '\"' => str.push_str("\\\""),
                '\'' => str.push_str("\\\'"),
                '\\' => str.push_str("\\\\"),
                '\n' => str.push_str("\\n"),
                '\t' => str.push_str("\\t"),
                '\x0B' => str.push_str("\\v"),
                '\r' => str.push_str("\\r"),
                '\0' => str.push_str("\\0"),
                '\x08' => str.push_str("\\b"),
                '\x0C' => str.push_str("\\f"),
                _ => {
                    let c_num = c as u32;
                    if c_num >= 128 {
                        str.push_str("\\u");

                        for i in (0..4).rev() {
                            let shift = i * 4;
                            let digit = ((c_num >> shift) & 0xF) as u8;
                            if digit < 10 {
                                str.push((digit + b'0') as char);
                            } else {
                                str.push((digit + b'A' - 10u8) as char);
                            }
                        }
                    } else {
                        str.push(c);
                    }
                }
            }
        }
        
        str.push('\"');
    }
    
    pub(crate) fn str_push_value(
        cxt: &JsonContext,
        str: &mut String,
        value: &JsonValue,
        tab: usize,
        beautify: bool,
    ) {
        match value.typ {
            JsonType::Int => str.push_str(&(value.val as i64).to_string()),
            JsonType::Float => str.push_str(format!("{:?}", &f64::from_bits(value.val)).as_str()),
            JsonType::Boolean => str.push_str(if value.val == 1 { &"true" } else { &"false" }),
            JsonType::String => Self::push_string(str, cxt.strings.get(&value.val).unwrap()),
            JsonType::Null => str.push_str("null"),
            JsonType::Object => str.push_str(
                &cxt.objects
                    .get(&value.val)
                    .unwrap()
                    .to_string(cxt, tab, beautify),
            ),
            JsonType::Array => {
                str.push_str("[");
                let arr = cxt.arrays.get(&value.val).unwrap();

                for val in arr.iter() {
                    if beautify {
                        str.push_str("\n");
                        for _ in 0..(tab + 1) {
                            str.push_str("\t");
                        }
                    }
                    JsonObject::str_push_value(cxt, str, val, tab + 1, beautify);

                    str.push_str(",");
                }

                if !arr.is_empty() {
                    str.pop();
                }

                if beautify {
                    str.push_str("\n");
                    for _ in 0..tab {
                        str.push_str("\t");
                    }
                }

                str.push_str("]");
            }
        }
    }

    fn to_string(&self, cxt: &JsonContext, p_tab: usize, beautify: bool) -> String {
        let mut tab = p_tab;
        let mut str = String::new();

        str.push_str("{");
        tab += 1;

        for (key, value) in &self.values {
            if beautify {
                str.push_str("\n");
                for _ in 0..tab {
                    str.push_str("\t");
                }
            }

            Self::push_string(&mut str, &key);
            
            str.push(':');
            if beautify {
                str.push_str(" ");
            }

            JsonObject::str_push_value(cxt, &mut str, value, tab, beautify);
            str.push_str(",");
        }

        if str.chars().last().unwrap() == ',' {
            str.pop(); //pop the comma
        }

        if beautify {
            str.push_str("\n");
            for _ in 0..(tab - 1) {
                str.push_str("\t");
            }
        }

        str.push_str("}");

        return str;
    }
}
