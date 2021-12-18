use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy)]
struct Area {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

fn read_file() -> Area {
    let mut file = File::open("./input/input17.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let relevant: &Vec<&str> = &contents[13..].split(", ").collect(); // chop off "target area: " which is 13 chars
    let x_part = relevant[0];
    let y_part = relevant[1];
    let x_ends: Vec<&str> = x_part[2..].split("..").collect();
    let x_min = x_ends[0].parse().unwrap();
    let x_max = x_ends[1].parse().unwrap();
    let y_ends: Vec<&str> = y_part[2..].split("..").collect();
    let y_min = y_ends[0].parse().unwrap();
    let y_max = y_ends[1].parse().unwrap();
    Area {
        x_min,
        x_max,
        y_min,
        y_max,
    }
}

/*
Need to use reasoning to figure out what code to write! (for part 1, this is)
NOTE: after initially trying to adapt this for part 2 (before giving up and just brute forcing), I came
to have some doubts as to whether this is 100% accurate, particularly for the checks on v_x. It's so easy
to make silly mistakes! But it still gave the correct answer in practice, so I'm not too worried...

Let the starting velocity be (v_x, v_y). We can clearly assume both v_x and v_y are positive.
(If v_x is negative the target can never be hit, and if v_y is negative the max height will be 0 which
is absurd assuming there is any way at all to hit the target from a starting velocity with positive y component.)
Then the x values at successive steps will be:
v_x
v_x + (v_x - 1)
...
v_x + (v_x - 1) + ... + 2 + 1,
after which the x coordinate will never change again (as the x velocity has decreased to 0 and will stay there
forever after).
(This last value is well known to be given by the formula v_x * (v_x + 1) / 2.)
Meanwhile, the y co-ordinate is doing the same thing at each step, with v_x replaced by v_y.
Except it does keep moving afterwards, changing from going up to going down. Thus the highest point is going
to be v_y * (v_y + 1) / 2, so v_y is all we have to work out.

The only constraint is that at at least one of these points on the trajectory must fall inside the target area.
But in practice (in both the example given and in the real data), the y values in the target area are all negative.
But all the y values corresponding to the x values above are clearly positive. They will only go negative when
the change in y has already reached below 0 - and in fact gone down to -v_y. At that point - after 2*v_y + 1 steps -
the y position will finally be back to 0. Only after that will it go through negative numbers, of the form:
-v_y - 1
-2*v_y - 3
-3*v_y - 6

That already restricts v_y quite a bit. In particular, v_y can be AT MOST (-y_min) - 1 (if bigger, -v_y - 1 is
below y_min and hence all the numbers above will be - ie the y co-ord can never be in range.

And anything smaller clearly works.

For such a v_y, we can choose any i=1,2... provided that the value -(i*v_y + i*(i+1)/2) is in the y-range.

The above y position, for a given i, occurs at step 2*v_y + i + 1. At this point, the x-coordinate will be
- if 2*v_y + i + 1 >= v_x: v_x * (v_x + 1) / 2 (call this case 1)
- if 2*v_y + i + 1 < v_x: (2 * v_y + i + 1) + (2 * v_y + i + 2) + ... + v_x (case 2)
In case 1, the x co-ordinate will already have stabilised at its max. In case 2 it will still be "on the way".

So the most general procedure will be as follows:
- start with v_y = -(1 + y_min) and work down from there to 1. Test each v_y as follows:
- find all i such that -(i*v_y + i*(i+1)/2) - or equivalently -(i*v_y + 1 + 2 + ... + i) - is in the y range.
(This is easily done, start with i = 1 and continue until the expression falls below the y_min. The expression
obviously decreases as i gets bigger, since all variables are positive)
- for each such i, compute the value of 2*v_y + i. We now test possible v_x values as follows:
- for v_x = 1,..,2*v_y + i, compute the sum 1 + ... + v_x (equal to v_x * (v_x + 1) / 2). If any of those falls
in the x range, we are done - this value of i works, so this v_y is valid.
- if not, now test larger values of v_x for whether (2*v_y + i + 1) + ... + v_x is in x range or not. This process
cannot go on forever because this expression keeps on increasing (remember that v_y is fixed at this point), so
eventually it will overshoot the range, and if this happens without finding anything within the range, we can
stop and test the next v_y (the current v_y, decremented by 1).

An example of how this works with the given example:

- y_min is -10, so our initial test is for v_y = 9. (Which we know in advance will be successful!)
- we need all i > 1 for which -(i * 9 + 1 + 2 + ... + i) is in the range -10 to -5, inclusive.
Clearly this can be trivially programmed, and here comes out as only i = 1 possible. (I think this will always
be the onlly possibility for the maximum theoretically possible v_y.)
- we now test v_x, first for values up to and including 2*v_y + 1, or 19:
- for such values we need the sum 1 + ... + v_x to be in the x range - here 20..30. Clearly 6 and 7 both work,
while still being <= 19.
So we are done very early in this case, and v_y = 9 is confirmed as correct.

Thus the answer is 1 + 2 + ... + 9 = 45.
*/

// trivial computation but used quite a lot!
fn total_up_to(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn get_max_starting_y(target: Area) -> usize {
    let Area {
        x_min,
        x_max,
        y_min,
        y_max,
    } = target;

    let mut max_starting_y = -(y_min + 1);
    while max_starting_y > 0 {
        let mut i = 1;
        let mut test = -(i * max_starting_y + total_up_to(i as usize) as isize);
        while test >= y_min {
            if test <= y_max {
                for starting_x in 1..=(2 * max_starting_y + i + 1) {
                    let end_x = total_up_to(starting_x as usize) as isize;
                    if end_x >= x_min && end_x <= x_max {
                        return max_starting_y as usize;
                    }
                }
                let mut starting_x = 2 * max_starting_y + i + 2;
                let mut x_test = total_up_to(starting_x as usize) as isize
                    - total_up_to(2 * max_starting_y as usize + i as usize) as isize;
                while x_test <= x_max {
                    if x_test >= x_min {
                        return max_starting_y as usize;
                    }
                    starting_x += 1;
                    x_test = total_up_to(starting_x as usize) as isize
                        - total_up_to(2 * max_starting_y as usize + i as usize) as isize;
                }
            }
            i += 1;
            test = -(i * max_starting_y + total_up_to(i as usize) as isize);
        }
        max_starting_y -= 1;
    }
    panic!("no possible y value found!");
}

fn solve_part_1(target: Area) -> usize {
    let max_starting = get_max_starting_y(target);
    total_up_to(max_starting)
}

// It's not easy to reuse the code of part 1, as much of the logic behind it relied on the assumption
// that v_y - the starting y velocity - had to be positive. That is clearly no longer true in finding all
// paths.
// So we simpy "brute force" instead.
// the starting x velocity has to be positive, and clearly can't be more than x_max.
// the starting y velocity can't be less than y_min, and we already know the max it can be.
// So we have a finite number of possibilities for x, y and can just follow the path blindly for each.

// helper functio to follow the path and see if we hit the target
fn hits_target(target: Area, start_x: isize, start_y: isize) -> bool {
    let Area {
        x_min,
        x_max,
        y_min,
        y_max,
    } = target;

    let mut x = 0;
    let mut y = 0;
    let mut v_x = start_x;
    let mut v_y = start_y;
    while x <= x_max && y >= y_min {
        if x >= x_min && y <= y_max {
            return true;
        }
        x += v_x;
        y += v_y;
        if v_x > 0 {
            v_x -= 1;
        }
        v_y -= 1;
    }
    false
}

fn solve_part_2(target: Area) -> usize {
    let mut total = 0;
    let max_starting_y = get_max_starting_y(target) as isize;
    for x in 1..=target.x_max {
        for y in target.y_min..=max_starting_y {
            if hits_target(target, x, y) {
                total += 1;
            }
        }
    }
    total
}

pub fn part_1() -> usize {
    let target = read_file();
    solve_part_1(target)
}

pub fn part_2() -> usize {
    let target = read_file();
    solve_part_2(target)
}
