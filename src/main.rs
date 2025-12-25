mod client;
mod crt;

use crate::client::{execute_request, load_config_file, Config, Format, COLUMN_COMMON_NAME, COLUMN_ENTRY_TIMESTAMP, COLUMN_ID, COLUMN_ISSUER_CA_ID, COLUMN_ISSUER_NAME, COLUMN_NAME_VALUE, COLUMN_NOT_AFTER, COLUMN_NOT_BEFORE, COLUMN_RESULT_COUNT, COLUMN_SERIAL_NUMBER};
use crate::client::{DEFAULT_RETRY_COUNT, DEFAULT_RETRY_DELAY, DEFAULT_TIMEOUT_SECS, DEFAULT_FORMAT};
use clap::Parser;
use std::error::Error;

const API_URL: &str = "https://crt.sh/?q=example.com&output=json";
const ERROR_MISSING_URL: &str = "URL is required. Use -u/--url option or specify in config file.";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to a configuration file
    #[arg(short, long)]
    config: Option<String>,

    /// List of column names to display in a table
    #[arg(long = "column_name", action = clap::ArgAction::Append)]
    column_names: Vec<String>,

    /// Output format (table or raw)
    #[arg(short, long, default_value = DEFAULT_FORMAT)]
    format: Option<String>,

    /// Hostname to search for
    #[arg(long)]
    hostname: Option<String>,

    /// Preset name from a configuration file
    #[arg(long)]
    preset: Option<String>,

    /// Number of retry attempts
    #[arg(long, default_value_t = DEFAULT_RETRY_COUNT)]
    retry: u32,

    /// Delay between retries in seconds
    #[arg(long, default_value_t = DEFAULT_RETRY_DELAY)]
    retry_delay: f64,

    /// Timeout duration in seconds
    #[arg(short, long, default_value_t = DEFAULT_TIMEOUT_SECS)]
    timeout: u64,

    /// Display timing information
    #[arg(long, default_value_t = false)]
    timing: bool,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // 設定ファイルの読み込み
    let mut config = load_config_if_specified(&args)?;

    // コマンドライン引数で設定ファイルの値をオーバーライド
    apply_args_to_config(&mut config, args);

    // URLが設定されていない場合はエラー
    validate_config(&config)?;

    // HTTP リクエスト実行
    execute_request(config)?;

    Ok(())
}

/// 設定ファイルが指定されている場合に読み込む
fn load_config_if_specified(args: &Args) -> Result<Config, Box<dyn Error>> {
    match &args.config {
        Some(config_path) => load_config_file(config_path, args.preset.as_deref()),
        None => Ok(Config::default()),
    }
}

/// 設定の有効性を検証
fn validate_config(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.url.is_empty() {
        return Err(ERROR_MISSING_URL.into());
    }
    Ok(())
}

/// コマンドライン引数を設定に反映
fn apply_args_to_config(config: &mut Config, args: Args) {
    // apply_auth_config(config, &args);
    // apply_data_config(config, &args);
    apply_request_config(config, &args);
    // apply_proxy_config(config, &args);
    apply_output_config(config, &args);
    apply_retry_config(config, &args);
    apply_flags(config, &args);
}

/// リクエスト設定の適用
fn apply_request_config(config: &mut Config, args: &Args) {
    if let Some(hostname) = &args.hostname {
        config.url = API_URL.replace("example.com", hostname);
    }

    if args.timeout != DEFAULT_TIMEOUT_SECS {
        config.timeout = args.timeout;
    }
}

/// 出力設定の適用
fn apply_output_config(config: &mut Config, args: &Args) {
    if let Some(format) = &args.format {
        config.format = match format.as_str() {
            "table" => Format::Table,
            "raw" => Format::Raw,
            _ => Format::Table,
        };
    }

    if args.column_names.len() > 0 {
        config.column_names = args.column_names.clone();

        return;
    }

    config.column_names = vec![
        COLUMN_ID.to_string(),
        COLUMN_COMMON_NAME.to_string(),
        COLUMN_ENTRY_TIMESTAMP.to_string(),
        COLUMN_ISSUER_CA_ID.to_string(),
        COLUMN_ISSUER_NAME.to_string(),
        COLUMN_NAME_VALUE.to_string(),
        COLUMN_NOT_BEFORE.to_string(),
        COLUMN_NOT_AFTER.to_string(),
        COLUMN_RESULT_COUNT.to_string(),
        COLUMN_SERIAL_NUMBER.to_string(),
    ];
}

// リトライ設定の適用
fn apply_retry_config(config: &mut Config, args: &Args) {
    if args.retry != DEFAULT_RETRY_COUNT {
        config.retry = args.retry;
    }

    if args.retry_delay != DEFAULT_RETRY_DELAY {
        config.retry_delay = args.retry_delay;
    }
}

// フラグの適用
fn apply_flags(config: &mut Config, args: &Args) {
    if args.timing {
        config.timing = true;
    }

    if args.verbose {
        config.verbose = true;
    }
}