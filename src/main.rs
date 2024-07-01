//1.引入clap
//2.创建结构体Cli接收命令行参数

use std::{path::Path, vec};

use clap::{Parser, Subcommand};
use anyhow::Result;
use csv::{self, StringRecord};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Parser)]
#[command(name="rcli", version, author, about="a commend test")]
struct Cli {
    name: Option<String>,
    #[arg(short, long, default_value = "3")]
    config: Option<String>,
    #[arg(short, long, default_value_t = 1)]
    debug:u8,
    #[command(subcommand)]
    commend: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    CSV(Params)
}

#[derive(Parser)]
#[command(name = "csv", version, author, about="a commend test")]
struct Params {
    #[arg(short, long, value_parser = check_input)]
    input: String,
    #[arg(short, long)]
    output: String,
    #[arg(long)]
    header: Option<String>,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char
}

//判断路径是否存在
fn check_input(v: &str) -> Result<String, String> {
    Path::new(v).exists().then(|| v.to_string()).ok_or_else(|| "file not found".to_string())
}

fn main() {
    let cli: Cli = Cli::parse();
    cli.commend.map(|c| match c {
        Commands::CSV(p) => {
            println!("input: {}, output: {}, delimiter: {}", p.input, p.output, p.delimiter);
            process_csv(p.input, p.output, p.delimiter)
        }
    });
}

fn process_csv(input: String, output: String, delimiter: char) -> Result<()>{
    //读取csv文件获取reader
    let mut rdr = csv::Reader::from_path(&input)?;
    //获取头部信息
    let headers = rdr.headers()?.clone();
    //获取内容数据
    let records = rdr.records();
    let mut res = vec![];
    for result in records {
        let r = result?;
        //将StringRecord转换为serde_json::Value
        let map = headers.iter().zip(r.iter()).collect::<serde_json::Value>();
        //将record保存到数组中
        res.push(map);
    }
    //将数组转换为json
    let json = serde_json::to_string_pretty(&res)?;
    //将json写入文件
    std::fs::write(output, json)?;
    Ok(())
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename= "PascalCase") ]
struct Record {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit")]
    kits: String,
    #[serde(rename = "Number")]
    number: u8
}
