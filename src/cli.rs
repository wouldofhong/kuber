// src/cli.rs

use clap::{ArgGroup, Parser};

/// Kubernetes 리소스를 빠르고 유연하게 조회하는 도구
#[derive(Parser, Debug)]
#[clap(version, author, about)]
#[clap(group(
    ArgGroup::new("query_mode")
        .required(true)
        .args(&["keys", "grep"]),
))]
pub struct Cli {
    /// 가져올 리소스 타입 (예: pod, svc, deploy, ing)
    #[clap(required = true)]
    pub object_type: String,

    /// [모드 1] 추출할 값의 정확한 경로 (쉼표로 구분)
    #[clap(short, long, multiple_values = true, use_delimiter = true)]
    pub keys: Vec<String>,

    /// [모드 2] 경로에 포함될 키워드 검색 (대소문자 구분 없음)
    #[clap(short, long)]
    pub grep: Option<String>,

    /// 이 플래그가 있으면, 모든 네임스페이스의 리소스를 조회합니다.
    #[clap(short = 'A', long)]
    pub all_namespaces: bool,
}