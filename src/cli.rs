// src/cli.rs

use clap::Parser;

/// Kubernetes 리소스를 빠르고 유연하게 조회하는 도구
#[derive(Parser, Debug)]
#[clap(version, author="Your Name", about)]
pub struct Cli {
    /// 가져올 리소스 타입 (예: pod, svc, deploy, ing)
    #[clap(required = true)]
    pub object_type: String,

    /// 추출할 값의 경로 (쉼표로 구분)
    /// 예: spec.containers.0.image,metadata.labels.app
    #[clap(short, long, multiple_values = true, use_delimiter = true, required = true)]
    pub keys: Vec<String>,

    /// 이 플래그가 있으면, 모든 네임스페이스의 리소스를 조회합니다.
    /// 없으면 현재 컨텍스트의 기본 네임스페이스만 조회합니다.
    #[clap(short = 'A', long)]
    pub all_namespaces: bool,
}