fn ways_to_play(time: usize) -> Vec<(usize, usize)> {
    (0..=time).map(|n| (n, n * (time - n))).collect()
}

fn ways_to_beat_record(time: usize, record: usize) -> Vec<(usize, usize)> {
    ways_to_play(time).into_iter().filter(|&(_, distance)| distance > record).collect()
}

mod test {
    use super::*;

    #[test]
    fn figures_out_ways_to_play() {
        let ways = ways_to_play(7);
        assert_eq!(ways, vec![(0, 0), (1, 6), (2, 10), (3, 12), (4, 12), (5, 10), (6, 6), (7, 0)])
    }

    #[test]
    fn figures_out_ways_to_beat_record() {
        let ways = ways_to_beat_record(7, 9);
        assert_eq!(ways, vec![(2, 10), (3, 12), (4, 12), (5, 10)])
    }

    #[test]
    fn solves_part_one() {
        let solution = [(42, 284), (68, 1005), (69, 1122), (85, 1341)]
            .iter()
            .fold(1, |result, &(time, record)| result * ways_to_beat_record(time, record).len());

        assert_eq!(solution, 440000)
    }

    #[test]
    fn solves_part_two() {
        let solution = ways_to_beat_record( 42686985, 284100511221341).len();

        assert_eq!(solution, 26187338)
    }
}