use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "link-checker", about = "Краулер для проверки ссылок на сайте")]
pub struct Cli{

    /// Стартовый URL, например https://example.com
    #[arg(long)]
    pub url: String,
    /// Максимальная глубина обхода
    #[arg(long, default_value_t = 3)]
    pub depth: usize,

    /// Максимум страниц
    #[arg(long, default_value_t = 500)]
    pub max_pages: usize,

    /// Таймаут запроса в секундах
    #[arg(long, default_value_t = 10)]
    pub timeout: u64,

    /// Одновременных запросов
    #[arg(long, default_value_t = 10)]
    pub concurrency: usize,

    /// Куда сохранить JSON-отчёт (если не указано — не сохраняем)
    #[arg(long)]
    pub output: Option<String>,
}
