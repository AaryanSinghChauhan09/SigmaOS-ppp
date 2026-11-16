// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-ppp: ppp component model.
//!
//! Real implementation: ppp_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PppItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PppError {
    EmptyPppItemName,
    DuplicatePppItem(String),
    EmptyEventSubject,
}

impl fmt::Display for PppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPppItemName => write!(f, "ppp_item name must not be empty"),
            Self::DuplicatePppItem(id) => write!(f, "duplicate ppp_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for PppError {}

pub struct Ppp {
    pub name: String,
    ppp_items: BTreeMap<String, PppItem>,
    events: BTreeMap<String, Event>,
}

impl Ppp {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ppp_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_ppp_item(&mut self, item: PppItem) -> Result<(), PppError> {
        if item.name.trim().is_empty() {
            return Err(PppError::EmptyPppItemName);
        }
        let id = item.id.clone();
        if self.ppp_items.contains_key(&id) {
            return Err(PppError::DuplicatePppItem(id.clone()));
        }
        self.ppp_items.insert(id, item);
        Ok(())
    }

    pub fn ppp_item_count(&self) -> usize {
        self.ppp_items.len()
    }

    pub fn get_ppp_item(&self, id: &str) -> Option<&PppItem> {
        self.ppp_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PppError> {
        if event.subject.trim().is_empty() {
            return Err(PppError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_ppp_item(id: &str, name: &str) -> PppItem {
        PppItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Ppp::new("Ppp");
        assert_eq!(m.name, "Ppp");
        assert_eq!(m.ppp_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_ppp_item() {
        let mut m = Ppp::new("Ppp");
        m.add_ppp_item(sample_ppp_item("a1", "Alpha")).unwrap();
        assert_eq!(m.ppp_item_count(), 1);
        assert!(m.get_ppp_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Ppp::new("Ppp");
        assert!(m.add_ppp_item(sample_ppp_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Ppp::new("Ppp");
        m.add_ppp_item(sample_ppp_item("x", "One")).unwrap();
        assert!(m.add_ppp_item(sample_ppp_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Ppp::new("Ppp");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
