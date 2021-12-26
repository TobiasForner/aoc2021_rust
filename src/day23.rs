use anyhow::Result;

pub fn part1(path: &str) -> Result<usize> {
    let hallway = vec!['.'; 11];
    let room1 = vec!['C', 'D'];
    let room2 = vec!['A', 'C'];
    let room3 = vec!['B', 'A'];
    let room4 = vec!['D', 'B'];
    let res = solve_stack(hallway, room1, room2, room3, room4, 2);
    Ok(res)
}

pub fn part2(path: &str) -> Result<usize> {
    let hallway = vec!['.'; 11];
    let room1 = vec!['C', 'D', 'D', 'D'];
    let room2 = vec!['A', 'C', 'B', 'C'];
    let room3 = vec!['B', 'B', 'A', 'A'];
    let room4 = vec!['D', 'A', 'C', 'B'];

    /*let room1 = vec!['B', 'D', 'D', 'A'];
    let room2 = vec!['C', 'C', 'B', 'D'];
    let room3 = vec!['B', 'B', 'A', 'C'];
    let room4 = vec!['D', 'A', 'C', 'A'];*/

    /*let mut hallway = vec!['.'; 11];
    hallway[0] = 'D';

    let room1 = vec!['A', 'A', 'A', 'A'];
    let room2 = vec!['B', 'B', 'B', 'B'];
    let room3 = vec!['C', 'C', 'C', 'C'];
    let room4 = vec!['.', 'D', 'D', 'D'];*/
    let res = solve_stack(hallway, room1, room2, room3, room4, 4);
    Ok(res)
}

