use std::collections::HashMap;
use rand::Rng;

pub struct UCBBandit {
    arms: HashMap<String, ArmStats>,
    total_pulls: u64,
    alpha: f64,
}

struct ArmStats {
    pulls: u64,
    reward_sum: f64,
    recent_rewards: Vec<f64>,
    decay_factor: f64,
}

impl UCBBandit {
    pub fn new(alpha: f64) -> Self {
        Self { arms: HashMap::new(), total_pulls: 0, alpha }
    }

    pub fn add_arm(&mut self, id: &str) {
        self.arms.insert(id.to_string(), ArmStats {
            pulls: 0, reward_sum: 0.0, recent_rewards: Vec::new(), decay_factor: 0.95,
        });
    }

    pub fn select_arm(&mut self) -> Option<String> {
        let mut best = None;
        let mut best_score = f64::NEG_INFINITY;
        for (id, stats) in &self.arms {
            let score = if stats.pulls == 0 {
                f64::INFINITY
            } else {
                let avg = stats.reward_sum / stats.pulls as f64;
                let exploration = self.alpha * ((2.0 * (self.total_pulls as f64).ln() / stats.pulls as f64).sqrt());
                avg + exploration
            };
            if score > best_score {
                best_score = score;
                best = Some(id.clone());
            }
        }
        best
    }

    pub fn update_reward(&mut self, arm_id: &str, reward: f64) {
        if let Some(stats) = self.arms.get_mut(arm_id) {
            stats.reward_sum = stats.reward_sum * stats.decay_factor + reward;
            stats.pulls += 1;
            stats.recent_rewards.push(reward);
            if stats.recent_rewards.len() > 100 { stats.recent_rewards.remove(0); }
        }
        self.total_pulls += 1;
    }

    pub fn get_scores(&self) -> Vec<(String, f64)> {
        self.arms.iter().map(|(id, stats)| {
            let score = if stats.pulls == 0 { 0.0 } else {
                stats.reward_sum / stats.pulls as f64 + self.alpha * ((2.0 * (self.total_pulls as f64).ln() / stats.pulls as f64).sqrt())
            };
            (id.clone(), score)
        }).collect()
    }
}
