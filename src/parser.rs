//the code to parse a string of json into a struct
use crate::JsonContext;
use std::result::Result;
use std::str::Chars;

/**
* unexpected character, line, pos
*/
#[derive(Debug)]
pub enum ParserError {
    EmptyString,
    KeyExists(String, usize, usize),
    BadBeginning(String, usize, usize),
    UnexpectedEOL(usize, usize),
    UnexpectedEndOfTokens(),
    ExpectedChar(char, usize, usize),
    ExpectedWord(String, usize, usize),
    UnexpectedChar(char, usize, usize),
    UnexpectedToken(String, usize, usize),
    StringUnicode(char, usize, usize),
    StringEscapeChar(char, usize, usize),
    IntegerInvalidDecimal(char, usize, usize),
    IntegerInvalidBinary(char, usize, usize),
    IntegerInvalidOctal(char, usize, usize),
    IntegerInvalidHex(char, usize, usize),
    Test,
}

struct Position {
    line: usize,
    pos: usize,
}

impl Position {
    fn new() -> Self {
        Self { line: 1, pos: 0 }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TokenType {
    OBrk,  //{
    CBrk,  //}
    OSBrk, //[
    CSBrk, //]
    Colon, //:
    Comma, //,
    True,
    False,
    Null,
    Number,
    String,
}

#[derive(Debug)]
struct Token {
    value: String,
    typ: TokenType,
    line: usize,
    pos: usize,
}

impl Token {
    fn new(value: String, typ: TokenType, line: usize, pos: usize) -> Self {
        Self {
            value,
            typ,
            line,
            pos,
        }
    }

    fn new_str(value: &str, typ: TokenType, line: usize, pos: usize) -> Self {
        Self::new(value.to_string(), typ, line, pos)
    }

    fn new_ch(value: char, typ: TokenType, line: usize, pos: usize) -> Self {
        Self::new(value.to_string().to_string(), typ, line, pos)
    }
}

fn eat_whitespace(chars: &mut Chars, pos: &mut Position) -> (bool, char) {
    'main_loop: loop {
        let c = chars.next();

        match c {
            Some(x) => {
                if x == '\n' {
                    pos.line += 1;
                    pos.pos = 0;
                    continue 'main_loop;
                } else {
                    pos.pos += 1;
                }

                if x != ' ' && x != '\t' {
                    return (false, x);
                }
            }
            None => {
                return (true, '\0');
            }
        }
    }
}

fn parse_num(str: &String, line: usize, pos: usize) -> Result<(i64, f64, bool), ParserError> {
    let mut chars = str.chars();
    let beg_op = chars.next();

    match beg_op {
        Some(mut beg) => {
            let mut offset = 0;
            let negative = beg == '-';
            if negative || beg == '+' {
                match chars.next() {
                    Some(c) => {
                        beg = c;
                    }
                    None => {
                        return Err(ParserError::UnexpectedEOL(line, pos + 1));
                    }
                }
            }
            
            if str.contains('.')
                || str.contains('f')
                || str.contains('F')
                || str.contains('d')
                || str.contains('D')
                || str.contains('e')
                || str.contains('E')
            {
                //float
                let mut num = 0f64;
                let mut exp = 0.1f64;
                let mut decimal = false;

                let beg_num = beg as u8;
                if beg_num >= b'0' && beg_num <= b'9' {
                    num = (beg_num - b'0') as f64;
                } else if beg == '.' {
                    decimal = true;
                } else {
                    return Err(ParserError::UnexpectedChar(beg, line, pos + offset));
                }

                while let Some(c) = chars.next() {
                    offset += 1;

                    if c == '.' {
                        if decimal {
                            return Err(ParserError::UnexpectedChar(c, line, pos + offset));
                        }
                        decimal = true;
                        continue;
                    }

                    let c_num = c as u8;
                    if c_num >= b'0' && c_num <= b'9' {
                        if decimal {
                            num += (c_num - b'0') as f64 * exp;
                            exp *= 0.1;
                        } else {
                            num *= 10.0;
                            num += (c_num - b'0') as f64;
                        }
                    } else if c == 'f' || c == 'F' || c == 'd' || c == 'D' {
                        break;
                    } else if c == 'e' || c == 'E' {
                        offset += 1;
                        match chars.next() {
                            Some(beg_e) => {
                                let mut exp = 0isize;
                                let mut exp_neg = false;
                                if beg_e == '+' || beg_e == '-' {
                                    if beg_e == '-' {
                                        exp_neg = true;
                                    }
                                } else if beg_e.is_numeric() {
                                    exp = (beg_e as u8 - b'0') as isize;
                                } else {
                                    return Err(ParserError::UnexpectedChar(
                                        beg_e,
                                        line,
                                        pos + offset,
                                    ));
                                }

                                while let Some(char_e) = chars.next() {
                                    offset += 1;
                                    let char_e_num = char_e as u8;
                                    if char_e_num >= b'0' && char_e_num <= b'9' {
                                        exp = exp * 10 + (char_e_num - b'0') as isize;
                                    } else {
                                        return Err(ParserError::UnexpectedChar(
                                            char_e,
                                            line,
                                            pos + offset,
                                        ));
                                    }
                                }

                                if exp_neg {
                                    for _ in 0..exp {
                                        num *= 0.1;
                                    }
                                } else {
                                    for _ in 0..exp {
                                        num *= 10.0;
                                    }
                                }

                                break;
                            }
                            None => {
                                return Err(ParserError::UnexpectedEOL(line, pos + offset));
                            }
                        }
                    } else {
                        return Err(ParserError::UnexpectedChar(c, line, pos + offset));
                    }
                }

                let next_char = chars.next();
                if next_char.is_some() {
                    return Err(ParserError::UnexpectedChar(
                        next_char.unwrap(),
                        line,
                        pos + offset + 1,
                    ));
                }

                if negative {
                    num = -num;
                }
                
                return Ok((0, num, true));
            } else {
                //number
                let mut num = 0u64;
                let mut radix = 10;

                if beg == '0' {
                    let typ_op = chars.next();

                    if typ_op == None {
                        return Ok((0, 0.0, false));
                    }

                    let typ = typ_op.unwrap();

                    match typ {
                        'x' | 'X' => radix = 16,
                        'b' | 'B' => radix = 2,
                        _ => {
                            radix = 8;

                            let typ_b = typ as u8;
                            if typ_b >= b'0' && typ_b <= b'7' {
                                num = (typ_b - b'0') as u64;
                            } else if typ != 'o' && typ != 'O' {
                                return Err(ParserError::IntegerInvalidOctal(
                                    typ,
                                    line,
                                    pos + offset + 1,
                                ));
                            }
                        }
                    }

                    offset += 2;
                } else {
                    let beg_b = beg as u8;
                    if beg_b >= b'0' && beg_b <= b'9' {
                        num = (beg_b - b'0') as u64;
                    } else {
                        return Err(ParserError::IntegerInvalidOctal(beg, line, pos + offset));
                    }
                    offset += 1;
                }

                while let Some(c) = chars.next() {
                    let c_num = c as u8;

                    offset += 1;

                    match radix {
                        2 => {
                            if c != '0' && c != '1' {
                                return Err(ParserError::IntegerInvalidBinary(
                                    c,
                                    line,
                                    pos + offset,
                                ));
                            }
                            num <<= 1;
                        }
                        8 => {
                            if c_num < b'0' || c_num > b'7' {
                                return Err(ParserError::IntegerInvalidOctal(
                                    c,
                                    line,
                                    pos + offset,
                                ));
                            }
                            num <<= 3;
                        }
                        10 => {
                            if c_num < b'0' || c_num > b'9' {
                                return Err(ParserError::IntegerInvalidDecimal(
                                    c,
                                    line,
                                    pos + offset,
                                ));
                            }
                            num *= 10;
                        }
                        16 => {
                            if !(c_num >= b'0' || c_num <= b'9')
                                && !(c_num >= b'a' || c_num <= b'f')
                                && !(c_num >= b'A' || c_num <= b'F')
                            {
                                return Err(ParserError::IntegerInvalidHex(c, line, pos + offset));
                            }
                            num <<= 4;
                        }
                        _ => { /* should not happen */ }
                    }

                    if c_num >= b'0' && c_num <= b'9' {
                        num += (c_num - b'0') as u64;
                    } else if c_num >= 'a' as u8 && c_num <= 'f' as u8 {
                        num += (c_num - b'a' + 10) as u64;
                    } else if c_num >= 'A' as u8 && c_num <= 'F' as u8 {
                        num += (c_num - b'A' + 10) as u64;
                    } else {
                        break;
                    }
                }

                return Ok((
                    num as i64 * if negative { -1 } else { 1 },
                    num as f64,
                    false,
                ));
            }
        }
        None => {
            Ok((0, 0.0, false)) //empty string, technically not possible from the tokens code
        }
    }
}

fn parse_string(chars: &mut Chars, pos: &mut Position) -> Result<String, ParserError> {
    let mut key = String::new();

    loop {
        pos.pos += 1;
        let ch_op = chars.next();

        match ch_op {
            Some(ch) => {
                //TODO escape character handling
                if ch == '\"' {
                    break;
                } else if ch == '\\' {
                    pos.pos += 1;
                    match chars.next() {
                        Some(esc) => match esc {
                            '\"' => key.push('\"'),
                            '\'' => key.push('\''),
                            '\\' => key.push('\\'),
                            'n' => key.push('\n'),
                            't' => key.push('\t'),
                            'v' => key.push('\x0B'),
                            'r' => key.push('\r'),
                            '0' => key.push('\0'),
                            'b' => key.push('\x08'),
                            'f' => key.push('\x0C'),
                            'x' | 'u' => {
                                let mut code = 0u32;

                                for _ in 0..(if esc == 'x' { 2 } else { 4 }) {
                                    pos.pos += 1;
                                    let ch_esc = chars.next();

                                    match ch_esc {
                                        Some(x) => {
                                            code <<= 4;

                                            if x >= '0' && x <= '9' {
                                                code |= ((x as u8) - b'0') as u32;
                                            } else if x >= 'a' && x <= 'f' {
                                                code |= ((x as u8) - b'a' + 10) as u32;
                                            } else if x >= 'A' && x <= 'F' {
                                                code |= ((x as u8) - b'A' + 10) as u32;
                                            } else {
                                                return Err(ParserError::StringUnicode(
                                                    x, pos.line, pos.pos,
                                                ));
                                            }
                                        }
                                        None => {
                                            return Err(ParserError::UnexpectedEOL(
                                                pos.line, pos.pos,
                                            ));
                                        }
                                    }
                                }

                                key.push(char::from_u32(code).unwrap());
                            }
                            _ => {
                                return Err(ParserError::StringEscapeChar(esc, pos.line, pos.pos));
                            }
                        },
                        None => {
                            return Err(ParserError::UnexpectedEOL(pos.line, pos.pos));
                        }
                    }
                } else {
                    key.push(ch);
                }
            }
            None => {
                break;
            }
        }
    }

    return Ok(key);
}

fn pop_token(tokens: &mut Vec<Token>, expect: TokenType) -> Result<Token, ParserError> {
    match tokens.pop() {
        Some(token) => {
            if token.typ != expect {
                return Err(ParserError::UnexpectedToken(
                    token.value,
                    token.line,
                    token.pos,
                ));
            }

            Ok(token)
        }
        None => Err(ParserError::UnexpectedEndOfTokens()),
    }
}

fn parse_obj(cxt: &mut JsonContext, obj_id: u64, tokens: &mut Vec<Token>) -> Option<ParserError> {
    let mut err: Option<ParserError> = None;

    'obj: loop {
        match tokens.pop() {
            Some(tkn) => match tkn.typ {
                TokenType::String => {
                    let key = tkn.value;

                    if cxt.contains(obj_id, &key) {
                        err.replace(ParserError::KeyExists(key, tkn.line, tkn.pos));
                        break 'obj;
                    }

                    match pop_token(tokens, TokenType::Colon) {
                        Ok(_) => match tokens.pop() {
                            Some(value_tkn) => {
                                match value_tkn.typ {
                                    TokenType::True => {
                                        cxt.set_val(obj_id, key, cxt.val_bool(true));
                                    }
                                    TokenType::False => {
                                        cxt.set_val(obj_id, key, cxt.val_bool(false));
                                    }
                                    TokenType::Null => {
                                        cxt.set_val(obj_id, key, cxt.val_null());
                                    }
                                    TokenType::Number => match parse_num(
                                        &value_tkn.value,
                                        value_tkn.line,
                                        value_tkn.pos,
                                    ) {
                                        Ok(int) => {
                                            if int.2 {
                                                cxt.set_val(obj_id, key, cxt.val_float(int.1));
                                            } else {
                                                cxt.set_val(obj_id, key, cxt.val_int(int.0));
                                            }
                                        }
                                        Err(e) => {
                                            err.replace(e);
                                            break 'obj;
                                        }
                                    },
                                    TokenType::String => {
                                        let value = cxt.val_string(value_tkn.value);
                                        cxt.set_val(obj_id, key, value);
                                    }
                                    TokenType::OBrk => {
                                        let (nobj, nobj_id) = cxt.val_obj();
                                        cxt.set_val(obj_id, key, nobj);
                                        match parse_obj(cxt, nobj_id, tokens) {
                                            Some(e) => {
                                                err.replace(e);
                                                break 'obj;
                                            }
                                            None => {}
                                        }
                                    }
                                    TokenType::OSBrk => {
                                        let (arr, arr_id) = cxt.val_array();
                                        cxt.set_val(obj_id, key, arr);
                                        match parse_arr(cxt, arr_id, tokens) {
                                            Some(e) => {
                                                err.replace(e);
                                                break 'obj;
                                            }
                                            None => {}
                                        }
                                    }
                                    _ => {
                                        err.replace(ParserError::UnexpectedToken(
                                            value_tkn.value,
                                            value_tkn.line,
                                            value_tkn.pos,
                                        ));
                                        break 'obj;
                                    }
                                }

                                match tokens.pop() {
                                    Some(end_tkn) => {
                                        if end_tkn.typ != TokenType::Comma
                                            && end_tkn.typ != TokenType::CBrk
                                        {
                                            err.replace(ParserError::ExpectedChar(
                                                ',',
                                                end_tkn.line,
                                                end_tkn.pos,
                                            ));
                                            break 'obj;
                                        } else if end_tkn.typ == TokenType::CBrk {
                                            break 'obj;
                                        }
                                    }
                                    None => {
                                        err.replace(ParserError::UnexpectedEndOfTokens());
                                        break 'obj;
                                    }
                                }
                            }
                            None => {
                                err.replace(ParserError::UnexpectedEndOfTokens());
                                break 'obj;
                            }
                        },
                        Err(e) => {
                            err.replace(e);
                            break 'obj;
                        }
                    }
                }
                TokenType::CBrk => {
                    break 'obj;
                }
                _ => {
                    err.replace(ParserError::UnexpectedToken(tkn.value, tkn.line, tkn.pos));
                    break 'obj;
                }
            },
            None => {
                err.replace(ParserError::UnexpectedEndOfTokens());
                break 'obj;
            }
        }
    }

