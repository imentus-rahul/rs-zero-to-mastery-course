// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct NameData<'a> {
    inner_names: Vec<&'a str>,
}
struct TitleData<'a> {
    inner_titles: Vec<&'a str>,
}

fn main() {
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();
    let names: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(1))
        .collect();
    let titles: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(4))
        .collect();

    // printing filtered values
    for n in titles.iter().take(3) {
        println!("{}", n)
    }

    let mut NameStruct = NameData {
        inner_names: vec![],
    };

    let mut TitleStruct = TitleData {
        inner_titles: vec![],
    };

    // iter by default gives (& reference) value
    for n in names.iter() {
        NameStruct.inner_names.push(n);
    }
    for n in titles.iter() {
        TitleStruct.inner_titles.push(n);
    }

    // println!("{:?}", NameStruct.inner_names.iter());
    // println!("{:?}", TitleStruct.inner_titles.iter());

    let data = NameStruct
        .inner_names
        .iter()
        .zip(TitleStruct.inner_titles.iter());

    // printing the zip created
    for (name, title) in data{
    println!("🚀 ~ file: a32.rs ~ line 62 ~ fnmain ~ name {:?}, title {:?}", name, title);

    }
}
