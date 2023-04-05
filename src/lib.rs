#[derive(Debug)]
pub struct HighScores {
    score: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            score: scores.to_vec(),
        }
        // unimplemented!("Construct a HighScores struct, given the scores: {scores:?}")
    }

    pub fn scores(&self) -> &[u32] {
        &self.score[0..self.score.len()]
        // unimplemented!("Return all the scores as a slice")
    }

    pub fn latest(&self) -> Option<u32> {
        match self.score.last() {
            Some(n) => Some(*n),
            None => None,
        }
        // unimplemented!("Return the latest (last) score")
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.score.iter().max() {
            Some(n) => Some(*n),
            None => None,
        }
        // unimplemented!("Return the highest score")
    }

    pub fn personal_top_three(self) -> Vec<u32> {
        let mut clone = self.score.clone();
        clone.sort_by(|a,b| b.cmp(a));
        if clone.len() < 3 {return clone } else { clone[0 .. 3].to_vec() }
        
        // unimplemented!("Return 3 highest scores")
    }
}
