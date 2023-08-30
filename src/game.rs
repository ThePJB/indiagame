use crate::kmath::*;
use crate::kapp::*;

pub struct Game {
    frame_no: u64,
    seed: u32,
    t: f32,

    player_pos: Vec2,
    player_vel: Vec2,
    player_facing: Vec2,
    player_health: f32,
    player_t_swing: f32,

    ball_pos: Vec2,
    ball_vel: Vec2,

    enemy_pos: Vec<Vec2>,
    enemy_vel: Vec<Vec2>,
    enemy_health: Vec<f32>,
}

impl Game {
    pub fn new(seed: u32) -> Self {
        Game {
            frame_no: 0,
            seed,
            t: 0.0,

            player_pos: v2(0.0, 0.0),
            player_vel: v2(0.0, 0.0),
            player_facing: v2(0.0, 0.0),
            player_health: 100.0,
            player_t_swing: -100.0,
            
            ball_pos: v2(0.15, 0.2),
            ball_vel: v2(0.1, 0.21),

            enemy_pos: vec![],
            enemy_vel: vec![],
            enemy_health: vec![],
        }
    }

    pub fn frame(&mut self, inputs: &FrameInputs, outputs: &mut FrameOutputs) {
        let dt = inputs.dt;
        let player_speed = 0.3;
        let player_r = inputs.screen_rect.h * 0.025;
        let ball_r = inputs.screen_rect.h * 0.035;
        let game_rect = inputs.screen_rect.dilate_pc(-0.05);
        let swing_cooldown = 0.5;
        let swing_radius = 0.25;
        let swing_degrees = 90.0;
        let fg = v4(1., 0.25, 0.0, 1.0);
        let bg = v4(1., 0.9, 0.8, 1.0);


        if self.frame_no == 0 {
            self.player_pos = inputs.screen_rect.centroid();
            self.ball_pos = self.player_pos + self.ball_pos;
        }
        
        let mut move_vec = v2(0.0, 0.0);
        if inputs.key_held(VirtualKeyCode::W) {
           move_vec.y -= 1.0;
        }
        if inputs.key_held(VirtualKeyCode::S) {
           move_vec.y += 1.0;
        }
        if inputs.key_held(VirtualKeyCode::A) {
           move_vec.x -= 1.0;
        }
        if inputs.key_held(VirtualKeyCode::D) {
           move_vec.x += 1.0;
        }
        move_vec = move_vec.normalize();
        self.player_vel = player_speed * move_vec;
        self.player_facing = (inputs.mouse_pos - self.player_pos).normalize();
        self.player_pos = self.player_pos + move_vec * player_speed * dt;
        let player_to_ball = self.ball_pos - self.player_pos;
        let d = player_to_ball.magnitude();
        let dp = player_to_ball.normalize().dot(self.player_facing);
        let degoff = dp.acos() / PI * 180.0;

        if inputs.lmb == KeyStatus::JustPressed {
           if self.t - self.player_t_swing > swing_cooldown {
               self.player_t_swing = self.t;
               println!("swing");
               if d < swing_radius && degoff < swing_degrees/2.0 {
                   println!("hit ball");
                   self.ball_vel = self.player_facing * 0.5;
               }
           }
        }
        if self.t - self.player_t_swing < 0.05 {
            outputs.canvas.put_circle_arc(self.player_pos, swing_radius, self.player_facing, swing_degrees, 1.05, fg);
        }

        self.player_pos.x = self.player_pos.x.max(game_rect.x + player_r).min(game_rect.right() - player_r);
        self.player_pos.y = self.player_pos.y.max(game_rect.y + player_r).min(game_rect.right() - player_r);

        self.ball_pos = self.ball_pos + self.ball_vel * dt;
        if self.ball_pos.x < game_rect.x + ball_r {
            self.ball_pos.x = game_rect.x + ball_r;
            self.ball_vel.x *= -1.0;
        }
        if self.ball_pos.x > game_rect.right() - ball_r {
            self.ball_pos.x = game_rect.right() - ball_r;
            self.ball_vel.x *= -1.0;
        }
        if self.ball_pos.y < game_rect.y + ball_r {
            self.ball_pos.y = game_rect.y + ball_r;
            self.ball_vel.y *= -1.0;
        }
        if self.ball_pos.y > game_rect.bot() - ball_r {
            self.ball_pos.y = game_rect.bot() - ball_r;
            self.ball_vel.y *= -1.0;
        }


        outputs.canvas.put_rect(inputs.screen_rect, 1.01, fg);
        outputs.canvas.put_rect(game_rect, 1.02, bg);
        
        outputs.canvas.put_circle(self.player_pos, player_r, 1.03, fg);
        outputs.canvas.put_circle(self.player_pos + player_r * v2(self.player_facing.y, -self.player_facing.x), player_r * 0.5, 1.03, fg);
        outputs.canvas.put_circle(self.player_pos + player_r * v2(-self.player_facing.y, self.player_facing.x), player_r * 0.5, 1.03, fg);

        outputs.canvas.put_circle(self.ball_pos, ball_r, 1.03, fg);
        outputs.canvas.put_circle(self.ball_pos, ball_r * 0.8, 1.04, bg);

        self.frame_no += 1;
        self.t += dt;
    }
}
