use reqwest::Client;
use serde::Serialize;
use std::env;
use std::error::Error;

#[derive(Serialize)]
struct Document {
    emp_id: String,
    name: String,
    department: String,
    position: String,
    salary: f64,
}

async fn add_document_to_opensearch(doc: &Document, index: &str) -> Result<(), Box<dyn Error>> {
    // 取得使用者名稱和密碼
    let username = env::var("OPENSEARCH_INITIAL_ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let password = env::var("OPENSEARCH_INITIAL_ADMIN_PASSWORD").expect("密碼未設置在環境變數中");

    // OpenSearch URL
    let url = format!("https://localhost:9200/{}/_doc", index);

    // 建立 HTTP 客戶端並設置身份驗證
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    // 發送請求並帶上帳號密碼進行身份驗證
    let res = client
        .post(&url)
        .basic_auth(username, Some(password))
        .json(&doc)
        .send()
        .await?;

    if res.status().is_success() {
        println!("Document added successfully!");
    } else {
        println!("Failed to add document. Status: {}", res.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化文件資料
    let doc = Document {
        emp_id: "E123".to_string(),
        name: "John Doe".to_string(),
        department: "Engineering".to_string(),
        position: "Software Engineer".to_string(),
        salary: 85000.0,
    };

    // 指定索引名稱
    let index = "employees";

    // 新增文件到 OpenSearch
    add_document_to_opensearch(&doc, index).await?;
    Ok(())
}

