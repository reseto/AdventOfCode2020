use std::time;
use std::fmt;
// use itertools::Itertools;

fn part_a_ugly(input: &str) -> time::Duration {
    let start = time::Instant::now();
    let mut main: Vec<String> = Vec::new();
    let mut side: Vec<String> = Vec::new();
    
    for line in input.lines() {
        let instruction = &line[0..3]; // char at [3] is always space
        let rest = line[4..].to_string();
        let visited = format!("0{}", instruction);
        main.push(visited);
        main.push(rest);
    }
    main.reverse();
    // program is loaded, now execute instructions:
    let mut sum = 0;
    loop {
        if main.is_empty() { break; }
        let visited = main.pop().unwrap();
        let rest = main.pop().unwrap();
        let number = as_num(&rest);
        if &visited[0..1] == "1" { 
            println!("finishing, found visited operation {} {}", visited, rest);
            break; 
        }
        let instruction = &visited[1..4];

        match instruction {
            "fin" => {
                // added custom instruction to end of input
                println!("finishing successfully"); // will not happen
                break;
            }
            "nop" => {
                // println!("nop");
                side.push(rest);
                side.push(format!("1{}", &visited[1..])); // visit
            },
            "acc" => {
                sum += number;
                // println!("adding {}, sum is {}", number, sum);
                side.push(rest);
                side.push(format!("1{}", &visited[1..])); // visit
            },
            "jmp" => {
                println!("jumping {}", number);
                let n: usize = rest[1..].parse().unwrap();
                let direction: char = rest[0..1].parse().unwrap();
                if n == 0 { panic!("jump to the same location!"); }
                side.push(rest);
                side.push(format!("1{}", &visited[1..])); // visit

                // vidlactvo nezastavis, najebat to zo strany na stranu jak robotnik
                if direction == '-' {
                    for _i in 0..n+1 {
                        let a = side.pop().unwrap();
                        let b = side.pop().unwrap();
                        main.push(b);
                        main.push(a);
                    }
                } else {
                    for _i in 0..n-1 {
                        let a = main.pop().unwrap();
                        let b = main.pop().unwrap();
                        side.push(b);
                        side.push(a);
                    }
                }
            },
            _ => println!("PANIC {} {}", instruction, number)
        }
    }
    println!(" answer A: {}", sum);
    start.elapsed()
}

fn as_num(line: &String) -> isize {
    let sign = &line[0..1];
    let n: usize = line[1..].parse().unwrap();
    let mut number: isize = n as isize; 
    if sign == "-" { number -= 2 * number; }
    // println!("number: {} :end", number);
    number
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
struct Op<'a> { // instruction, but OP is shorter
    code: &'a str,
    num: isize,
    seen: isize
}

impl PartialEq for Op<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.num == other.num && self.seen == other.seen
    }
}

impl fmt::Display for Op<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Op: {} {:4} {}", self.code, self.num, if self.seen == 1 { "seen" } else { "" })
    }
}

fn load_program(input: &str) -> Vec<Op> {
    input.lines().map(|line| Op {
        code: &line[0..3],
        num: line[4..].parse::<isize>().unwrap(),
        seen: 0
    }).collect()
}

fn run_program(vec: &Vec<Op>) -> Result<i32, i32> {
    let mut ip: usize = 0; // instruction pointer
    let mut sum: i32 = 0;
    let mut program = vec.clone();
    loop {
        let pip = &mut program[ip];
        // let pip = program.get_mut(ip).unwrap(); // same thing
        if pip.seen == 1 { return Err(sum) }
        pip.seen = 1;
        match pip.code {
            "fin" => { return Ok(sum) }
            "nop" => { ip += 1 },
            "acc" => { sum += pip.num as i32; ip += 1 },
            "jmp" => { ip = (ip as isize + pip.num) as usize },
            _ => return Err(0) // should not happen
        }
    }
}

fn part_a_nice(input: &str) -> time::Duration {
    let start = time::Instant::now();
    let program: Vec<Op> = load_program(input);
    println!(" answer A: {}", run_program(&program).unwrap_err());
    start.elapsed()
}

fn part_b(input: &str) -> time::Duration {
    let start = time::Instant::now();
    let program: Vec<Op> = load_program(input);
    let mut i = 200; // start later
    loop {
        let mut clone = program.clone();
        let mut op = &mut clone[i];
        match op.code {
            "nop" => { op.code = "jmp" },
            "jmp" => { op.code = "nop" },
            "fin" => { break; },
            _ => { i += 1; continue; } 
        }
        // println!("running program with changed position {} to instruction {}", i, op);
        match run_program(&clone) {
            Ok(sum) => { println!(" answer B: {}", sum); break; }
            Err(_sum) => { i += 1; continue; }
        }
    }
    start.elapsed()
}

