use std::collections::{HashMap, HashSet};

pub struct SecurityToolTracker {
    tools: HashMap<String, ToolData>,
}

impl SecurityToolTracker {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn add_tool(&mut self, name: &str, tool_type: &str) {
        let tool_data = ToolData::new(tool_type);
        self.tools.insert(name.to_string(), tool_data);
    }

    pub fn remove_tool(&mut self, name: &str) {
        self.tools.remove(name);
    }

    pub fn update_tool(&mut self, name: &str, new_type: &str) {
        if let Some(tool_data) = self.tools.get_mut(name) {
            tool_data.set_tool_type(new_type);
        }
    }

    pub fn get_tool(&self, name: &str) -> Option<&ToolData> {
        self.tools.get(name)
    }

    pub fn get_all_tools(&self) -> Vec<&ToolData> {
        self.tools.values().collect()
    }
}

struct ToolData {
    tool_type: String,
    vuln_count: usize,
    threat_level: ThreatLevel,
}

impl ToolData {
    fn new(tool_type: &str) -> Self {
        Self {
            tool_type: tool_type.to_string(),
            vuln_count: 0,
            threat_level: ThreatLevel::Low,
        }
    }

    fn set_tool_type(&mut self, new_type: &str) {
        self.tool_type = new_type.to_string();
    }

    fn increment_vuln_count(&mut self) {
        self.vuln_count += 1;
        if self.vuln_count > 5 {
            self.threat_level = ThreatLevel::High;
        } else if self.vuln_count > 2 {
            self.threat_level = ThreatLevel::Medium;
        }
    }

    fn decrement_vuln_count(&mut self) {
        self.vuln_count -= 1;
        if self.vuln_count <= 2 {
            self.threat_level = ThreatLevel::Low;
        } else if self.vuln_count <= 5 {
            self.threat_level = ThreatLevel::Medium;
        }
    }
}

enum ThreatLevel {
    Low,
    Medium,
    High,
}

fn main() {
    let mut tracker = SecurityToolTracker::new();

    tracker.add_tool("Tool1", "Vulnerability Scanner");
    tracker.add_tool("Tool2", "Penetration Tester");

    println!("All tools: {:?}", tracker.get_all_tools());

    tracker.update_tool("Tool1", "Penetration Tester");
    println!("Tool1 updated: {:?}", tracker.get_tool("Tool1"));

    let mut tool1 = tracker.get_tool("Tool1").unwrap().clone();
    tool1.increment_vuln_count();
    tool1.increment_vuln_count();
    tracker.update_tool("Tool1", tool1.tool_type.as_str());

    println!("Tool1 threat level: {:?}", tracker.get_tool("Tool1").unwrap().threat_level);
}