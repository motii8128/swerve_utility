use super::types::Position2D;


/// ３平方の定理よりベクトルの大きさを計算する
pub fn distance(a : Position2D)->f32
{
    (a.x * a.x + a.y * a.y).sqrt()
}

/// ２次元回転行列を計算してベクトルを回転する
/// * `a` - ２次元ベクトル
/// * `theta` - 角度(rad)
pub fn rotate_vector(a : Position2D, theta : f32)->Position2D
{
    Position2D::new(
        a.x * theta.cos() - a.y * theta.sin(), 
        a.x * theta.sin() + a.y * theta.cos())
}

/// ベクトルの向きをarc tanで算出する。結果はラジアン
/// * `a` - ２次元ベクトル
pub fn get_direction(a : Position2D)->f32
{
    (a.y / a.x).atan()
}