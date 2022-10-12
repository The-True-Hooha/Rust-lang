use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

fn main() {

    println!("hello, welcome to the rust-todo cli app");

    // program takes in three argument from the user and displays that in the terminal
    // run program like this cargo run -- hi welcome 
    let todo_action = std::env::args().nth(1).expect("please input an action"); // reads the args passed starting from the first element
    let todo_item = std::env::args().nth(2).expect("please input an item"); // 
    let mut _todo = TodoApp::add_new_todo().expect("Initialization of todo failed");
    // let todo_word = std::env::args().nth(3).expect("please input a word"); //

    println!("{:?}, {:?}", todo_action, todo_item);

    let mut todo = TodoApp{
        map: HashMap::new(),
    };
    if todo_action == "add" {
        todo.insert_action(todo_item);
        match todo.save(){
            Ok(_) => println!("your todo is saved"),
            Err(error) => println!("error occurred here: {}", error),
        }
    }

    struct TodoApp{
        map: HashMap<String, bool>
    }

    impl TodoApp {
        // updates the hashmap with new todo_items
        fn add_new_todo() -> Result<TodoApp, std::io::Error>{
            
            // opens the todo.txt file
            let mut open_todo = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .read(true)
                .open("todo.txt")?;
            
            // read the contents in a new string
            let mut todo_content = String::new();
            &mut open_todo.read_to_string(&mut todo_content)?;

            // allocate an empty Hashmap for the string
            let mut map = HashMap::new();

            //loops over each lines of the file
            for entries in todo_content.lines(){
                // splits and binds the values
                let mut values = entries.split('\t');
                let key = values.next().expect("No Key");
                let val = values.next().expect("No value");

                // inserts them into the hashmap
                map.insert(String::from(key), bool::from_str(val).unwrap());
            }
            Ok(TodoApp { map })

        }
        fn insert_action(&mut self, key: String){
            self.map.insert(key, true);
        }

        // saves the new action to the disk
        fn save(self) -> Result<(), std::io::Error> {
            let mut content_body = String::new();
            for(k, v) in self.map {
                let record = format!("{}\t{}\n", k, v);
                content_body.push_str(&record)
            }
            std::fs::write("todo.txt", content_body)
        }
    }

}