fn main() {
    // let input = _example();
    let input = converted();

    let ta = part_a_ugly(&input);
    let tan = part_a_nice(&input);
    let tb = part_b(&input);

    println!("");
    println!("a took {:?}", ta);
    println!("a took {:?}", tan);
    println!("b took {:?}", tb);
}

fn _example() -> &'static str {
    // 6
r########"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
fin +0"########
}

fn converted() -> &'static str {
r########"acc +6
acc +21
nop +297
jmp +2
jmp +156
acc -7
acc +6
acc -16
acc +14
jmp +127
acc +8
acc -16
acc +48
nop +189
jmp +214
acc +20
acc +25
acc +3
acc +10
jmp +571
acc -7
acc -16
acc +29
acc +24
jmp +346
acc +1
acc -11
acc -14
acc +14
jmp +18
acc -5
acc +22
jmp +351
acc +13
acc +6
nop -2
acc +20
jmp +405
acc +13
acc +40
jmp +265
acc +32
acc -3
acc +13
acc +9
jmp +175
jmp +237
nop +113
jmp +127
acc -10
acc +49
nop -36
acc +17
jmp +156
nop +181
jmp +419
acc +11
jmp +1
acc +49
nop +187
jmp +427
nop +568
acc -14
nop +481
acc +47
jmp +163
jmp +309
jmp +410
acc +23
acc -17
nop +229
acc -4
jmp +538
acc -12
jmp +200
acc +35
acc +22
acc -14
acc +46
jmp +388
acc +20
acc +15
acc +0
acc +26
jmp +197
jmp +267
acc +33
jmp -82
acc +44
acc -19
jmp +42
acc +14
jmp +1
jmp +263
acc -18
jmp +80
acc -12
jmp +248
acc +0
acc +44
acc +6
jmp +134
jmp +1
acc +2
acc +38
jmp +40
acc +30
acc -1
jmp +141
jmp +1
jmp +460
nop +241
acc +34
acc +35
jmp +1
jmp -6
nop +172
acc +6
nop +299
jmp +298
acc +20
nop +52
acc +47
acc +32
jmp +293
acc +22
acc -4
jmp +417
acc -8
jmp +276
jmp +452
jmp +381
acc +0
nop -79
jmp +101
jmp +253
acc +30
acc +42
acc +21
jmp +231
acc +35
acc +20
acc -10
acc -19
jmp +173
jmp +2
jmp -16
acc -8
acc +47
acc -9
acc +11
jmp -35
acc -2
jmp +121
acc -16
acc -19
acc +47
acc +34
jmp -11
jmp +34
acc +40
acc -3
acc -18
nop +359
jmp +9
acc +26
jmp +117
acc -8
acc -14
jmp +1
jmp +123
nop +117
jmp -63
jmp +13
jmp +227
acc +41
jmp +207
nop -86
acc +37
acc -15
jmp +55
acc +24
acc +8
jmp +1
jmp +79
jmp +270
jmp -125
jmp +214
jmp +100
acc -8
acc -19
acc +23
jmp -167
nop -18
acc -14
acc +19
nop +291
jmp +361
acc +25
acc +21
nop +99
jmp +229
nop +228
acc +4
acc +24
jmp -12
jmp +1
acc +26
jmp +75
acc +22
nop +105
acc +46
acc -8
jmp -81
acc +46
jmp -168
acc +14
jmp -57
acc -13
jmp +137
nop +362
acc +28
jmp +352
acc +8
acc +21
acc +30
acc +13
jmp -91
acc +39
jmp +296
acc +27
acc +16
acc +5
acc +48
jmp -6
nop -210
acc +29
acc +47
jmp -78
jmp +228
acc +13
acc -11
acc +3
jmp +96
acc +0
jmp +313
acc +30
jmp +251
jmp +203
nop -202
nop -177
acc -17
acc +30
jmp -128
jmp +227
nop +84
acc +21
acc +3
jmp -18
acc +33
nop -128
nop +368
jmp -87
acc +30
jmp +88
acc -3
acc +17
jmp +63
acc +37
acc -13
jmp +340
jmp +1
acc +11
jmp +325
jmp -58
acc +43
jmp +23
jmp +157
acc +35
acc +10
jmp +25
jmp +124
jmp -109
nop +40
jmp +183
acc +46
acc +37
jmp +88
acc -8
jmp +162
acc +4
acc +22
jmp +220
acc +32
jmp -214
acc +3
nop -56
acc +30
jmp -138
acc +22
jmp +329
acc +12
nop +115
acc +38
jmp -231
acc +9
jmp +1
acc +25
acc +47
jmp +329
acc +14
nop +220
acc -19
acc -19
jmp +225
jmp -248
jmp +249
jmp -231
nop -30
acc +26
acc +32
acc +12
jmp +190
acc +4
jmp -251
acc +20
nop -27
acc +20
acc +16
jmp -41
acc +21
acc +45
acc +24
jmp -65
acc +39
acc -9
acc -12
acc +23
jmp -50
acc +49
nop -105
acc +17
jmp +180
acc +14
jmp +238
acc +1
nop -285
acc +26
acc -15
jmp +216
nop -95
jmp +60
jmp -261
acc +49
acc +31
jmp +210
jmp +1
acc -5
jmp -304
acc +48
acc +0
nop +2
jmp -347
acc +50
jmp +148
acc +5
nop -193
acc -5
acc +0
jmp +221
acc +39
acc -14
jmp +39
acc +24
jmp -100
acc +0
acc -16
acc +4
jmp -190
acc +21
acc -2
acc -16
jmp +162
jmp +28
acc +26
acc +19
acc -5
jmp -362
acc -16
nop -6
jmp -223
jmp +101
acc -7
acc -10
acc -16
jmp -146
nop +126
acc -18
acc +25
nop -232
jmp +61
jmp -86
acc -9
nop -20
jmp -318
jmp -90
nop -315
acc +33
acc +0
acc +18
jmp +41
jmp +1
jmp +8
acc +4
acc +13
acc -19
jmp -128
acc +46
jmp -322
jmp -404
acc -3
acc +24
acc +5
nop +159
jmp -322
acc -10
acc -18
acc +42
acc +44
jmp -130
acc -4
acc +4
jmp -302
jmp +18
acc -3
acc -18
acc +29
acc +36
jmp -405
acc +15
nop +91
jmp +98
acc -14
nop +88
acc +16
jmp +86
acc -14
acc +20
acc -10
nop -7
jmp +81
jmp +74
acc +3
acc +18
jmp -294
acc +16
jmp -173
nop +65
acc +31
acc -14
jmp -77
acc +16
nop -415
jmp -33
acc +41
nop -398
jmp +46
acc +50
acc +7
acc -19
nop -15
jmp -317
acc +38
jmp -298
acc -3
jmp -273
jmp -120
nop +10
acc +26
jmp +39
jmp -348
nop -204
acc +0
acc +0
nop -179
jmp -208
jmp -52
nop -299
acc +36
acc +14
jmp +108
jmp -476
jmp -283
acc +31
acc +33
acc +32
acc -13
jmp -295
acc +42
acc +27
jmp -112
acc +37
acc +22
acc +34
jmp -236
acc -2
acc +33
nop +75
nop -503
jmp -146
nop -324
jmp -148
acc +39
jmp -112
acc +32
acc -15
acc -5
jmp +8
acc +30
jmp -196
acc +5
jmp -361
acc +29
jmp -153
jmp -26
jmp -344
acc +39
acc +25
nop -153
acc +34
jmp -79
nop -97
nop -90
acc +21
acc -16
jmp +4
acc +41
jmp +1
jmp -233
acc +37
jmp -235
acc +41
acc +44
jmp -12
nop +1
acc -18
jmp +74
acc +49
nop -30
acc -17
jmp -364
acc -13
acc +50
jmp -223
nop -507
acc +17
acc +2
jmp -302
acc +29
jmp -8
acc +33
nop -514
acc +15
acc +31
jmp -189
nop -98
acc +47
acc +21
jmp -322
jmp -77
jmp -555
acc +44
nop -142
jmp -96
acc +45
acc +44
nop -121
jmp -322
acc +35
acc +48
acc +0
jmp -393
jmp +1
acc +0
nop -356
jmp -512
acc +31
acc -10
acc +37
jmp -71
acc +25
acc -8
jmp -200
acc -7
acc -12
acc +0
acc -16
jmp -369
acc +14
acc +11
acc +35
jmp -510
jmp -472
acc +39
acc +27
acc +50
nop -360
jmp -498
acc +13
jmp -500
acc -17
acc +11
acc +10
acc +7
jmp -11
acc +6
acc +30
acc +25
acc -19
jmp -373
acc -4
acc +19
nop -329
jmp -582
acc +11
acc +26
acc -1
acc +35
jmp -548
nop -59
nop -576
acc +27
jmp -145
acc -7
acc +41
nop -272
acc +35
jmp -170
acc -6
acc -4
acc +33
acc +33
jmp +1
fin +0"########
}

