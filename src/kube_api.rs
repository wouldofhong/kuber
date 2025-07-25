// src/kube_api.rs

use kube::api::{ApiResource, GroupVersionKind};

/// 사용자가 입력한 짧은 이름(alias)을 Kubernetes ApiResource로 변환합니다.
/// `kubectl api-resources`에서 볼 수 있는 대부분의 리소스를 지원합니다.
pub fn resolve_api_resource(alias: &str) -> Result<ApiResource, String> {
    let gvk = match alias.to_lowercase().as_str() {
        // Core v1
        "po" | "pod" | "pods" => ("core", "v1", "Pod"),
        "svc" | "service" | "services" => ("core", "v1", "Service"),
        "ns" | "namespace" | "namespaces" => ("core", "v1", "Namespace"),
        "no" | "node" | "nodes" => ("core", "v1", "Node"),
        "cm" | "configmap" | "configmaps" => ("core", "v1", "ConfigMap"),
        "secret" | "secrets" => ("core", "v1", "Secret"),
        "sa" | "serviceaccount" | "serviceaccounts" => ("core", "v1", "ServiceAccount"),
        "pv" | "persistentvolume" | "persistentvolumes" => ("core", "v1", "PersistentVolume"),
        "pvc" | "persistentvolumeclaim" | "persistentvolumeclaims" => ("core", "v1", "PersistentVolumeClaim"),
        "ep" | "endpoint" | "endpoints" => ("core", "v1", "Endpoints"),

        // Apps v1
        "deploy" | "deployment" | "deployments" => ("apps", "v1", "Deployment"),
        "sts" | "statefulset" | "statefulsets" => ("apps", "v1", "StatefulSet"),
        "ds" | "daemonset" | "daemonsets" => ("apps", "v1", "DaemonSet"),
        "rs" | "replicaset" | "replicasets" => ("apps", "v1", "ReplicaSet"),

        // Batch v1
        "job" | "jobs" => ("batch", "v1", "Job"),
        "cj" | "cronjob" | "cronjobs" => ("batch", "v1", "CronJob"),

        // Networking v1
        "ing" | "ingress" | "ingresses" => ("networking.k8s.io", "v1", "Ingress"),
        "netpol" | "networkpolicy" | "networkpolicies" => ("networking.k8s.io", "v1", "NetworkPolicy"),

        // Storage v1
        "sc" | "storageclass" | "storageclasses" => ("storage.k8s.io", "v1", "StorageClass"),

        // RBAC v1
        "role" | "roles" => ("rbac.authorization.k8s.io", "v1", "Role"),
        "clusterrole" | "clusterroles" => ("rbac.authorization.k8s.io", "v1", "ClusterRole"),
        "rolebinding" | "rolebindings" => ("rbac.authorization.k8s.io", "v1", "RoleBinding"),
        "clusterrolebinding" | "clusterrolebindings" => ("rbac.authorization.k8s.io", "v1", "ClusterRoleBinding"),

        // 여기에 필요한 다른 리소스 타입을 계속 추가할 수 있습니다.
        _ => return Err(format!("알 수 없는 리소스 타입: '{}'", alias)),
    };

    // "core" 그룹은 API 서버에서 빈 문자열로 인식됩니다.
    let group = if gvk.0 == "core" { "" } else { gvk.0 };

    Ok(ApiResource::from_gvk(&GroupVersionKind {
        group: group.to_string(),
        version: gvk.1.to_string(),
        kind: gvk.2.to_string(),
    }))
}