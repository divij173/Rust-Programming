#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuestStatus {
    Complete,
    Ongoing
}

/// This is what a "quest" should do.
/// Note that all `Quests` implement Debug and Display.
/// Traits' Debug implementation does not matter, but
/// they should implement [`std::fmt::Display`] to show
/// the current progress of the quest.
pub trait Quest<Event>: std::fmt::Display + std::fmt::Debug {
    /// Whenever something happens, you call "register_event" to tell the quest what's happened.
    fn register_event(&mut self, event: &Event) -> QuestStatus;

    /// Reset the quest, so that players can restart.
    fn reset(&mut self)
    {
        
    }
}