use std::io::BufRead;
use std::str::FromStr;

// このプログラムは標準入力から整数を読み込み、その合計を計算します。
// 標準入力を停止するには、以下のキーを押してください：
// - macOS/Linux: Ctrl+D (EOF信号)
// - Windows: Ctrl+Z を押してから Enter
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Chapter 15.5.9 try_fold, try_rfold");

    let stdin = std::io::stdin();
    let sum = stdin.lock().lines().try_fold(
        0,
        |sum, line| -> Result<u64, Box<dyn std::error::Error>> {
            Ok(sum + u64::from_str(&line?.trim())?)
        },
    )?;
    println!("sum: {}", sum);
    Ok(())
}
