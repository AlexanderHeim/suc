use std::time::{Duration, Instant};

use rand::{Rng, distributions::Alphanumeric, prelude::ThreadRng};


pub struct SessionPool {
    sessions: Vec<Session>,
    duration: Duration,
    rng: ThreadRng,
}

impl SessionPool {
    pub fn new(duration: Duration) -> Self {
        SessionPool {
            sessions: Vec::new(),
            duration,
            rng: ThreadRng::default(),
        }
    }

    pub fn generate(&mut self) -> String {
        let s: String = (&mut self.rng)
            .sample_iter(&Alphanumeric)
            .take(48)
            .map(char::from)
            .collect();
        self.sessions.push(Session::new(&s));
        s
    }

    pub fn check(&mut self, token: &str) -> bool {
        let index = match self.find(token) {
            Some(r) => r,
            None => return false
        };
        if self.sessions[index].since() > self.duration {
            self.sessions.remove(index);
            return false;
        }
        true
    }

    pub fn remove(&mut self, token: &str) {
        let r = match self.find(token) {
            Some(r) => r,
            None => return
        };
        self.sessions.remove(r);
    }

    fn find(&self, token: &str) -> Option<usize> {
        match self.sessions.iter().enumerate().find(|x| x.1.token.as_str() == token) {
            Some(r) => Some(r.0),
            None => None,
        }
    }
}

pub struct Session {
    token: String,
    creation: Instant,
}

impl Session {
    pub fn new(token: &str) -> Self {
        let t = String::from(token);
        Session {
            token: t,
            creation: Instant::now(),
        }
    }

    pub fn since(&self) -> Duration {
        Instant::now().duration_since(self.creation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_pool() {
        let mut session_pool = SessionPool::new(Duration::from_secs(1));
        let session_token = session_pool.generate();
        assert_eq!(session_token.len(), 48);
        assert!(session_pool.check(&session_token));
        std::thread::sleep(Duration::from_secs(2));
        assert!(!session_pool.check(&session_token));
        assert!(session_pool.sessions.is_empty());
    }
}