    err
}

fn parse_arr(cxt: &mut JsonContext, arr_id: u64, tokens: &mut Vec<Token>) -> Option<ParserError> {
    let mut err: Option<ParserError> = None;

    'arr: loop {
        match tokens.pop() {
            Some(tkn) => {
                match tkn.typ {
                    TokenType::True => {
                        cxt.array_push(arr_id, cxt.val_bool(true));
                    }
                    TokenType::False => {
                        cxt.array_push(arr_id, cxt.val_bool(false));
                    }
                    TokenType::Null => {
                        cxt.array_push(arr_id, cxt.val_null());
                    }
                    TokenType::Number => match parse_num(&tkn.value, tkn.line, tkn.pos) {
                        Ok(int) => {
                            if int.2 {
                                cxt.array_push(arr_id, cxt.val_float(int.1));
                            } else {
                                cxt.array_push(arr_id, cxt.val_int(int.0));
                            }
                        }
                        Err(e) => {
                            err.replace(e);
                            return err;
                        }
                    },
                    TokenType::String => {
                        let value = cxt.val_string(tkn.value);
                        cxt.array_push(arr_id, value);
                    }
                    TokenType::OBrk => {
                        let (nobj, nobj_id) = cxt.val_obj();
                        cxt.array_push(arr_id, nobj);
                        match parse_obj(cxt, nobj_id, tokens) {
                            Some(e) => {
                                err.replace(e);
                                return err;
                            }
                            None => {}
                        }
                    }
                    TokenType::CSBrk => {
                        break 'arr;
                    }
                    TokenType::OSBrk => {
                        let (narr, narr_id) = cxt.val_array();
                        cxt.array_push(arr_id, narr);
                        match parse_arr(cxt, narr_id, tokens) {
                            Some(e) => {
                                err.replace(e);
                                break 'arr;
                            }
                            None => {}
                        }
                    }
                    _ => {
                        err.replace(ParserError::UnexpectedToken(tkn.value, tkn.line, tkn.pos));
                        return err;
                    }
                }

                match tokens.pop() {
                    Some(end_tkn) => {
                        if end_tkn.typ != TokenType::Comma && end_tkn.typ != TokenType::CSBrk {
                            err.replace(ParserError::ExpectedChar(',', end_tkn.line, end_tkn.pos));
                            break 'arr;
                        } else if end_tkn.typ == TokenType::CSBrk {
                            break 'arr;
                        }
                    }
                    None => {
                        err.replace(ParserError::UnexpectedEndOfTokens());
                        break 'arr;
                    }
                }
            }
            None => {
                err.replace(ParserError::UnexpectedEndOfTokens());
                break 'arr;
            }
        }
    }

    err
}

