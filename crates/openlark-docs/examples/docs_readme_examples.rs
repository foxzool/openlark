//! Compile-check companion for the public `openlark-docs` README snippets.

use openlark_core::{SDKResult, config::Config};
use openlark_docs::DocsClient;
use openlark_docs::baike::baike::v1::GetEntityRequest;
use openlark_docs::base::bitable::v1::GetAppRequest;
use openlark_docs::ccm::docs::v1::GetDocsContentRequest;
use openlark_docs::ccm::drive::v1::file::DownloadFileRequest;

#[tokio::main]
async fn main() -> SDKResult<()> {
    let config = Config::builder()
        .app_id("app_id")
        .app_secret("app_secret")
        .build();
    let docs = DocsClient::new(config);

    // README: 基础使用
    let docs_request = GetDocsContentRequest::new("doc_token", "docx", "markdown");
    let _docs_content_future = openlark_docs::ccm::docs::v1::content::get::get_docs_content(
        docs_request,
        docs.config(),
        None,
    );

    // README: 多维表格使用
    let _bitable_request = GetAppRequest::new(docs.config().clone()).app_token("app_token");

    // README: 云文档协同使用
    let _download_request = DownloadFileRequest::new(docs.config().clone(), "file_token");

    // README: 知识库使用
    let _entity_request = GetEntityRequest::new(docs.config().clone(), "entity_id");

    Ok(())
}
