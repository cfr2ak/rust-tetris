static O: &'static str = " @@  @@         ";
static I: &'static str = "@@@@            ";
static T: &'static str = "@@@  @          ";
static S: &'static str = "  @@ @@         ";
static Z: &'static str = " @@   @@        ";
static J: &'static str = "@   @@@         ";
static L: &'static str = "  @ @@@         ";
static SW: i32 = 4;
static SH: i32 = 4;

fn main() {
    let mw = 8;
    let mh = 12;
    let gm = init(mw, mh, '#');

    loop {
        update();
        render(mw, &gm);
        if check() == true {
            break;
        }
    }
}

fn init(mw: i32, mh: i32, c: char) -> String {
    // game map String
    let mut gm = String::new();
    // padding for map's horizon (two for left and right)
    let mw = mw + 2;

    for _ in 0..mw {
        gm.push(c);
    }
    for _ in 0..mh {
        for w in 0..mw {
            if w == 0 || w == (mw - 1) {
                gm.push(c);
            } else {
                gm.push(' ');
            }
        }
    }
    for _ in 0..mw {
        gm.push(c);
    }
    gm
}

fn update() {
    input();
}

fn render(mw: i32, gm: &String){
    let mw = mw + 2;
    // counter for interator (gm.chars())
    let mut cnt = 0;
    for c in gm.chars() {
        if cnt < mw {
            print!("{}", c);
        } else {
            println!("");
            print!("{}", c);
            cnt = 0;
        }
        cnt += 1;
    }
}

fn check() -> bool {
    true
}

fn input() {
}
