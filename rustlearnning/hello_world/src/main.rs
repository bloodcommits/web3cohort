fn main() {
    let mut s1 = String::from("tushar");
    println!("{}", s1);
    // borrowing and uding
    change_s(&mut s1);
    println!("{}", s1);

    let newval = change_owner(s1);
    println!("{}", newval);
    // println!("{}", s1);
}

fn change_s(val: &mut String) {
    val.push_str(" gupta");
}

fn change_owner(val: String) -> String {
    let ans = val;
    return ans;
}
