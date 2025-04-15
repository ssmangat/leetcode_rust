pub struct RecentCounter{
    requests:Vec<i32>,
}

impl RecentCounter{
    pub fn new() -> Self {
        RecentCounter{
            requests: Vec::new(),
        }
    }

    pub fn ping(&mut self,t:i32)-> i32 {
        self.requests.push(t);
        while !self.requests.is_empty() && self.requests[0] < t- 3000 {
            self.requests.remove(0);
        }
        self.requests.len() as i32
    }
}
