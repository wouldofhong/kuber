// src/path_parser.rs

use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

lazy_static! {
    // 1. '.0' 같은 형식을 '[0]'으로 변환하기 위한 정규식
    static ref DOT_INDEX_RE: Regex = Regex::new(r"\.([0-9]+)").unwrap();

    // 2. 경로의 각 부분을 추출하기 위한 정규식
    //    - 그룹 1: ["quoted.key"]
    //    - 그룹 2: [0]
    //    - 그룹 3: simplekey
    static ref PATH_PART_RE: Regex = Regex::new(r#"\["([^"\]]+)"\]|\[([0-9]+)\]|([^.\[\]]+)"#).unwrap();
}

/// 주어진 경로를 따라 JSON Value에서 값을 추출합니다.
/// 지원 형식:
/// - simple.key
/// - array.0.key
/// - mixed.array.0.map["complex-key.with/slash"]
pub fn extract_by_path<'a>(obj: &'a Value, path: &str) -> Option<&'a Value> {
    // 먼저 '.0'과 같은 인덱스를 '[0]'으로 변경하여 일관성을 유지합니다.
    let processed_path = DOT_INDEX_RE.replace_all(path, "[$1]");

    let mut current = obj;
    // 정규식을 사용해 경로의 각 부분을 순회합니다.
    for caps in PATH_PART_RE.captures_iter(&processed_path) {
        let part_matched = caps.get(1).or(caps.get(2)).or(caps.get(3));

        if let Some(part) = part_matched {
            let key_or_index = part.as_str();

            // 현재 값이 객체(map)인지 확인
            if let Some(obj_map) = current.as_object() {
                if let Some(val) = obj_map.get(key_or_index) {
                    current = val;
                    continue;
                }
            }

            // 현재 값이 배열인지 확인
            if let Some(arr) = current.as_array() {
                if let Ok(index) = key_or_index.parse::<usize>() {
                    if let Some(val) = arr.get(index) {
                        current = val;
                        continue;
                    }
                }
            }

            // 어느 쪽에서도 값을 찾지 못하면 즉시 None 반환
            return None;
        }
    }
    Some(current)
}

/// serde_json::Value를 깔끔한 문자열로 변환합니다.
pub fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "".to_string(), // .을 ::으로 수정했습니다.
        _ => v.to_string(),
    }
}