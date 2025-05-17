/// 平面における座標を管理する
#[derive(Clone, Copy)]
pub struct Position2D
{
    pub x : f32,
    pub y : f32,
}
impl Position2D {
    /// 初期化
    /// * `x_` - x方向の位置
    /// * `y_` - y方向の位置
    pub fn new(x_ : f32, y_ : f32)->Self
    {
        Self { x: x_, y: y_ }
    }
}
impl std::ops::Add for Position2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x : self.x + rhs.x,
            y : self.y + rhs.y
        }
    }
}
impl std::ops::Sub for Position2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x : self.x - rhs.x,
            y : self.y - rhs.y
        }
    }
    
}


/// 独立ステアリングのユニット
/// * `position` - 車体中心から見た位置
/// * `steer_theta` - 操舵角(rad)
/// * `velocity` - 駆動速度(rpm)
#[derive(Clone, Copy)]
pub struct SwerveUnit
{
    pub position : Position2D,
    pub steer_theta : f32,
    pub velocity : f32,
}

impl SwerveUnit {
    /// 初期化
    /// * `x_` - x方向の位置
    /// * `y_` - y方向の位置
    pub fn new(x_:f32, y_:f32)->Self
    {
        Self {
            position : Position2D::new(x_, y_), 
            steer_theta: 0.0 , 
            velocity : 0.0
        }
    }
}