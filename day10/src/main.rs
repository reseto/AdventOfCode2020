use std::time::{Instant};
// use itertools::Itertools;

fn read_input(input: &str) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();
    for line in input.lines() {
        numbers.push(line.parse::<u64>().unwrap());
    }
    numbers
}

fn _part_a_ugly(input: &str) {
    let mut numbers = read_input(input);
    numbers.sort_unstable();
    
    let mut one_diff = 0;
    let mut three_diff = 0;
    let mut taken: Vec<u64> = Vec::new();
    taken.push(0);
    for num in numbers {
        let last = *taken.last().unwrap();
        let diff = num - last;
        if diff > 3 {
            panic!("bad input");
        } else if diff == 3 {
            three_diff += 1;
        } else if diff ==1 {
            one_diff += 1;
        }
        taken.push(num);
    }
    let last = *taken.last().unwrap();
    taken.push(last + 3);
    three_diff += 1;
    // println!("taken: {:?}", taken);
    // println!("taken len: {:?}", taken.len());
    // println!("1 diff: {:?}", one_diff);
    // println!("3 diff: {:?}", three_diff);
    println!("answer a: {}", one_diff * three_diff);
}

fn part_a(input: &str) {
    let mut numbers = read_input(input);
    numbers.sort_unstable();

    let mut diff = [0,0,0,0]; // diff[0 and 2] never used
    let mut last = 0;
    for num in numbers {
        diff[(num - last) as usize] += 1;
        last = num;
    }
    println!("answer a: {}", diff[1] * (diff[3] + 1));
}

fn part_b(input: &str) {
    let mut numbers = read_input(input);
    numbers.sort_unstable();
    numbers.push(*numbers.last().unwrap() + 3);

    let mut ans: [usize; 300] = [0; 300];
    ans[0] = 1;
    for n in &numbers {
        let num = *n as usize;
        if num < 3 { ans[num] = num; continue; }
        ans[num] = ans[num-1] + ans[num-2] + ans[num-3];
    }

    // println!("sorted: {:?}", numbers);
    // println!("last: {}", *numbers.last().unwrap());
    println!("answer b: {}", ans[*numbers.last().unwrap() as usize]);
}

fn main() {
    // let input = _example();
    // let input = _example2();
    let input = input();

    let start_a = Instant::now();
    part_a(&input);
    let start_b = Instant::now();
    part_b(&input);
    println!(" A took {:?}", start_a.elapsed());
    println!(" B took {:?}", start_b.elapsed());
}

fn _example() -> &'static str {
r########"16
10
15
5
1
11
7
19
6
12
4"########
}

fn _example2() -> &'static str {
r########"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"########
}

fn input() -> &'static str {
r########"46
63
21
115
125
35
89
17
116
90
51
66
111
142
148
60
2
50
82
20
47
24
80
101
103
16
34
72
145
141
124
14
123
27
62
61
95
138
29
7
149
147
104
152
22
81
11
96
97
30
41
98
59
45
88
37
10
114
110
4
56
122
139
117
108
91
36
146
131
109
31
75
70
140
38
121
3
28
118
54
107
84
15
76
71
102
130
132
87
55
129
83
23
42
69
1
77
135
128
94"########
}

