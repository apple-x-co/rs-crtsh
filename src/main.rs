mod client;
mod crt;

use crate::client::{execute_request, load_config_file, Config};
use crate::client::{DEFAULT_RETRY_COUNT, DEFAULT_RETRY_DELAY, DEFAULT_TIMEOUT_SECS};
use clap::Parser;
use std::error::Error;

const API_URL: &str = "https://crt.sh/?q=example.com&output=json";
const ERROR_MISSING_URL: &str = "URL is required. Use -u/--url option or specify in config file.";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(long)]
    preset: Option<String>,

    #[arg(long)]
    hostname: Option<String>,

    #[arg(long, default_value_t = DEFAULT_RETRY_COUNT)]
    retry: u32,

    #[arg(long, default_value_t = DEFAULT_RETRY_DELAY)]
    retry_delay: f64,

    #[arg(short, long, default_value_t = DEFAULT_TIMEOUT_SECS)]
    timeout: u64,

    #[arg(long, default_value_t = false)]
    timing: bool,

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
    // apply_output_config(config, &args);
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