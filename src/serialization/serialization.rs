use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct S {
    s: String,
    i: i32,
    f: f64,
}

impl S {
    fn to_string(&self) -> String {
        format!("{{ s:\"{s}\", i:{0}, f:{float} }}", self.i, s = self.s, float = self.f)
    }
}

fn main() {
    let s = S {
        s: "hello 😃".to_string(),
        i: 42,
        f: 3.14,
    };

    let s2 = s.to_string();
    println!("{}", s2);

    // assert_eq!(s2, "{ s:\"hello\", i:42, f:3.14 }");

    println!("{:?}", s);


    // from_string("{ i:42, s\"hello\", f:3.14 ") // to complex to do

    let serialized = serde_json::to_string(&s).unwrap();

    println!("{serialized}");

    // let serialized = "{ s:\"hello\" i:42, f:3.14 }";

    let deserialized = serde_json::from_str::<S>(&serialized);
    match deserialized {
        Ok(_x) => { println!("deserialized = {:?}", s); }
        Err(_err) => { println!("Try again"); }
    }

    println!("len:{}", s.s.len());

    let l = s.s.len();
    for b in l.to_be_bytes()
    {
        println!("{:x}", b);
    }

    println!("---");
    // let bytes = s.s.bytes();
    // for i  in 0..s.s.len() {
    //     let b = bytes[i];
    //     println!("{}", b);
    // }

    for b in s.s.bytes() {
        println!("0x{:x}", b);
    }

    // let s0 = "hello";
    // let s1 = "hello".to_string();
}