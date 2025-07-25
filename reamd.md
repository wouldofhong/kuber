물론입니다. 동료나 상사에게 칭찬받을 만한, 이 프로젝트의 매력을 200% 보여줄 수 있는 `README.md`를 작성해 드리겠습니다. 이 README는 단순히 기능을 나열하는 것을 넘어, **왜 이 도구가 필요한지, 기존 방식보다 무엇이 뛰어난지, 그리고 얼마나 사용하기 편리한지**를 효과적으로 전달하는 데 초점을 맞췄습니다.

아래 내용을 복사해서 프로젝트 최상단에 `README.md` 파일로 저장하세요.

---

# 🚀 kuber: 당신의 새로운 Kubernetes 친구

**`kubectl`과 `jq`, `grep`을 번거롭게 조합하던 시대는 끝났습니다. `kuber`는 Rust로 작성된 초고속, 무의존성 단일 바이너리로, 터미널에서 쿠버네티스 리소스를 탐색하는 경험을 완전히 바꿔놓을 것입니다.**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/your-username/kuber)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/kuber.svg)](https://crates.io/crates/kuber)

`kuber`는 복잡한 `kubectl get ... -o json | jq ...` 파이프라인을 단 하나의 직관적인 명령어로 대체합니다. 특정 값을 빠르게 추출하거나, 리소스에 어떤 필드가 있는지 탐색하는 모든 작업을 빛의 속도로 처리하세요.

## ✨ 주요 기능

*   **압도적인 속도**: Rust로 작성되어 네이티브 성능을 자랑합니다. 셸 파이프라인보다 월등히 빠릅니다.
*   **단일 바이너리**: `jq`나 다른 도구에 대한 의존성 없이, `kuber` 파일 하나만 있으면 어디서든 동작합니다.
*   **두 가지 강력한 모드**:
    *   **추출 모드 (`-k`)**: 원하는 값의 경로를 정확히 지정하여 깔끔한 테이블 형식으로 출력합니다.
    *   **탐색 모드 (`-g`)**: 키워드로 리소스의 모든 하위 경로를 `grep`하여 사용 가능한 필드를 쉽게 탐색합니다.
*   **직관적인 경로 문법**: `spec.containers.0.image` 나 `metadata.annotations["app.kubernetes.io/name"]` 과 같은 복잡한 경로를 손쉽게 파싱합니다.
*   **폭넓은 리소스 지원**: `pod`, `svc`, `deploy` 등 `kubectl`이 지원하는 거의 모든 리소스 단축 이름(alias)을 인식합니다.
*   **간편한 네임스페이스 제어**: `-A` 플래그 하나로 전체 클러스터 또는 현재 네임스페이스를 자유롭게 넘나들 수 있습니다.

## 🎬 데모

`kuber`가 얼마나 편리한지 직접 확인해 보세요.

![kuber demo gif](https://user-images.githubusercontent.com/your-github-id/your-repo-id/kuber-demo.gif)
*(위 이미지는 예시입니다. [asciinema](https://asciinema.org/) 같은 도구로 실제 사용 장면을 녹화하여 GIF로 변환한 후 여기에 추가하세요.)*

## 📦 설치

**1. crates.io 사용 (권장)**

Rust 툴체인이 설치되어 있다면, 가장 쉬운 방법입니다.

```bash
cargo install kuber
```

**2. 소스에서 직접 빌드**

```bash
git clone https://github.com/your-username/kuber.git
cd kuber
cargo build --release
# ./target/release/kuber 를 원하는 경로로 복사하여 사용하세요.
```

**3. GitHub 릴리스**

프로젝트의 [릴리스 페이지](https://github.com/your-username/kuber/releases)에서 당신의 OS에 맞는 최신 바이너리를 다운로드할 수 있습니다. (추후 설정 필요)

## 💡 사용법

`kuber`의 사용법은 간단합니다: `kuber <리소스 타입> [모드]`

---

### 모드 1: 특정 값 추출 (`-k`, `--keys`)

정확한 필드 경로를 알고 있을 때 사용합니다. 출력은 `NAMESPACE | NAME | KEY1 | KEY2 ...` 형식의 탭으로 구분된 테이블입니다.

**예제 1: 모든 파드의 IP와 실행 중인 노드 이름 가져오기**

*   **The Old Way 😫**
    ```bash
    kubectl get pods -A -o jsonpath='{range .items[*]}{.metadata.namespace}{"\t"}{.metadata.name}{"\t"}{.status.podIP}{"\t"}{.spec.nodeName}{"\n"}{end}'
    ```

*   **The `kuber` Way 😎**
    ```bash
    kuber pod -A -k status.podIP,spec.nodeName
    ```

**예제 2: `default` 네임스페이스에 있는 디플로이먼트의 이미지와 복제본 수 확인**

```bash
# -A 플래그가 없으면 현재 네임스페이스에서만 검색합니다.
kuber deploy -k spec.template.spec.containers.0.image,status.availableReplicas
```

---

### 모드 2: 사용 가능한 경로 탐색 (`-g`, `--grep`)

어떤 필드를 사용해야 할지 모를 때, 리소스 구조를 탐색하기 위해 사용합니다. 키워드가 포함된 모든 고유한(unique) 경로를 찾아서 정렬된 목록으로 보여줍니다.

**예제 1: `Pod` 리소스에 `image` 라는 단어가 포함된 모든 필드 경로 찾아보기**

*   **The Old Way 😫**
    ```bash
    # 매우 길고 느린 명령어...
    kubectl get pod -A -o json | jq -c 'paths | map(tostring) | join(".")' | grep -iE '(image)' | sort -u
    ```

*   **The `kuber` Way 😎**
    ```bash
    kuber pod -A -g image
    ```
    **결과 예시:**
    ```
    spec.containers.0.image
    spec.initContainers.0.image
    status.containerStatuses.0.image
    status.containerStatuses.0.imageID
    status.initContainerStatuses.0.image
    ```

**예제 2: `Service`에 `port`가 포함된 경로 찾아보기**

```bash
kuber svc -A -g port
```

## 📚 지원 리소스

`kuber`는 다음과 같은 리소스 단축 이름을 지원합니다.

| 리소스              | 단축 이름 (Aliases)                               |
| ------------------- | ------------------------------------------------- |
| Pods                | `po`, `pod`                                       |
| Services            | `svc`, `service`                                  |
| Deployments         | `deploy`, `deployment`                            |
| Namespaces          | `ns`, `namespace`                                 |
| Nodes               | `no`, `node`                                      |
| ConfigMaps          | `cm`, `configmap`                                 |
| Secrets             | `secret`                                          |
| Ingresses           | `ing`, `ingress`                                  |
| ... *(이하 kube_api.rs에 정의된 모든 리소스)* |

## 🎯 동기

`kubectl`은 훌륭한 도구이지만, JSON 출력을 파싱하여 원하는 정보를 얻는 과정은 종종 번거롭습니다. `jq`와 같은 추가 도구가 필요하며, 셸 파이프라인은 길고 복잡해지기 쉽습니다.

`kuber`는 이 과정을 단순화하고, Rust의 강력한 성능과 안정성을 활용하여 개발자와 DevOps 엔지니어의 생산성을 높이기 위해 탄생했습니다.

## 🙌 기여하기

이 프로젝트는 이제 막 시작되었습니다! 버그 리포트, 기능 제안, PR 등 모든 종류의 기여를 환영합니다. [이슈 트래커](https://github.com/your-username/kuber/issues)에 자유롭게 의견을 남겨주세요.

## ⚖️ 라이선스

이 프로젝트는 [MIT 라이선스](LICENSE)를 따릅니다.