fn solve_stack(
    hallway: Vec<char>,
    room1: Vec<char>,
    room2: Vec<char>,
    room3: Vec<char>,
    room4: Vec<char>,
    room_size: usize,
) -> usize {
    let mut stack: Vec<(Vec<char>, Vec<char>, Vec<char>, Vec<char>, Vec<char>, usize)> =
        Vec::with_capacity(100000);
    stack.push((
        hallway.clone(),
        room1.clone(),
        room2.clone(),
        room3.clone(),
        room4.clone(),
        0,
    ));
    let mut res = usize::MAX;
    'outer: while !stack.is_empty() {
        let (hallway, room1, room2, room3, room4, mut cost) = stack.pop().unwrap();

        //check if solved
        if solved(&hallway, &room1, &room2, &room3, &room4) {
            if res > cost {
                res = cost;
            }
            continue;
        }
        if cost > res {
            continue;
        }
        let hallway_positions: Vec<usize> = vec![0, 1, 3, 5, 7, 9, 10];
        //try to move from the hallway into correct room
        let mut change = true;
        while change {
            change = false;
            for (pos, c) in hallway.iter().enumerate() {
                let c = *c;
                if c == '.' {
                    continue;
                }
                let (front_pos, costs, mut dest_room) = match c {
                    'A' => (2, 1, room1.clone()),
                    'B' => (4, 10, room2.clone()),
                    'C' => (6, 100, room3.clone()),
                    'D' => (8, 1000, room4.clone()),
                    _ => panic!("Invalid char {}", c),
                };
                let room_enter = dest_room.iter().all(|x| *x == '.' || *x == c);

                if room_enter {
                    let mut success = false;
                    if pos < front_pos && (pos + 1..=front_pos).all(|x| hallway[x] == '.') {
                        cost += costs
                            * (front_pos - pos + dest_room.iter().filter(|x| **x == '.').count());
                        success = true;
                    } else if pos >= front_pos && (front_pos..=pos - 1).all(|x| hallway[x] == '.') {
                        cost += costs
                            * (pos - front_pos + dest_room.iter().filter(|x| **x == '.').count());
                        success = true;
                    }
                    if cost > res {
                        continue;
                    }
                    if success {
                        change = true;
                        //println!("moved into correct room!");
                        let mut hallway = hallway.clone();
                        let place = (0..room_size)
                            .filter(|x| dest_room[*x as usize] == '.')
                            .max()
                            .unwrap();
                        dest_room[place] = c;
                        hallway[pos] = '.';
                        /*let rooms = match c {
                            'A' => (dest_room, room2.clone(), room3.clone(), room4.clone()),
                            'B' => (room1.clone(), dest_room, room3.clone(), room4.clone()),
                            'C' => (room1.clone(), room2.clone(), dest_room, room4.clone()),
                            'D' => (room1.clone(), room2.clone(), room3.clone(), dest_room),
                            _ => panic!("Invalid char {}", c),
                        };*/

                        if solved(&hallway, &room1, &room2, &room3, &room4) {
                            if res > cost {
                                res = cost;
                                println!("update");
                            }
                            continue 'outer;
                        }
                    }
                }
            }
        }

        //move into hallway from start of room
        let room_with_pos = vec![
            (room1.clone(), 2, 'A'),
            (room2.clone(), 4, 'B'),
            (room3.clone(), 6, 'C'),
            (room4.clone(), 8, 'D'),
        ];
        for (room, room_pos, c) in room_with_pos {
            let room_enter = room.iter().all(|x| *x == '.' || *x == c);
            if room_enter {
                continue;
            }
            for (pos, c) in room.iter().enumerate() {
                let c = *c;
                if c != '.' {
                    //try to move right
                    for p in &hallway_positions {
                        let p = *p;

                        if p > room_pos {
                            if hallway[p] != '.' {
                                break;
                            } else {
                                //println!("moving into the hallway");
                                let mut hallway = hallway.clone();
                                hallway[p] = c;
                                let mut room = room.clone();
                                room[pos] = '.';
                                let rooms = match c {
                                    'A' => (room, room2.clone(), room3.clone(), room4.clone()),
                                    'B' => (room1.clone(), room, room3.clone(), room4.clone()),
                                    'C' => (room1.clone(), room2.clone(), room, room4.clone()),
                                    'D' => (room1.clone(), room2.clone(), room3.clone(), room),
                                    _ => panic!("Invalid char {}", c),
                                };
                                let costs: usize = match c {
                                    'A' => 1,
                                    'B' => 10,
                                    'C' => 100,
                                    'D' => 1000,
                                    _ => panic!("Invalid char {}", c),
                                };
                                let new_cost = cost + costs * (pos + 1 + p - room_pos);
                                if new_cost > res {
                                    continue;
                                }
                                if solved(&hallway, &rooms.0, &rooms.1, &rooms.2, &rooms.3) {
                                    if new_cost < res {
                                        res = new_cost;
                                        println!("update");
                                    }
                                    continue;
                                }

                                stack.push((hallway, rooms.0, rooms.1, rooms.2, rooms.3, new_cost));
                            }
                        }
                    }

                    // try to move left
                    for p in hallway_positions.clone().iter().rev() {
                        let p = *p;

                        if p < room_pos {
                            if hallway[p] != '.' {
                                break;
                            } else {
                                //println!("moving into the hallway");
                                let mut hallway = hallway.clone();
                                hallway[p] = c;
                                let mut room = room.clone();
                                room[pos] = '.';
                                let rooms = match c {
                                    'A' => (room, room2.clone(), room3.clone(), room4.clone()),
                                    'B' => (room1.clone(), room, room3.clone(), room4.clone()),
                                    'C' => (room1.clone(), room2.clone(), room, room4.clone()),
                                    'D' => (room1.clone(), room2.clone(), room3.clone(), room),
                                    _ => panic!("Invalid char {}", c),
                                };
                                let costs: usize = match c {
                                    'A' => 1,
                                    'B' => 10,
                                    'C' => 100,
                                    'D' => 1000,
                                    _ => panic!("Invalid char {}", c),
                                };
                                let new_cost = cost + costs * (pos + 1 + room_pos - p);
                                if new_cost > res {
                                    continue;
                                }
                                if solved(&hallway, &rooms.0, &rooms.1, &rooms.2, &rooms.3) {
                                    if new_cost < res {
                                        res = new_cost;
                                        println!("update");
                                        continue;
                                    }
                                }
                                stack.push((hallway, rooms.0, rooms.1, rooms.2, rooms.3, new_cost));
                            }
                        }
                    }

                    break;
                }
            }
        }
    }
    res
}

fn solved(
    hallway: &Vec<char>,
    room1: &Vec<char>,
    room2: &Vec<char>,
    room3: &Vec<char>,
    room4: &Vec<char>,
) -> bool {
    hallway.iter().all(|x| *x == '.')
        && room1.iter().all(|x| *x == 'A')
        && room2.iter().all(|x| *x == 'B')
        && room3.iter().all(|x| *x == 'C')
        && room4.iter().all(|x| *x == 'D')
}

