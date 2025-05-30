use chrono::{DateTime, Local}; //TimeDelta};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::{io, process::exit};
use std::num::ParseIntError;


// All structures definition are organized here at the top
// Functions declaration are organized at the bottom (after main())

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Contact {
    name: String,
    phone: String,
    email: String,
    created_at: DateTime<Local>,
}

const STORAGE_FILE_PATH: &str = "./.instance/contact.json";



fn main() {

    println!("\n\n--- Contact BOOK ---\n");
    

    loop {

        // Action menu
        let action = action_menu();
        if action == 0 {
            continue; // Invalid input, prompt again
        }
    
        match action {
            // Add a contact
            1 => {
                let name = get_str_input("Contact Name:");
                
                if !validate_name(&name) {
                    println!("\nName must consist of only alphabet and must not be empty\n");
                    continue;
                }


                let phone = get_str_input("Contact Number:");

                if !validate_number(&phone) {
                    println!("\nNumber must be 10 digits and above. Digits only\n");
                    continue;
                }


                let email = get_str_input("Contact Email:");

                if !validate_email(&email) {
                    println!("\nInvalid email address\n");
                    continue;
                }


                let new_contact = 
                match create_unique_contact(name, phone, email){
                    Ok(contact) => contact,
                    Err(err) => {
                        println!("\nError: {}\n", err);
                        continue;
                    },
                };

                
                let consent: bool = confirm_action("Do you want to save this contact?");
                 
                
                if !consent {println!("\nOperation aborted"); continue;}

                
                add_contact(new_contact);

                println!("\nContact saved!\n\n");
            }
            
            // View all contacts (alphabetically)
            2 => {

                let contacts = sort_contacts_alphabetically();

                if contacts.is_empty() {
                    println!("\nYou do not have any contact\n");
                    continue;
                }

                println!("\n\nYOUR CONTACTS\n");

                for contact in contacts.iter() {
                    display_contact(contact);
                }
    
            }
            
            // Delete contact by name
            3 => {
                let mut contacts = load_contacts();

                let rm_contact: String = get_str_input("Search Contact to delete by name:");

                let index = match get_contact_index_by_name(&rm_contact) {
                    Some(i) => i,
                    None => {println!("\nContact not found\n"); continue;},
                };

                println!();
                display_contact(&contacts[index]);
                
                // Confirm Delete
                let consent: bool = confirm_action("Do you want to DELETE this contact?");
                
                if !consent {println!("\nOperation aborted"); continue;}

                // Delete contact
                let removed = contacts.remove(index);
                save_contacts(&contacts);


                println!();
                display_contact(&removed);
                println!("^^^^^^^^^ \nContact has been delected from Contact Book\n");
                
            }

            // Edit existing contact
            4 => {

                let mut contacts = load_contacts();

                let edt_contact: String = get_str_input("Search Contact to edit by name:");


                // find the position contact in contact list

                let index = match get_contact_index_by_name(&edt_contact) {
                    Some(i) => i,
                    None => {println!("\nContact not found\n"); continue;},
                };


                let old_version = &contacts[index];

                println!("\nYOU ARE ABOUT TO EDIT THIS CONTACT\n");
                display_contact(old_version);
                    
                println!("Enter new data for selected contact. \nPress enter to continue with existing data for each");

               
                // Accept name
                let mut name = get_str_input("Contact Name:");
                
                if name.is_empty(){
                    name = old_version.name.to_string();

                }else if  !validate_name(&name) {
                    println!("\nName must consist of only alphabet and must not be empty\n");
                    continue;
                }


                // Accept phone
                let mut phone = get_str_input("Contact Number:");

                if phone.is_empty(){
                    phone = old_version.phone.to_string();
                } else if !validate_number(&phone) {

                    println!("\nNumber must be 10 digits and above. Digits only\n");
                    continue;
                }


                // Accept email
                let mut email = get_str_input("Contact Email:");

                if email.is_empty(){
                    email = old_version.email.to_string();

                } else if !validate_email(&email) {
                    println!("\nInvalid email address\n");
                    continue;
                }
                

                // Edited version
                let new_version = Contact { 
                    name: name, 
                    phone: phone, 
                    email: email, 
                    created_at: old_version.created_at,
                };

                let prompt = format!("Do you want to save CHANGES ON this contact FROM \n\"{}\" \n\n \tTO \n\n\"{}\"",
                                            display_contact(old_version), display_contact(&new_version));

                println!();
                // Confirm action                            
                let consent: bool = confirm_action(&prompt);
                
                if !consent {println!("\nOperation aborted"); continue;}

                // Save edit
                contacts[index] = new_version;

                save_contacts(&contacts);

                println!("\nContact Updated!")
            }

            // Search contact by name
            5 => {
                let contacts = load_contacts();
                if contacts.is_empty() {
                    println!("\nYou do not have any contact\n");
                    continue;
                }

                let name: String = get_str_input("Enter Contact name to search:");

                let index = match get_contact_index_by_name(&name) {
                    Some(i) => i,
                    None => {println!("\nContact not found\n"); continue;},
                };

                let contact = &contacts[index];
                println!();
                display_contact(contact);
            }
    
            6 => {
                println!("Bye!");
                exit(0);
            }

            _ => {
                println!("Please enter a valid action number.\n");
                continue;
            }
        }

    }

}







