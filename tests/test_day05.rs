use anyhow::Result;
use aoc2021_rust::day05::{part1, part2, Edge, EdgeType, Point};

#[test]
fn test_part1() -> Result<()> {
    assert_eq!(part1("inputs/day05_test.txt")?, 5);
    assert_eq!(part1("inputs/day05.txt")?, 6572);
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    assert_eq!(part2("inputs/day05_test.txt")?, 12);
    assert_eq!(part2("inputs/day05.txt")?, 21466);
    Ok(())
}

#[test]
fn test_edge_intersections() -> Result<()> {
    use EdgeType::*;
    let horizontal = Edge {
        start: Point { x: 3, y: 3 },
        end: Point { x: 6, y: 3 },
        typ: Horizontal,
    };
    let diag = Edge {
        start: Point { x: 3, y: 3 },
        end: Point { x: 5, y: 5 },
        typ: Diagonal,
    };
    let res = horizontal.intersection(&diag, true)?.len();
    assert_eq!(res, 1);
    let res = diag.intersection(&horizontal, true)?.len();
    assert_eq!(res, 1);

    let diag = Edge {
        start: Point { x: 3, y: 3 },
        end: Point { x: 5, y: 5 },
        typ: Diagonal,
    };
    let res = horizontal.intersection(&diag, true)?;
    assert_eq!(res.len(), 1);
    let res = diag.intersection(&horizontal, true)?.len();
    assert_eq!(res, 1);

    let diag = Edge {
        start: Point { x: 1, y: 1 },
        end: Point { x: 3, y: 3 },
        typ: Diagonal,
    };
    let res = horizontal.intersection(&diag, true)?.len();
    assert_eq!(res, 1);
    let res = diag.intersection(&horizontal, true)?.len();
    assert_eq!(res, 1);

    let res = diag.intersection(&horizontal, true)?.len();
    assert_eq!(res, 1);

    /*let vertical = Edge::Horizontal(Point { x: 3, y: 3 }, Point { x: 6, y: 3 });
    let diag = Edge::Diagonal(Point { x: 3, y: 3 }, Point { x: 5, y: 5 });
    let res = vertical.intersection(&diag, true).len();
    assert_eq!(res, 1);
    let res = diag.intersection(&vertical, true).len();
    assert_eq!(res, 1);

    let diag = Edge::Diagonal(Point { x: 3, y: 3 }, Point { x: 1, y: 1 });
    let res = vertical.intersection(&diag, true).len();
    assert_eq!(res, 1);
    let res = diag.intersection(&&vertical, true).len();
    assert_eq!(res, 1);

    let diag1 = Edge::Diagonal(Point { x: 0, y: 0 }, Point { x: 2, y: 2 });
    let diag2 = Edge::Diagonal(Point { x: 1, y: 3 }, Point { x: 3, y: 1 });
    let res = diag1.intersection(&diag2, true).len();
    assert_eq!(res, 1);

    let diag1 = Edge::Diagonal(Point { x: 2, y: 2 }, Point { x: 0, y: 0 });
    let diag2 = Edge::Diagonal(Point { x: 1, y: 3 }, Point { x: 3, y: 1 });
    let res = diag1.intersection(&diag2, true).len();
    assert_eq!(res, 1);*/
    Ok(())
}
