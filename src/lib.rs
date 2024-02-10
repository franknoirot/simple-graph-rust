pub mod constants;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imported_constants() {
        assert_eq!(constants::DELETE_EDGE, "DELETE FROM edges WHERE source = ? AND target = ?");
    }
}
