







fn main() {


    let grill = vec![
    "--o-o-o-o-o--",
    "--x-x-x-x-x--",
    "--o--o--o--o--",
    "--x--x--x--x--",
    "--o-x-o-x-o--",
    "--x-o-x-o-x--",
    "--o--x-o--x--",
   "-------------"
    ];

    let (vegetarian_skewers, non_vegetarian_skewers, empty_skewers) = check_skewers(&grill);
    println!("Vegetarian skewers: {}", vegetarian_skewers);
    println!("Non-vegetarian skewers: {}", non_vegetarian_skewers);
    println!("Empty skewers: {}", empty_skewers);


}

fn check_skewers(grill: &Vec<&str>) -> (i32, i32, i32) {
    let mut vegetarian_skewers = 0;
    let mut non_vegetarian_skewers = 0;
    let mut empty_skewers = 0;

    for skewer in grill {
        if skewer.contains('x') || skewer.contains('o') {
            if skewer.contains('x'){
                non_vegetarian_skewers += 1;
            }
            else{
                vegetarian_skewers += 1;
            }
        }
        else{
            empty_skewers += 1;
        }
    }

    (vegetarian_skewers, non_vegetarian_skewers, empty_skewers)
}
