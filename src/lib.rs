pub mod types;
mod utils;
use utils::{distance, rotate_vector, get_direction};
use types::{Position2D, SwerveUnit};
use std::f32::consts::PI;

/// ４輪独立ステアリングの制御を行う
pub struct SwerveDriver
{
    front_left : SwerveUnit,
    front_right : SwerveUnit,
    rear_left : SwerveUnit,
    rear_right : SwerveUnit,
}

impl SwerveDriver {
    /// 初期化
    /// * `dist_front_back` - 駆動輪同士の前後間の距離
    /// * `dist_left_right` - 駆動輪同士の左右間の距離
    pub fn new(dist_front_back : f32, dist_left_right : f32)->Self
    {
        let half_dist_fb = dist_front_back / 2.0;
        let half_dist_lr = dist_left_right / 2.0;

        Self { 
            front_left: SwerveUnit::new(-1.0 * half_dist_lr, half_dist_fb), 
            front_right: SwerveUnit::new(half_dist_lr, half_dist_fb), 
            rear_left: SwerveUnit::new(-1.0 * half_dist_lr, -1.0 * half_dist_fb), 
            rear_right: SwerveUnit::new(half_dist_lr, -1.0 * half_dist_fb)
        }
    }

    /// 計算を行う
    /// * `x_vec` - ｘ方向の速度(m/s)右が正
    /// * `y_vec` - y方向の速度(m/s)前が正
    /// * `omega` - 回転速度(rad/s)時計回りが正
    pub fn compute(&mut self, x_vec : f32, y_vec : f32, omega : f32)
    {
        // 旋回がない場合シンプル
        if omega == 0.0
        {
            // arc tan(y / x)で角度をラジアンで取得
            let direction = y_vec.atan2(x_vec);

            // 駆動スピードは３平方の定理で求める
            let speed = (x_vec * x_vec + y_vec * y_vec).sqrt();

            self.front_left.steer_theta = direction;
            self.front_right.steer_theta = direction;
            self.rear_left.steer_theta = direction;
            self.rear_right.steer_theta = direction;
            self.front_left.velocity = speed;
            self.front_right.velocity = speed;
            self.rear_left.velocity = speed;
            self.rear_right.velocity = speed;

            return;
        }

        // 並進速度。ｘとｙの合成速度
        let vel = (x_vec*x_vec + y_vec * y_vec).sqrt();

        // 旋回半径。回転の中心までの距離
        let rotation_radius = vel / omega.abs();

        // 速度ベクトルの方向
        let direction = (y_vec / x_vec).atan();

        // 旋回方向の判定
        let omega_dir = omega / omega.abs();

        // 回転中心
        let center_rotation = Position2D::new(
            rotation_radius * (direction - PI / 2.0).cos() * omega_dir, 
            rotation_radius * (direction - PI / 2.0).sin() * omega_dir,
        );

        //　回転中心から各ユニットへのベクトル
        let fl_to_cr_vector = self.front_left.position - center_rotation;
        let fr_to_cr_vector = self.front_right.position - center_rotation;
        let rl_to_cr_vector = self.rear_left.position - center_rotation;
        let rr_to_cr_vector = self.rear_right.position - center_rotation;

        // それぞれのベクトルを円の接線方向へ回転する
        // 半径方向のベクトルと接線方向のベクトルは垂直に交わる
        let rotate_rad = PI / -2.0 * omega_dir;
        let fl_steer_vector = rotate_vector(fl_to_cr_vector, rotate_rad);
        let fr_steer_vector = rotate_vector(fr_to_cr_vector, rotate_rad);
        let rl_steer_vector = rotate_vector(rl_to_cr_vector, rotate_rad);
        let rr_steer_vector = rotate_vector(rr_to_cr_vector, rotate_rad);

        // ベクトルの大きさを線速度とする
        self.front_left.velocity = distance(fl_steer_vector);
        self.front_right.velocity = distance(fr_steer_vector);
        self.rear_left.velocity = distance(rl_steer_vector);
        self.rear_right.velocity = distance(rr_steer_vector);

        // ベクトルの向きがステアリング角度になる
        self.front_left.steer_theta = get_direction(fl_steer_vector);
        self.front_right.steer_theta = get_direction(fr_steer_vector);
        self.rear_left.steer_theta = get_direction(rl_steer_vector);
        self.rear_right.steer_theta = get_direction(rr_steer_vector);
    }

    /// 計算結果を出力する（左前）
    /// * `(駆動速度、ステア角)`
    pub fn get_front_left_target(&mut self)->(f32, f32)
    {
        (self.front_left.velocity, self.front_left.steer_theta)
    }

    /// 計算結果を出力する（右前）
    /// * `(駆動速度、ステア角)`
    pub fn get_front_right_target(&mut self)->(f32, f32)
    {
        (self.front_right.velocity, self.front_right.steer_theta)
    }

    /// 計算結果を出力する（左後）
    /// * `(駆動速度、ステア角)`
    pub fn get_rear_left_target(&mut self)->(f32, f32)
    {
        (self.rear_left.velocity, self.rear_left.steer_theta)
    }

    /// 計算結果を出力する（右後）
    /// * `(駆動速度、ステア角)`
    pub fn get_rear_right_target(&mut self)->(f32, f32)
    {
        (self.rear_right.velocity, self.rear_right.steer_theta)
    }
}

