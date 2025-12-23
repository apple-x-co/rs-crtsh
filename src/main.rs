mod client;
mod crt;

use crate::client::{execute_request, load_config_file, Config};
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
    // apply_retry_config(config, &args);
    apply_flags(config, &args);
}

/// 認証設定の適用
// fn apply_auth_config(config: &mut Config, args: &Args) {
//     if let (Some(basic_user), Some(basic_pass)) = (&args.basic_user, &args.basic_pass) {
//         config.basic_auth = Some(BasicAuthConfig {
//             user: basic_user.clone(),
//             pass: basic_pass.clone(),
//         });
//     }
// }

/// データ送信設定の適用
// fn apply_data_config(config: &mut Config, args: &Args) {
//     if let Some(form_data) = &args.form_data {
//         config.form_data = Some(form_data.clone());
//     }
//
//     if let Some(form) = &args.form {
//         config.form = Some(form.clone());
//     }
//
//     if let Some(json) = &args.json {
//         config.json = Some(json.clone());
//     }
//
//     if let Some(json_filter) = &args.json_filter {
//         config.json_filter = Some(json_filter.clone());
//     }
// }

/// リクエスト設定の適用
fn apply_request_config(config: &mut Config, args: &Args) {
    // if args.method != DEFAULT_METHOD {
    //     config.method = args.method.clone();
    // }
    //
    // if let Some(headers) = &args.headers {
    //     config.headers = Some(headers.clone());
    // }
    //
    // if let Some(cookies) = &args.cookies {
    //     config.cookies = Some(cookies.clone());
    // }

    if let Some(hostname) = &args.hostname {
        config.url = API_URL.replace("example.com", hostname);
    }

    // if args.timeout != DEFAULT_TIMEOUT_SECS {
    //     config.timeout = args.timeout;
    // }
}

///// プロキシ設定の適用
// fn apply_proxy_config(config: &mut Config, args: &Args) {
//     if let (Some(proxy_host), Some(proxy_port)) = (&args.proxy_host, &args.proxy_port) {
//         config.proxy = Some(ProxyConfig {
//             host: proxy_host.clone(),
//             port: proxy_port.clone(),
//             user: args.proxy_user.clone(),
//             pass: args.proxy_pass.clone(),
//         });
//     }
// }

///// 出力設定の適用
// fn apply_output_config(config: &mut Config, args: &Args) {
//     if let Some(output) = &args.output {
//         config.output = Some(output.clone());
//     }
// }

///// リトライ設定の適用
// fn apply_retry_config(config: &mut Config, args: &Args) {
//     if args.retry != DEFAULT_RETRY_COUNT {
//         config.retry = args.retry;
//     }
//
//     if args.retry_delay != DEFAULT_RETRY_DELAY {
//         config.retry_delay = args.retry_delay;
//     }
// }

// フラグの適用
fn apply_flags(config: &mut Config, args: &Args) {
//     if args.dry_run {
//         config.dry_run = true;
//     }
//
//     if args.pretty_json {
//         config.pretty_json = true;
//     }
//
//     if args.silent {
//         config.silent = true;
//     }
//
//     if args.timing {
//         config.timing = true;
//     }
//
    if args.verbose {
        config.verbose = true;
    }
}