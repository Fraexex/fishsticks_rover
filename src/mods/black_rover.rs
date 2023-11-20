/*
// Movement
U = Up
D = Down
L = Left
R = Right

// Actions (append with space key & direction (U/D/L/R)):
A = Attack
M = Mine
S = Scan
P = Place

// Buying (prefix with B and space key):
S = Sight
A = Attack
D = Drill (mine)
R = Radar (one time)
B = Battery (one time)
H = Heal

1) Get a ton of cobblestone
2) Build the Berlin wall
3) ...
4) Profit
*/
//use rand::Rng;

struct FileInputs {
    movement: String,
    action: String,
    buying: String,
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn up(&mut self) {
        self.y += 1;
    }
}

