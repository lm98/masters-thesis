// inside vm_status.rs

#[cfg(test)]
mod test {
    // this import directive lets us use the VMStatus struct and its methodss
    use super::*;
    use crate::path::Path;
    use crate::slot::Slot::{Nbr, Rep};

    #[test]
    fn test_empty() {
        let status = VMStatus::new();
        assert_eq!(status.path, Path::new());
        assert_eq!(status.index, 0);
        assert_eq!(status.neighbour, None)
    }

    #[test]
    fn test_fold_unfold() {
        let mut status_1 = VMStatus::new();
        let mut status_2 = VMStatus::new();
        assert_eq!(status_1.neighbour, None);
        assert!(!status_1.is_folding());
        status_1.fold_into(Some(7));
        status_2.fold_into(Some(8));
        assert_eq!(status_1.neighbour, Some(7));
        assert!(status_1.is_folding());
        assert_eq!(status_2.neighbour, Some(8));
        assert!(status_2.is_folding())
    }

    // other test functions...
}