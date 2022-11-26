pub fn min_meeting_rooms(input: &Vec<(usize, usize)>) -> usize {
    let mut intervals: Vec<(usize, i32)> = vec![];
    let mut occupied = 0;
    let mut result = 0;

    input.iter().for_each(|(begin, end)| {
        assert!(begin <= end);
        intervals.push((*begin, 1));
        intervals.push((*end, -1));
    });

    intervals.sort_by(|a, b| a.0.cmp(&b.0));

    intervals.iter().for_each(|(_, value)| {
        occupied += value;
        result = result.max(occupied);
    });

    result as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(min_meeting_rooms(&vec![(0, 1), (1, 2), (2, 3)]), 1);
        assert_eq!(min_meeting_rooms(&vec![(0, 1), (1, 2), (1, 3)]), 2);

        assert_eq!(min_meeting_rooms(&vec![(0, 8), (5, 10), (9, 12)]), 2);
        assert_eq!(min_meeting_rooms(&vec![(0, 8), (5, 10), (7, 12)]), 3);

        assert_eq!(min_meeting_rooms(&vec![(0, 100), (5, 15), (20, 30)]), 2);
        assert_eq!(min_meeting_rooms(&vec![(0, 100), (5, 15), (10, 30)]), 3);

        assert_eq!(min_meeting_rooms(&vec![(100, 200), (50, 101), (10, 51), (50, 51)]), 3);
        assert_eq!(min_meeting_rooms(&vec![(0, 10), (0, 10), (0, 10), (0, 10)]), 4);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(min_meeting_rooms(&vec![(0, 0)]), 1);
        assert_eq!(min_meeting_rooms(&vec![(0, 0), (1, 1)]), 1);

        assert_eq!(min_meeting_rooms(&vec![(usize::MIN, usize::MAX)]), 1);
        assert_eq!(min_meeting_rooms(&vec![(0, 65535), (65535, usize::MAX)]), 1);
    }
}
