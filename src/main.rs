use std::io;
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::path::Path;
fn main() {
    println!("Welcome to sorter app.\n\n");
    
    loop {
        println!("1) input data manually\nLoad from file:\n2) from XML\n3) from .txt\n9) exit");
        let mut menu_choice: String = String::new();
        io::stdin().read_line(&mut menu_choice).expect("Failed to read your input");
        match menu_choice.trim() {
            "1" => {
                manual_input_sorting();
            },
            "2" => {
                read_sort_xml();
            },
            "3" => {
                let _ = read_sort_txt();
            },
            "9" => {break;},
            _ => println!("Invalid input provided!")
        }
    }
}

#[derive(Debug, Deserialize)]
struct Person {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Attribute_i32")]
    attribute1_i32: i32,
}

#[derive(Debug, Deserialize)]
struct People {
    #[serde(rename = "Person")]
    people: Vec<Person>
}

fn manual_input_sorting() {
    let mut people: Vec<Person> = Vec::new();
    'grand: loop {
        let mut p_name = String::new();
        'confirmation: loop {
            println!(r#"Please input the object name or "sort" to sort added objects:"#);
            io::stdin()
                .read_line(&mut p_name)
                .expect("Failed to read input");
            if p_name.trim() == "sort" {break 'grand;}
            loop {
                println!("Value read: {} \nIs that correct? (Y/n)", p_name);
                let mut response = String::new();
                    io::stdin()
                    .read_line(&mut response)
                    .expect("Failed to read your response");
                match response.trim() {
                    "Y" => {
                        response.clear();
                        let cleaned_name = p_name.trim().lines();
                        p_name = cleaned_name.collect();
                        break 'confirmation;
                    },
                    "n" => { 
                        p_name.clear();
                        response.clear();
                        break; 
                    },
                    _   => println!("Invalid response provided! \n Try again")
                }
            }
        }

        people.push(add_attributes_numeric(p_name));

        for person in &people {
            println!("{} {}", person.name, person.attribute1_i32);
        }
    }
    
    for element in sort_given_vector(people) {
        println!("{} {}", element.name, element.attribute1_i32);
    }
}

fn read_sort_xml() {
    'path_check: loop {
        println!("Please provide filepath:");
        let mut data_file_path: String = String::new();
        io::stdin().read_line(&mut data_file_path).expect("Failed to read file path");
        let data_file_path = Path::new(data_file_path.trim());
        match (data_file_path.exists() && data_file_path.is_file()) && data_file_path.extension().and_then(std::ffi::OsStr::to_str) == Some("xml"){
            true => {
                let mut file = File::open(data_file_path).expect("Failed to open file");
                let mut xml_content = String::new();
                file.read_to_string(&mut xml_content).expect("Failed to read file");

                let people_xml: People = serde_xml_rs::from_str(&xml_content).expect("Failed to parse XML");
                
                for person in sort_given_vector(people_xml.people) {
                    println!("{} {}", person.name, person.attribute1_i32);
                }
            },
            false => {
                println!("Path invalid");
                continue;
            }
        }
    }
}

fn read_sort_txt() -> io::Result<()> {
    'path_check: loop {
        println!("Please provide filepath:");
        let mut data_file_path: String = String::new();
        io::stdin().read_line(&mut data_file_path).expect("Failed to read file path");
        let data_file_path = Path::new(data_file_path.trim());
        match (data_file_path.exists() && data_file_path.is_file()) && data_file_path.extension().and_then(std::ffi::OsStr::to_str) == Some("txt"){
            true => {
                let file = File::open(data_file_path).expect("Failed to open file");
                let buf_reader = BufReader::new(file);
                let mut people_vec: People = People { people: Vec::new() };

                for line in buf_reader.lines() {
                    let line = line?;
                    let parts: Vec<&str> = line.split_whitespace().collect();

                    if let [firstname, secondname, attribute1_i32] = parts.as_slice() {
                        if let Ok(attribute1_i32) = attribute1_i32.parse() {
                            let person = Person {
                                name: firstname.to_string() + " " + secondname,
                                attribute1_i32,
                            };
                            people_vec.people.push(person);
                        } else {
                            println!("Failed to parse Attribute1_i32: {}", attribute1_i32);
                        }
                    } else {
                        println!("Invalid line format: {}", line);
                    }
                }
                for person in sort_given_vector(people_vec.people) {
                    println!("{} {}", person.name, person.attribute1_i32)
                }
                return Ok(())
            },
            false => {
                println!("Path invalid");
                continue;
            }
        }
    }
}

fn sort_given_vector(mut vector: Vec<Person>) -> Vec<Person> {
    vector.sort_by_key(|person| person.attribute1_i32);

    vector
}

fn create_new_person(name: String, attribute1_i32: i32) -> Person {
    Person {
        name,
        attribute1_i32,
    }
}


fn add_attributes_numeric(name: String) -> Person{
    let mut attribute = String::new();
    'confirmation: loop {
        println!("Please input the numeric object attribute:");
        io::stdin()
            .read_line(&mut attribute)
            .expect("Failed to read input");
        let mut attribute:i32 = attribute.trim().parse::<i32>().unwrap();
        loop {
            println!("Value read: {} \nIs that correct? (Y/n)", attribute);
            let mut response = String::new();
                io::stdin()
                .read_line(&mut response)
                .expect("Failed to read your response");
            match response.trim() {
                "Y" => {
                    response.clear();
                    return create_new_person(name, attribute)
                },
                "n" => {
                    attribute = 0;
                    response.clear();
                    break;
                },
                _   => println!("Invalid response provided! \n Try again")
            }
        }
    }
}


