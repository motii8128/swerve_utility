# swerve_utility

|test|status|
|:--:|:--:|
|Rust|[![Rust](https://github.com/motii8128/swerve_utility/actions/workflows/rust.yml/badge.svg)](https://github.com/motii8128/swerve_utility/actions/workflows/rust.yml)|

# 使用方法
Cargo.tomlに以下のように追記する
```toml
[dependencies]
swerve_utility = {git = "https://github.com/motii8128/swerve_utility.git"}
```

# Example
```rs
// 基本的にはSwerveDriverのみを使う
use swerve_utility::SwerveDriver;

fn main()
{
    // ドライバーを初期化する
    // ひとつめの引数はユニット同士の前後間の距離
    // ふたつめの引数はユニット同士の左右間の距離
    let mut swerve_driver = SwerveDriver::new(1.0, 1.0);

    // 右方向への直進を入力
    // x = 1.0[m/s], y = 0.0[m/s], ω = 0.0[rad/s]
    swerve_driver.compute(1.0, 0.0, 0.0);

    // 左前のユニットの操作量を取得する
    // （駆動速度, ステアリング角度）というタプルで取得する
    let (front_left_vel, front_left_steer_rad) = swerve_driver.get_front_left_target();

    println!("FrontLeft -> vel : {}, steer : {}", front_left_vel, front_left_steer_rad);

    // 右前のユニットの操作量を取得する
    let front_right = swerve_driver.get_front_right_target();

    //　タプルはこんな感じでも使える
    println!("FrontRight -> vel : {}, steer : {}", front_right.0, front_right.1);
}
```
