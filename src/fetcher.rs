use std::time::Duration;

use reqwest::{Client, StatusCode};
use url::Url;

/// Результат одного fetch-запроса.
/// Сетевые ошибки сюда НЕ попадают — они в Result. Здесь только "сервер ответил".
#[derive(Debug, Clone)]
pub struct FetchResult {
    /// Исходный URL, который просили.
    pub requested_url: Url,

    /// Финальный URL после редиректов (может отличаться от requested_url).
    pub final_url: Url,

    /// HTTP-статус ответа.
    pub status: StatusCode,

    /// Тело ответа, если Content-Type был HTML. Иначе None.
    pub html: Option<String>,
}

/// Создаёт HTTP-клиент с таймаутом и user-agent.
/// Один клиент на весь обход — переиспользует соединения (keep-alive).
///Тут могу переписать редирект на количество переходов либо вообще, запретить редирект
pub fn build_client(timeout_secs: u64) -> anyhow::Result<Client> {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .user_agent(concat!(
            "link-cheker/",
            env!("CARGO_PKG_VERSION"),
            " (contact : email@example.com)"
        ))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;
    Ok(client)
}

/// Делает GET-запрос и возвращает FetchResult.
///
/// Возвращает Err ТОЛЬКО при сетевой ошибке (DNS, таймаут, TLS, refused).
/// HTTP-статусы (404, 500 и т.д.) — это Ok(FetchResult).
pub async fn fetch(client: &Client, url: &Url) -> anyhow::Result<FetchResult> {
    // 1. Отправляем GET. `?` пробросит сетевую ошибку наверх.
    let response = client
        .get(url.as_str())
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Сетевая ошибка длф {url} : {e}"))?;
    // 2. Запоминаем статус и финальный URL (после редиректов).
    let status = response.status();
    let final_url = response.url().clone();

    let html = if status.is_success() || status.is_redirection() {
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("xXx");

        if content_type.contains("text/html") {
            match response.text().await {
                Ok(body) => Some(body),
                Err(e) => {
                    tracing::warn!("не удалось прочитать тело {url}: {e}");
                    None
                }
            }
        } else {
            None
        }
    } else {
        None
    };

    Ok(FetchResult {
        requested_url: url.clone(),
        final_url,
        status,
        html,
    })
}
