/*
    Replace: r - w, l - w, R - W, L - W
*/


pub fn uwuify(to_uwu: String) -> String {
    let uwuified = to_uwu.replace("r", "w").replace("l", "w").replace("R", "W").replace("L", "W");
    uwuified
}