fn lexer(p_chars: Chars) -> Result<Vec<Token>, ParserError> {
    let mut tokens = Vec::<Token>::new();
    let mut chars = p_chars;
    let mut pos = Position::new();

    let mut num_pos = Position::new();
    let mut parse_num = false;
    let mut num_str = String::new();

    loop {
        let (failed, ch) = eat_whitespace(&mut chars, &mut pos);
        if failed {
            if parse_num {
                //parse_num = false;
                tokens.push(Token::new(
                    num_str.clone(),
                    TokenType::Number,
                    num_pos.line,
                    num_pos.pos,
                ));
                // num_str.clear();
            }
            break;
        }

        if parse_num {
            if ch.is_alphanumeric() || ch == '.' || ch == '+' || ch == '-' {
                num_str.push(ch);
                continue;
            } else {
                parse_num = false;
                tokens.push(Token::new(
                    num_str.clone(),
                    TokenType::Number,
                    num_pos.line,
                    num_pos.pos,
                ));
                num_str.clear();
            }
        }

        match ch {
            '{' => tokens.push(Token::new_ch(ch, TokenType::OBrk, pos.line, pos.pos)),
            '}' => tokens.push(Token::new_ch(ch, TokenType::CBrk, pos.line, pos.pos)),
            '[' => tokens.push(Token::new_ch(ch, TokenType::OSBrk, pos.line, pos.pos)),
            ']' => tokens.push(Token::new_ch(ch, TokenType::CSBrk, pos.line, pos.pos)),
            ':' => tokens.push(Token::new_ch(ch, TokenType::Colon, pos.line, pos.pos)),
            ',' => tokens.push(Token::new_ch(ch, TokenType::Comma, pos.line, pos.pos)),
            't' => {
                let (ol, op) = (pos.line, pos.pos);
                let (f0, c0) = eat_whitespace(&mut chars, &mut pos);
                let (f1, c1) = eat_whitespace(&mut chars, &mut pos);
                let (f2, c2) = eat_whitespace(&mut chars, &mut pos);
                if f0 || f1 || f2 || c0 != 'r' || c1 != 'u' || c2 != 'e' {
                    return Err(ParserError::ExpectedWord(
                        "true".to_string(),
                        pos.line,
                        pos.pos,
                    ));
                }

                tokens.push(Token::new_str("true", TokenType::True, ol, op));
            }
            'f' => {
                let (ol, op) = (pos.line, pos.pos);
                let (f0, c0) = eat_whitespace(&mut chars, &mut pos);
                let (f1, c1) = eat_whitespace(&mut chars, &mut pos);
                let (f2, c2) = eat_whitespace(&mut chars, &mut pos);
                let (f3, c3) = eat_whitespace(&mut chars, &mut pos);
                if f0 || f1 || f2 || f3 || c0 != 'a' || c1 != 'l' || c2 != 's' || c3 != 'e' {
                    return Err(ParserError::ExpectedWord(
                        "false".to_string(),
                        pos.line,
                        pos.pos,
                    ));
                }

                tokens.push(Token::new_str("false", TokenType::False, ol, op));
            }
            'n' => {
                let (ol, op) = (pos.line, pos.pos);
                let (f0, c0) = eat_whitespace(&mut chars, &mut pos);
                let (f1, c1) = eat_whitespace(&mut chars, &mut pos);
                let (f2, c2) = eat_whitespace(&mut chars, &mut pos);
                if f0 || f1 || f2 || c0 != 'u' || c1 != 'l' || c2 != 'l' {
                    return Err(ParserError::ExpectedWord(
                        "null".to_string(),
                        pos.line,
                        pos.pos,
                    ));
                }

                tokens.push(Token::new_str("null", TokenType::Null, ol, op));
            }
            '\"' => {
                let (ol, op) = (pos.line, pos.pos);
                match parse_string(&mut chars, &mut pos) {
                    Ok(str) => {
                        tokens.push(Token::new(str, TokenType::String, ol, op));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            '/' => {
                let (failedch, ch) = eat_whitespace(&mut chars, &mut pos);

                if failedch {
                    return Err(ParserError::UnexpectedEOL(pos.line, pos.pos));
                }

                if ch == '/' {
                    'comment_loop: loop {
                        match chars.next() {
                            Some(char) => {
                                if char == '\n' {
                                    break 'comment_loop;
                                }
                            }
                            None => {
                                break 'comment_loop;
                            }
                        }
                    }

                    pos.line += 1;
                    pos.pos = 0;
                } else if ch == '*' {
                    'comment_loop: loop {
                        match chars.next() {
                            Some(char) => {
                                pos.pos += 1;
                                if char == '*' {
                                    match chars.next() {
                                        Some(char1) => {
                                            if char1 == '/' {
                                                break 'comment_loop;
                                            }
                                        }
                                        None => {
                                            return Err(ParserError::UnexpectedEndOfTokens());
                                        }
                                    }
                                    break 'comment_loop;
                                } else if char == '\n' {
                                    pos.line += 1;
                                    pos.pos = 0;
                                }
                            }
                            None => {
                                return Err(ParserError::UnexpectedEndOfTokens());
                            }
                        }
                    }
                } else {
                    return Err(ParserError::UnexpectedChar(ch, pos.line, pos.pos));
                }
            }
            _ => {
                if ch.is_numeric() || ch == '-' || ch == '+' {
                    num_str.push(ch);
                    parse_num = true;
                    num_pos.line = pos.line;
                    num_pos.pos = pos.pos;
                } else {
                    return Err(ParserError::UnexpectedChar(ch, pos.line, pos.pos));
                }
            }
        }
    }

    Ok(tokens)
}

pub fn parse(str: &String) -> Result<(JsonContext, u64), ParserError> {
    if str.is_empty() {
        return Err(ParserError::EmptyString);
    }

    match lexer(str.chars()) {
        Ok(mut tokens) => {
            /*for token in tokens.iter() {
                println!("{}:{} {:?}", token.line, token.pos, token.value);
            }*/
            //println!("{tokens:?}");
            tokens.reverse();
            let token = tokens.pop().unwrap();

            if token.typ == TokenType::OBrk {
                let (mut cxt, root_id) = JsonContext::new(true);

                match parse_obj(&mut cxt, root_id, &mut tokens) {
                    Some(e) => {
                        return Err(e);
                    }
                    None => {}
                }

                if !tokens.is_empty() {
                    let tkn_end = tokens.pop().unwrap();
                    return Err(ParserError::UnexpectedToken(
                        tkn_end.value,
                        tkn_end.line,
                        tkn_end.pos,
                    ));
                }

                return Ok((cxt, root_id));
            } else if token.typ == TokenType::OSBrk {
                let (mut cxt, root_id) = JsonContext::new(false);

                match parse_arr(&mut cxt, root_id, &mut tokens) {
                    Some(e) => {
                        return Err(e);
                    }
                    None => {}
                }

                if !tokens.is_empty() {
                    let tkn_end = tokens.pop().unwrap();
                    return Err(ParserError::UnexpectedToken(
                        tkn_end.value,
                        tkn_end.line,
                        tkn_end.pos,
                    ));
                }
                return Ok((cxt, root_id));
            } else {
                return Err(ParserError::BadBeginning(
                    token.value,
                    token.line,
                    token.pos,
                ));
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}
