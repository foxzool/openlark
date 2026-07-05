## meta API 调用链规范（openlark-client）

目标：在 `openlark-client` 中提供一种与 `api_list_export.csv` 一致、可预测的调用方式，让调用路径直接映射到 CSV 的 `meta.*` 字段。

### 1. 映射规则

数据源：仓库根目录的 `api_list_export.csv`，字段如下：

- `meta.Project`
- `meta.Version`
- `meta.Resource`
- `meta.Name`

调用链规则：

```
client.{meta.Project}.{meta.Version}.{meta.Resource}.{meta.Name}(...)
```

说明：

- `{meta.Project}` 对应一个“业务入口”（例如 `cardkit`）。
- `{meta.Version}` 对应版本层（例如 `v1`）。
- `{meta.Resource}` 对应资源层（例如 `card`、`card.element`；其中 `.` 映射为模块/字段层级）。
- `{meta.Name}` 对应最终方法（例如 `create`、`update`）。

### 2. CardKit 示例：创建卡片实体

CSV 对应行（示例）：

- `meta.Project=cardkit`
- `meta.Version=v1`
- `meta.Resource=card`
- `meta.Name=create`
- `url=POST:/open-apis/cardkit/v1/cards`
- `docPath=https://open.feishu.cn/document/cardkit-v1/card/create`

代码示例（需要启用 `cardkit` feature）：

```rust,no_run
use openlark_client::prelude::*;
use openlark_cardkit::cardkit::cardkit::v1::card::create::CreateCardBody;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;

    let req = CreateCardBody {
        card_content: json!({"elements": []}),
        card_type: None,
        template_id: None,
        temp: None,
        temp_expire_time: None,
    };

    let resp = client.cardkit.v1.card.create(req).await?;
    println!("card_id={:?}", resp.card_id);
    Ok(())
}
```

### 3. 现状与约束

- `openlark-client` 的 `client.cardkit...` 链式调用由 `openlark-cardkit` 提供并复用（避免在 client 内重复实现与模型）。
- 已覆盖 `bizTag=cardkit` 的 v1 调用链（与 `api_list_export.csv` 对齐）：
  - `client.cardkit.v1.card.create(body).await?`
  - `client.cardkit.v1.card.update(body).await?`
  - `client.cardkit.v1.card.batch_update(body).await?`
  - `client.cardkit.v1.card.settings(body).await?`
  - `client.cardkit.v1.card.id_convert(body).await?`
  - `client.cardkit.v1.card.element.create(body).await?`
  - `client.cardkit.v1.card.element.update(body).await?`
  - `client.cardkit.v1.card.element.patch(body).await?`
  - `client.cardkit.v1.card.element.content(body).await?`
  - `client.cardkit.v1.card.element.delete(body).await?`

### 4. docs / communication / meeting（模块级链式入口）

由于 `openlark-docs` / `openlark-communication` / `openlark-meeting` 的 API 规模较大，目前在 `openlark-client` 中提供的是“模块级链式入口 + Config 透传”，用于把调用入口统一收敛到：

- `client.docs...`
- `client.communication...`
- `client.meeting...`

你可以在链式入口上拿到 `Config`，然后按各业务 crate 的既有请求类型/Builder 继续调用。

示例：会议（需要启用 `meeting` feature）

```rust,no_run
use openlark_client::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;

    // 会议：字段链式入口（note 资源已接线；room/meeting/reserve 经 strict 路径访问）
    let req = client.meeting.vc.v1.note.get("note_id");
    let _resp = req.execute().await?;

    Ok(())
}
```