pub fn solve(
    hallway: Vec<char>,
    room1: Vec<char>,
    room2: Vec<char>,
    room3: Vec<char>,
    room4: Vec<char>,
    cost: usize,
) -> usize {
    //check if solved
    if hallway.iter().all(|x| *x == '.')
        && room1.iter().all(|x| *x == 'A')
        && room2.iter().all(|x| *x == 'B')
        && room3.iter().all(|x| *x == 'C')
        && room4.iter().all(|x| *x == 'D')
    {
        return cost;
    }
    /*println!("-------------------");
    println!("{:?}", hallway);
    println!("{:?}", room1);
    println!("{:?}", room2);
    println!("{:?}", room3);
    println!("{:?}", room4);*/

    let mut new_cost = cost;
    let hallway_positions: Vec<usize> = vec![0, 1, 3, 5, 7, 9, 10];
    //try to move from the hallway into correct room
    for (pos, c) in hallway.iter().enumerate() {
        let c = *c;
        if c == '.' {
            continue;
        }
        let (front_pos, costs, mut dest_room) = match c {
            'A' => (2, 1, room1.clone()),
            'B' => (4, 10, room2.clone()),
            'C' => (6, 100, room3.clone()),
            'D' => (8, 1000, room4.clone()),
            _ => panic!("Invalid char {}", c),
        };
        let room_enter = dest_room.iter().all(|x| *x == '.' || *x == c);

        if room_enter {
            let mut success = false;
            if pos < front_pos && (pos + 1..=front_pos).all(|x| hallway[x] == '.') {
                new_cost +=
                    costs * (front_pos - pos + dest_room.iter().filter(|x| **x == '.').count());
                success = true;
            } else if pos >= front_pos && (front_pos..=pos - 1).all(|x| hallway[x] == '.') {
                new_cost +=
                    costs * (pos - front_pos + dest_room.iter().filter(|x| **x == '.').count());
                success = true;
            }
            if success {
                //println!("moved into correct room!");
                let mut hallway = hallway.clone();
                let place = (0..4)
                    .filter(|x| dest_room[*x as usize] == '.')
                    .max()
                    .unwrap();
                dest_room[place] = c;
                hallway[pos] = '.';
                let rooms = match c {
                    'A' => (dest_room, room2.clone(), room3.clone(), room4.clone()),
                    'B' => (room1.clone(), dest_room, room3.clone(), room4.clone()),
                    'C' => (room1.clone(), room2.clone(), dest_room, room4.clone()),
                    'D' => (room1.clone(), room2.clone(), room3.clone(), dest_room),
                    _ => panic!("Invalid char {}", c),
                };
                return solve(hallway, rooms.0, rooms.1, rooms.2, rooms.3, new_cost);
            }
        }
    }

    //move into hallway from start of room
    let room_with_pos = vec![
        (room1.clone(), 2, 'A'),
        (room2.clone(), 4, 'B'),
        (room3.clone(), 6, 'C'),
        (room4.clone(), 8, 'D'),
    ];
    let mut res = usize::MAX;
    for (room, room_pos, c) in room_with_pos {
        let room_enter = room.iter().all(|x| *x == '.' || *x == c);
        if room_enter {
            continue;
        }
        for (pos, c) in room.iter().enumerate() {
            let c = *c;
            if c != '.' {
                //try to move right
                for p in &hallway_positions {
                    let p = *p;

                    if p > room_pos {
                        if hallway[p] != '.' {
                            break;
                        } else {
                            //println!("moving into the hallway");
                            let mut hallway = hallway.clone();
                            hallway[p] = c;
                            let mut room = room.clone();
                            room[pos] = '.';
                            let rooms = match c {
                                'A' => (room, room2.clone(), room3.clone(), room4.clone()),
                                'B' => (room1.clone(), room, room3.clone(), room4.clone()),
                                'C' => (room1.clone(), room2.clone(), room, room4.clone()),
                                'D' => (room1.clone(), room2.clone(), room3.clone(), room),
                                _ => panic!("Invalid char {}", c),
                            };
                            let costs: usize = match c {
                                'A' => 1,
                                'B' => 10,
                                'C' => 100,
                                'D' => 1000,
                                _ => panic!("Invalid char {}", c),
                            };
                            let new_cost = cost + costs * (pos + 1 + p - room_pos);
                            let new_res =
                                solve(hallway, rooms.0, rooms.1, rooms.2, rooms.3, new_cost);
                            if new_res < res {
                                res = new_res;
                            }
                        }
                    }
                }

                // try to move left
                for p in hallway_positions.clone().iter().rev() {
                    let p = *p;

                    if p < room_pos {
                        if hallway[p] != '.' {
                            break;
                        } else {
                            //println!("moving into the hallway");
                            let mut hallway = hallway.clone();
                            hallway[p] = c;
                            let mut room = room.clone();
                            room[pos] = '.';
                            let rooms = match c {
                                'A' => (room, room2.clone(), room3.clone(), room4.clone()),
                                'B' => (room1.clone(), room, room3.clone(), room4.clone()),
                                'C' => (room1.clone(), room2.clone(), room, room4.clone()),
                                'D' => (room1.clone(), room2.clone(), room3.clone(), room),
                                _ => panic!("Invalid char {}", c),
                            };
                            let costs: usize = match c {
                                'A' => 1,
                                'B' => 10,
                                'C' => 100,
                                'D' => 1000,
                                _ => panic!("Invalid char {}", c),
                            };
                            let new_cost = cost + costs * (pos + 1 + room_pos - p);
                            let new_res =
                                solve(hallway, rooms.0, rooms.1, rooms.2, rooms.3, new_cost);
                            if new_res < res {
                                res = new_res;
                            }
                        }
                    }
                }

                break;
            }
        }
    }
    println!("Terminated");
    return res;
}
