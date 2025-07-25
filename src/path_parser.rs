// src/path_parser.rs

use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

// --- 기존 코드는 변경 없음 ---
lazy_static! {
    static ref DOT_INDEX_RE: Regex = Regex::new(r"\.([0-9]+)").unwrap();
    static ref PATH_PART_RE: Regex = Regex::new(r#"\["([^"\]]+)"\]|\[([0-9]+)\]|([^.\[\]]+)"#).unwrap();
}

pub fn extract_by_path<'a>(obj: &'a Value, path: &str) -> Option<&'a Value> {
    // ... (이전과 동일)
    let processed_path = DOT_INDEX_RE.replace_all(path, "[$1]");
    let mut current = obj;
    for caps in PATH_PART_RE.captures_iter(&processed_path) {
        let part_matched = caps.get(1).or(caps.get(2)).or(caps.get(3));
        if let Some(part) = part_matched {
            let key_or_index = part.as_str();
            if let Some(obj_map) = current.as_object() {
                if let Some(val) = obj_map.get(key_or_index) {
                    current = val;
                    continue;
                }
            }
            if let Some(arr) = current.as_array() {
                if let Ok(index) = key_or_index.parse::<usize>() {
                    if let Some(val) = arr.get(index) {
                        current = val;
                        continue;
                    }
                }
            }
            return None;
        }
    }
    Some(current)
}

pub fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "".to_string(),
        _ => v.to_string(),
    }
}

// --- 새로 추가된 함수 ---

/// 재귀적으로 JSON 객체를 순회하며 모든 가능한 경로를 생성합니다.
///
/// # Arguments
/// * `current_value` - 현재 탐색 중인 `serde_json::Value`
/// * `current_path` - 루트부터 현재까지의 경로 조각들
/// * `all_paths` - 최종적으로 생성된 모든 경로 문자열을 저장할 벡터
fn find_paths_recursive(current_value: &Value, current_path: &mut Vec<String>, all_paths: &mut Vec<String>) {
    match current_value {
        Value::Object(map) => {
            for (key, value) in map {
                current_path.push(key.clone());
                find_paths_recursive(value, current_path, all_paths);
                // 백트래킹: 현재 키 탐색이 끝났으므로 경로에서 제거
                current_path.pop();
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                current_path.push(index.to_string());
                find_paths_recursive(value, current_path, all_paths);
                // 백트래킹
                current_path.pop();
            }
        }
        _ => {
            // 리프 노드에 도달하면 현재까지의 경로를 하나의 문자열로 합쳐서 저장
            if !current_path.is_empty() {
                all_paths.push(current_path.join("."));
            }
        }
    }
}

/// JSON 객체의 모든 경로를 찾아 벡터로 반환하는 공개 함수
pub fn find_all_paths(obj: &Value) -> Vec<String> {
    let mut all_paths = Vec::new();
    find_paths_recursive(obj, &mut Vec::new(), &mut all_paths);
    all_paths
}