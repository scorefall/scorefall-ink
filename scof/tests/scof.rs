use muon_rs as muon;

#[test]
fn metadata() {
    let input = include_str!("../scof/Meta.muon");

    let meta: scof::Meta = muon::from_str(input).unwrap();

    println!("{:?}", meta);

    let output = muon::to_string(&meta).unwrap();

    println!("{}", output);

    let meta_clone: scof::Meta = muon::from_str(&output).unwrap();

    assert_eq!(meta, meta_clone);
    assert_eq!(input, output);
}

/*#[test]
fn movement() {
    let input = include_str!("../scof/Movement/The Beginning.muon");

    let movemt: scof::Mvmt = muon::from_str(input).unwrap();
    let movemt: scof::Movement = movemt.into();

    println!("{:?}", movemt);

    let output = muon::to_string(&movemt).unwrap();

    println!("{}", output);

    let movemt_clone: scof::Movement = muon::from_str(&output).unwrap();

    assert_eq!(movemt, movemt_clone);
    assert_eq!(input, output);
}*/

#[test]
fn style() {
    let input = include_str!("../scof/Style.muon");

    let style: scof::Style = muon::from_str(input).unwrap();

    println!("{:?}", style);

    let output = muon::to_string(&style).unwrap();

    println!("{}", output);

    let style_clone: scof::Style = muon::from_str(&output).unwrap();

    assert_eq!(style, style_clone);
    assert_eq!(input, output);
}
