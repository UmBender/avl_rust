use avl::Tree;

fn main() {
    let mut tree = Tree::<isize, isize>::new();
    let mut elements = vec![2, 4, 5, 6, 7];
    for i in (9..1000).filter(|x| x % 2 == 1) {
        elements.push(i);
    }
    for i in 0..1000 {
        tree.insert(i, i);
    }
    let _ = tree.delete(3);
    let _ = tree.delete(8);
    let _ = tree.delete(0);
    let _ = tree.delete(1);

    for i in (9..1000).filter(|x| x % 2 == 0) {
        tree.delete(i);
    }
    let mut okay = true;
    for i in elements.iter() {
        let result = tree.get(*i);
        if result.is_none() {
            okay = false;
            break;
        }
    }
    if okay {
        println!("All are okay");
    } else {
        println!("There is some error");
    }
    // tree.show();
    println!("tree size: {}", tree.size);
    println!("vector size: {}", elements.len());
}
