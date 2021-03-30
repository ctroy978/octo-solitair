use serde_json;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
struct MyStruct{
    somestring: String,
    someint: u64,
}

pub fn play_fish(){

    let incoming = r#"
        {
            "somestring": "hellop",
            "someint": 4
        }
        "#;

let result: MyStruct = serde_json::from_str(incoming).unwrap();

let data = MyStruct{
    somestring: "meme".to_string(),
    someint: 22,
};


println!("{}", serde_json::to_string(&data).unwrap());
println!("{}", result.somestring);


    
}
