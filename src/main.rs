// enum Ops {
//     Add,
//     Sub,
//     Mul,
//     Div,
// }

struct Cli {
    op: String,
    num_one: String,
    num_two: String,
}

fn main() {
    let num_one = std::env::args().nth(1).expect("no first number");
    let op = std::env::args().nth(2).expect("no op");
    let num_two = std::env::args().nth(3).expect("no second number");
    let args = Cli {
        op,
        num_one,
        num_two,
    };

    let result :i32;
    if args.op == "+" {
        result = args.num_one.parse::<i32>().unwrap() + args.num_two.parse::<i32>().unwrap();
    } else if args.op == "-" {
        result = args.num_one.parse::<i32>().unwrap() - args.num_two.parse::<i32>().unwrap();
    } else if args.op == "%" {
        result = args.num_one.parse::<i32>().unwrap() % args.num_two.parse::<i32>().unwrap();
    } else if args.op == "*" {
        result = args.num_one.parse::<i32>().unwrap() * args.num_two.parse::<i32>().unwrap();
    } else {
        panic!("Operation \"{}\" not supported", args.op)
    }

    println!(
        "{} {} {} = {}",
        &args.num_one, &args.op, &args.num_two, &result
    );

    println!("Hello, world!");
}
