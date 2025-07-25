// src/main.rs

mod cli;
mod kube_api;
mod path_parser;

use anyhow::Result;
use clap::Parser;
use kube::{
    api::{Api, DynamicObject, ListParams},
    Client,
};
use serde_json::Value;

// HashSet을 사용하기 위해 std::collections를 가져옵니다.
use std::collections::HashSet;

use crate::cli::Cli;
use crate::kube_api::resolve_api_resource;
use crate::path_parser::{extract_by_path, find_all_paths, value_to_string};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let api_resource = resolve_api_resource(&cli.object_type).map_err(|e| anyhow::anyhow!(e))?;
    let client = Client::try_default().await?;

    let api: Api<DynamicObject> = if cli.all_namespaces {
        Api::all_with(client, &api_resource)
    } else {
        let namespace = client.default_namespace();
        println!("현재 네임스페이스 '{}'에서 검색합니다. 모든 네임스페이스를 보려면 -A 옵션을 사용하세요.", namespace);
        Api::namespaced_with(client.clone(), namespace, &api_resource)
    };

    let list = api.list(&ListParams::default()).await?;

    // 모드 1: -k (키 추출) - 변경 없음
    if !cli.keys.is_empty() {
        for item in list.items {
            let name = item.metadata.name.as_deref().unwrap_or("<noname>");
            let namespace = item.metadata.namespace.as_deref().unwrap_or("<none>");
            let item_value: Value = serde_json::to_value(&item)?;

            let values: Vec<String> = cli
                .keys
                .iter()
                .map(|key| {
                    extract_by_path(&item_value, key)
                        .map(value_to_string)
                        .unwrap_or_else(|| "".to_string())
                })
                .collect();
            println!("{}\t{}\t{}", namespace, name, values.join("\t"));
        }
    }
    // 모드 2: -g (경로 검색) - 천재적으로 개선된 로직
    else if let Some(ref grep_term) = cli.grep {
        // 중복을 허용하지 않는 HashSet을 사용하여 유일한 경로만 저장합니다.
        let mut unique_paths = HashSet::new();
        let lower_grep_term = grep_term.to_lowercase();

        // 모든 리소스를 순회하며 일치하는 경로를 HashSet에 추가합니다.
        for item in list.items {
            let item_value: Value = serde_json::to_value(&item)?;
            let all_paths = find_all_paths(&item_value);
            for path in all_paths {
                if path.to_lowercase().contains(&lower_grep_term) {
                    unique_paths.insert(path);
                }
            }
        }

        // 최종적으로, 중복이 제거된 경로들을 벡터로 변환하고 정렬합니다.
        let mut sorted_paths: Vec<_> = unique_paths.into_iter().collect();
        sorted_paths.sort();

        // 정렬된 경로 목록을 출력합니다.
        for path in sorted_paths {
            println!("{}", path);
        }
    }

    Ok(())
}