fn load_contacts() -> Vec<Contact> {
    let mut file = match File::open(STORAGE_FILE_PATH) {
        Ok(file) => file,
        Err(_) => return vec![],
    };

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    serde_json::from_str(&data).unwrap()
}


fn save_contacts(contancts: &Vec<Contact>) {
    let json = serde_json::to_string(&contancts).unwrap();

    // Create parent folder if does not exist
    let path = Path::new(STORAGE_FILE_PATH);
    if let Some(parent) = path.parent(){
        fs::create_dir_all(parent).unwrap();
    }

    let mut file = File::create(STORAGE_FILE_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn add_contact(contact: Contact) {
    let mut contacts = load_contacts();

    contacts.push(contact);
    save_contacts(&contacts);
}

fn sort_contacts_alphabetically() -> Vec<Contact>{
    let mut contacts = load_contacts();

    contacts.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    contacts
}

fn validate_name(name: &String) -> bool {
    // Must be alphabetic and non-empty
    // Name may contain spaces between alphabets
    name.chars().count() > 0
    &&
    name.chars().all(|c| c.is_alphabetic()|| c.is_whitespace())
}

fn validate_number(phone: &String) -> bool {
    // Must be at least 10 digits
    // Must begin with '0' or '+' and between 11 to 14 digits

    let re = Regex::new(r"^[0|\+]\d{10, 14}").unwrap();

    !(phone.chars().count() < 10)
    &&
    re.is_match(&phone)
}

fn validate_email(email: &String) -> bool {
    // Must resemble an email address containing '@' and '.'
    let re = Regex::new(r"\w+@\w+\.\w+").unwrap();
    re.is_match(&email)
}

// fn calculate_contact_age(datetime: &DateTime<Local>) -> TimeDelta {
//     Local::now() - datetime 
// }


fn create_unique_contact(name: String, phone: String, email: String) -> Result<Contact, String> {
    // Creates a new unique contact
    let new_contact: Contact = Contact {
        name: name,
        phone: phone,
        email: email,
        created_at: Local::now(),
    };

    if contact_exist(&new_contact) {
        return Err("Contact with this name or number already exists".to_string());
    }
    
    Ok(new_contact)

}


fn contact_exist(contact: &Contact) -> bool {
    // Checks if contact name or contact number already exist
    let contacts = load_contacts();

    contacts.iter().find(|cont| cont.phone == contact.phone
    || cont.name == contact.name).is_some()
    
}


fn get_contact_index_by_name(name: &String) -> Option<usize> {
    // Returns a contact index in contact book
    let contacts = load_contacts();

    let index = contacts.iter().position(|cont| &cont.name == name);
    index
}




// I/O FUNCTIONS

fn get_str_input(prompt: &str) -> String {
    println!("\n{} ", prompt);
    let mut value: String = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Input failed");
                
    let value = value.trim().to_string();
    value
}


fn get_u8_input() -> Result<u8, ParseIntError> {
    let mut value: String = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Input failed");

    value.trim().parse::<u8>()
}


fn action_menu() -> u8 {
    println!("\nSelect your action:");
    println!("1. Add a new contact \n2. View all contacts \n3. Delete a contact by name \n4. Edit an existing contact \n5. Search for a contact by name \n6. Exit\n");

    let action: u8 = match get_u8_input() {
        Ok(num) => num,
        Err(_) => {
            println!("\nPlease enter a valid action number.\n");
            0
        },
    };

    action
}

fn confirm_action(prompt: &str) -> bool {
    loop {
        println!("\n{} \n1. Yes \n2. No\n", prompt);

        let consent: u8 = match get_u8_input() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease enter a valid action number.");
                continue;
            },
        };
        
        let mut feedback: bool = false;
        if consent == 1 {feedback = true;}
        return feedback;
    }
}

fn display_contact(contact: &Contact) -> String {
    let output = format!("Name {}\nNumber: {}\nEmail: {}\
    \nCreated on {}",
        contact.name, contact.phone, contact.email, contact.created_at.format("%Y-%m-%d"));

    println!("{}\n", output);
    output
}