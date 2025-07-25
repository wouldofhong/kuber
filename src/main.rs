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

use crate::cli::Cli;
use crate::kube_api::resolve_api_resource;
use crate::path_parser::{extract_by_path, value_to_string};

async fn process_items(api: &Api<DynamicObject>, keys: &[String]) -> Result<()> {
    let list = api.list(&ListParams::default()).await?;

    for item in list.items {
        let name = item.metadata.name.as_deref().unwrap_or("<noname>");
        let namespace = item.metadata.namespace.as_deref().unwrap_or("<none>");

        let item_value: Value = serde_json::to_value(&item)?;

        let values: Vec<String> = keys
            .iter()
            .map(|key| {
                extract_by_path(&item_value, key)
                    .map(value_to_string)
                    .unwrap_or_else(|| "".to_string())
            })
            .collect();

        println!("{}\t{}\t{}", namespace, name, values.join("\t"));
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let api_resource = resolve_api_resource(&cli.object_type).map_err(|e| anyhow::anyhow!(e))?;
    let client = Client::try_default().await?;

    if cli.all_namespaces {
        // Api::all_with는 client를 소유하지 않고 빌려가므로 clone이 필요 없습니다.
        let api: Api<DynamicObject> = Api::all_with(client, &api_resource);
        process_items(&api, &cli.keys).await?;
    } else {
        let namespace = client.default_namespace();
        // 수정된 부분: Api::namespaced_with에 복제된 client를 전달합니다.
        let api: Api<DynamicObject> = Api::namespaced_with(client.clone(), namespace, &api_resource);
        println!("현재 네임스페이스 '{}'에서 검색합니다. 모든 네임스페이스를 보려면 -A 옵션을 사용하세요.", namespace);
        process_items(&api, &cli.keys).await?;
    }

    Ok(())
}