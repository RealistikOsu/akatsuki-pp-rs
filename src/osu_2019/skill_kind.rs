use super::DifficultyObject;

const SINGLE_SPACING_TRESHOLD: f32 = 125.0;
const SPEED_ANGLE_BONUS_BEGIN: f32 = 5.0 * std::f32::consts::FRAC_PI_6;
const PI_OVER_4: f32 = std::f32::consts::FRAC_PI_4;
const PI_OVER_2: f32 = std::f32::consts::FRAC_PI_2;

const MIN_SPEED_BONUS: f32 = 75.0;
const MAX_SPEED_BONUS: f32 = 45.0;
const SPEED_BALANCING_FACTOR: f32 = 40.0;

const AIM_ANGLE_BONUS_BEGIN: f32 = std::f32::consts::FRAC_PI_3;
const TIMING_THRESHOLD: f32 = 107.0;

/// Nerf factor applied to stream-like (flow aim) patterns in relax mode.
/// Uses a cubic curve so the penalty remains strong even at moderate distances
/// (typical streams land at ~80% of SINGLE_SPACING_THRESHOLD after scaling):
///   dist=0%   → 0.25× strain (75% reduction)
///   dist=80%  → 0.63× strain (37% reduction)
///   dist=90%  → 0.80× strain (20% reduction)
///   dist=100% → 1.00× strain (jumps unaffected)
const RELAX_STREAM_NERF_FLOOR: f32 = 0.25;

#[derive(Copy, Clone)]
pub(crate) enum SkillKind {
    Aim,
    Speed,
}

impl SkillKind {
    pub(crate) fn strain_value_of(self, current: &DifficultyObject<'_>) -> f32 {
        match self {
            Self::Aim => {
                if current.base.is_spinner() {
                    return 0.0;
                }

                let mut result = 0.0;

                if let Some((prev_jump_dist, prev_strain_time)) = current.prev {
                    if let Some(angle) = current.angle.filter(|a| *a > AIM_ANGLE_BONUS_BEGIN) {
                        let scale = 90.0;

                        let angle_bonus = (((angle - AIM_ANGLE_BONUS_BEGIN).sin()).powi(2)
                            * (prev_jump_dist - scale).max(0.0)
                            * (current.jump_dist - scale).max(0.0))
                        .sqrt();

                        result = 1.5 * apply_diminishing_exp(angle_bonus.max(0.0))
                            / (TIMING_THRESHOLD).max(prev_strain_time)
                    }
                }

                let jump_dist_exp = apply_diminishing_exp(current.jump_dist);
                let travel_dist_exp = apply_diminishing_exp(current.travel_dist);

                let dist_exp =
                    jump_dist_exp + travel_dist_exp + (travel_dist_exp * jump_dist_exp).sqrt();

                (result + dist_exp / (current.strain_time).max(TIMING_THRESHOLD))
                    .max(dist_exp / current.strain_time)
            }
            Self::Speed => {
                if current.base.is_spinner() {
                    return 0.0;
                }

                let dist = SINGLE_SPACING_TRESHOLD.min(current.travel_dist + current.jump_dist);
                let delta_time = MAX_SPEED_BONUS.max(current.delta);

                let mut speed_bonus = 1.0;

                if delta_time < MIN_SPEED_BONUS {
                    let exp_base = (MIN_SPEED_BONUS - delta_time) / SPEED_BALANCING_FACTOR;
                    speed_bonus += exp_base * exp_base;
                }

                let mut angle_bonus = 1.0;

                if let Some(angle) = current.angle.filter(|a| *a < SPEED_ANGLE_BONUS_BEGIN) {
                    let exp_base = (1.5 * (SPEED_ANGLE_BONUS_BEGIN - angle)).sin();
                    angle_bonus = 1.0 + exp_base * exp_base / 3.57;

                    if angle < PI_OVER_2 {
                        angle_bonus = 1.28;

                        if dist < 90.0 && angle < PI_OVER_4 {
                            angle_bonus += (1.0 - angle_bonus) * ((90.0 - dist) / 10.0).min(1.0);
                        } else if dist < 90.0 {
                            angle_bonus += (1.0 - angle_bonus)
                                * ((90.0 - dist) / 10.0).min(1.0)
                                * ((PI_OVER_2 - angle) / PI_OVER_4).sin();
                        }
                    }
                }

                let strain = (1.0 + (speed_bonus - 1.0) * 0.75)
                    * angle_bonus
                    * (0.95 + speed_bonus * (dist / SINGLE_SPACING_TRESHOLD).powf(3.5))
                    / current.strain_time;

                // Apply a nerf to stream-like (flow aim) patterns.
                // Cubic curve keeps the penalty strong even at moderate distances
                // (~80% of threshold for typical CS4 streams), while full jumps
                // (dist = SINGLE_SPACING_THRESHOLD) remain completely unaffected.
                let stream_nerf = RELAX_STREAM_NERF_FLOOR
                    + (1.0 - RELAX_STREAM_NERF_FLOOR)
                        * (dist / SINGLE_SPACING_TRESHOLD).powi(3);
                strain * stream_nerf
            }
        }
    }
}

#[inline]
fn apply_diminishing_exp(val: f32) -> f32 {
    val.powf(0.99)
}
