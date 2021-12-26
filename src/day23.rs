use anyhow::Result;

pub fn part1(_path: &str) -> Result<usize> {
    let hallway = vec!['.'; 11];
    let room1 = vec!['C', 'D'];
    let room2 = vec!['A', 'C'];
    let room3 = vec!['B', 'A'];
    let room4 = vec!['D', 'B'];
    let res = solve_stack(hallway, room1, room2, room3, room4, 2);
    Ok(res)
    //Ok(0)
}

pub fn part2(_path: &str) -> Result<usize> {
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
        Vec::with_capacity(1000);
    check_state(&hallway, &room1, &room2, &room3, &room4, room_size);
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
        let (mut hallway, mut room1, mut room2, mut room3, mut room4, mut cost) =
            stack.pop().unwrap();
        check_state(&hallway, &room1, &room2, &room3, &room4, room_size);
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
            for (pos, c) in hallway.clone().iter().enumerate() {
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
                        let place = (0..room_size)
                            .filter(|x| dest_room[*x as usize] == '.')
                            .max()
                            .unwrap();
                        check_state(&hallway, &room1, &room2, &room3, &room4, room_size);
                        //println!("placing {} at {}; resetting hallway pos {}", c, place, pos);
                        dest_room[place] = c;
                        hallway[pos] = '.';
                        let rooms = match c {
                            'A' => (dest_room, room2.clone(), room3.clone(), room4.clone()),
                            'B' => (room1.clone(), dest_room, room3.clone(), room4.clone()),
                            'C' => (room1.clone(), room2.clone(), dest_room, room4.clone()),
                            'D' => (room1.clone(), room2.clone(), room3.clone(), dest_room),
                            _ => panic!("Invalid char {}", c),
                        };

                        if solved(&hallway, &rooms.0, &rooms.1, &rooms.2, &rooms.3) {
                            if res > cost {
                                res = cost;
                                //println!("update");
                            }
                            continue 'outer;
                        }
                        room1 = rooms.0;
                        room2 = rooms.1;
                        room3 = rooms.2;
                        room4 = rooms.3;
                        check_state(&hallway, &room1, &room2, &room3, &room4, room_size);
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
        for (room, room_pos, c_room) in room_with_pos {
            let room_enter = room.iter().all(|x| *x == '.' || *x == c_room);
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
                                /*println!(
                                    "moving {} into the hallway (1), room pos {}",
                                    c, room_pos
                                );*/
                                let mut hallway = hallway.clone();
                                hallway[p] = c;
                                let mut room = room.clone();
                                //println!("room that will be changed: {:?}", room);
                                //println!("changing pos {} to .", pos);
                                room[pos] = '.';
                                let rooms = match c_room {
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
                                        //println!("update");
                                    }
                                    continue;
                                }
                                check_state(
                                    &hallway, &rooms.0, &rooms.1, &rooms.2, &rooms.3, room_size,
                                );

                                stack.push((hallway, rooms.0, rooms.1, rooms.2, rooms.3, new_cost));
                            }
                        }
                    }
                    break;
                }
            }
            for (pos, c) in room.iter().enumerate() {
                let c = *c;
                if c != '.' {
                    // try to move left
                    for p in hallway_positions.clone().iter().rev() {
                        let p = *p;

                        if p < room_pos {
                            if hallway[p] != '.' {
                                break;
                            } else {
                                //println!("moving {} into the hallway (2)", c);
                                let mut hallway = hallway.clone();
                                hallway[p] = c;
                                let mut room = room.clone();
                                room[pos] = '.';
                                let rooms = match c_room {
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
                                        //println!("update");
                                        continue;
                                    }
                                }
                                check_state(
                                    &hallway, &rooms.0, &rooms.1, &rooms.2, &rooms.3, room_size,
                                );
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
    let res = hallway.iter().all(|x| *x == '.')
        && room1.iter().all(|x| *x == 'A')
        && room2.iter().all(|x| *x == 'B')
        && room3.iter().all(|x| *x == 'C')
        && room4.iter().all(|x| *x == 'D');
    /*println!("---------------------");
    println!("{:?}", hallway);
    println!("{:?}", room1);
    println!("{:?}", room2);
    println!("{:?}", room3);
    println!("{:?}", room4);*/
    return res;
}

fn check_state(
    hallway: &Vec<char>,
    room1: &Vec<char>,
    room2: &Vec<char>,
    room3: &Vec<char>,
    room4: &Vec<char>,
    room_size: usize,
) {
    let entries = hallway.iter().filter(|x| **x != '.').count()
        + room1.iter().filter(|x| **x != '.').count()
        + room2.iter().filter(|x| **x != '.').count()
        + room3.iter().filter(|x| **x != '.').count()
        + room4.iter().filter(|x| **x != '.').count();

    let a_count = hallway.iter().filter(|x| **x == 'A').count()
        + room1.iter().filter(|x| **x == 'A').count()
        + room2.iter().filter(|x| **x == 'A').count()
        + room3.iter().filter(|x| **x == 'A').count()
        + room4.iter().filter(|x| **x == 'A').count();
    let b_count = hallway.iter().filter(|x| **x == 'B').count()
        + room1.iter().filter(|x| **x == 'B').count()
        + room2.iter().filter(|x| **x == 'B').count()
        + room3.iter().filter(|x| **x == 'B').count()
        + room4.iter().filter(|x| **x == 'B').count();
    let c_count = hallway.iter().filter(|x| **x == 'C').count()
        + room1.iter().filter(|x| **x == 'C').count()
        + room2.iter().filter(|x| **x == 'C').count()
        + room3.iter().filter(|x| **x == 'C').count()
        + room4.iter().filter(|x| **x == 'C').count();
    let d_count = hallway.iter().filter(|x| **x == 'D').count()
        + room1.iter().filter(|x| **x == 'D').count()
        + room2.iter().filter(|x| **x == 'D').count()
        + room3.iter().filter(|x| **x == 'D').count()
        + room4.iter().filter(|x| **x == 'D').count();
    if entries != 4 * room_size
        || a_count != room_size
        || b_count != room_size
        || c_count != room_size
        || d_count != room_size
    {
        println!("---------------------");
        println!("{:?}", hallway);
        println!("{:?}", room1);
        println!("{:?}", room2);
        println!("{:?}", room3);
        println!("{:?}", room4);
        panic!("Invalid item count!");
    }
}
