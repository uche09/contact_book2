use chrono::{DateTime, Local, TimeDelta};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::{io, process::exit};


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

    println!("\n\nCLI PHONE BOOK\n");
    

    loop {

        println!("\nSelect your action:");

        println!("1. Add a contact \n2. View all contacts \n3. Delete contact by name \n4. Edit existing contact \n5. Search for contact by name \n6. Exit");
    
        let mut action: String = String::new();
        io::stdin().read_line(&mut action).expect("Input failed");
    
        let action: u8 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease enter a valid action number.\n");
                continue;  
            },
        };
    
        match action {
            // Add a contact
            1 => {
                let mut name: String = String::new();
                let mut phone: String = String::new();
                let mut email: String = String::new();

                println!("\nContact Name:");
                io::stdin()
                    .read_line(&mut name)
                    .expect("Input failed");
                
                let name = name.trim().to_string();
                
                if !validate_name(&name) {
                    println!("\nName must consist of only alphabet and must not be empty\n");
                    continue;
                }



                println!("\nContact Number:");
                io::stdin()
                    .read_line(&mut phone)
                    .expect("Input failed");

                let phone = phone.trim().to_string();

                if !validate_number(&phone) {
                    println!("\nNumber must be 10 digits and above. Digits only\n");
                    continue;
                }


                println!("\nContact Email:");
                io::stdin()
                    .read_line(&mut email)
                    .expect("Input failed");

                let email = email.trim().to_string();

                if !validate_email(&email) {
                    println!("\nInvalid email address\n");
                    continue;
                }


                let new_contact = Contact {
                    name: name,
                    phone: phone,
                    email: email,
                    created_at: Local::now(),
                };

                // Check if contact already exist in a contact.
                let contacts = load_contacts();

                if contacts.iter().find(|cont| cont.phone == new_contact.phone
                || cont.name == new_contact.name).is_some() {
                    println!("\nA contact already exist with this name or number. Please check name and number\n");
                    continue;
                }


                
                let consent: bool = loop {
                    println!("\nDo you want to save this contact? \n1. Yes \n2. No");

                    let mut consent: String = String::new();
                    io::stdin()
                        .read_line(&mut consent)
                        .expect("Input failed");

                    let consent: u8 = match consent.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("\nPlease enter a valid action number.");
                            continue;
                        },
                    };
                    
                    let mut feedback: bool = true;
                    if consent == 2 {feedback = false;}
                    break feedback
                };
                
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
                    println!("Name {}\nNumber: {}\nEmail: {}\n",
                    contact.name, contact.phone, contact.email);
                    println!();
                }
    
            }
            
            // Delete contact by name
            3 => {
                let mut contacts = load_contacts();

                println!("\nEnter Contact name to delete:");
                let mut rm_contact: String = String::new();

                io::stdin()
                    .read_line(&mut rm_contact)
                    .expect("Input failed");
                
                rm_contact = rm_contact.trim().to_string();

                let index = match contacts.iter().position(|cont| cont.name == rm_contact) {
                    Some(i) => i,
                    None => {println!("\nContact not found\n"); continue;},
                };

                println!("\nName: {} \nPhone: {} \nEmail: {} \nContact is {} days old",
                    contacts[index].name, contacts[index].phone, contacts[index].email,
                    calculate_contact_age(&contacts[index].created_at).num_days());

                
                // Confirm Delete
                let consent: bool = loop {
                    println!("\nDo you want to DELETE this contact? \n1. Yes \n2. No\n");

                    let mut consent: String = String::new();
                    io::stdin()
                        .read_line(&mut consent)
                        .expect("Input failed");

                    let consent: u8 = match consent.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("\nPlease enter a valid action number.\n");
                            continue;
                        },
                    };
                    
                    let mut feedback: bool = false;
                    if consent == 1 {feedback = true;}
                    break feedback
                };
                
                if !consent {println!("\nOperation aborted"); continue;}

                // Delete contact
                let removed = contacts.remove(index);
                save_contacts(&contacts);

                println!("{} \n{} \n{} \n^^^^^^^^^ \nContact has been delected from Contact Book\n",
                    removed.name, removed.phone, removed.email);
                
            }

            // Edit existing contact
            4 => {

                let mut contacts = load_contacts();

                println!("\nEnter Contact name to edit:");
                let mut edt_contact: String = String::new();

                io::stdin()
                    .read_line(&mut edt_contact)
                    .expect("Input failed");
                
                edt_contact = edt_contact.trim().to_string();


                // find the position contact in contact list

                let index = match contacts.iter().position(|cont| cont.name == edt_contact) {
                    Some(i) => i,
                    None => {println!("\nContact not found\n"); continue;},
                };


                let old_version = &contacts[index];

                println!("\nDO YOU WANT TO EDIT THIS CONTACT?\n");
                println!("Name {}\nNumber: {}\nEmail: {} \nContact is {} days old\n",
                    old_version.name, old_version.phone, old_version.email,
                    calculate_contact_age(&old_version.created_at).num_days());
                    
                println!("Enter new data for selected contact. \nPress enter to continue with existing data for each");

                // Accept new values
                let mut name: String = String::new();
                let mut phone: String = String::new();
                let mut email: String = String::new();

                // Accept name
                println!("\nContact Name:");
                io::stdin()
                    .read_line(&mut name)
                    .expect("Input failed");
                
                let mut name = name.trim().to_string();
                
                if name.is_empty(){
                    name = old_version.name.to_string();

                }else if  !validate_name(&name) {
                    println!("\nName must consist of only alphabet and must not be empty\n");
                    continue;
                }


                // Accept phone
                println!("\nContact Number:");
                io::stdin()
                    .read_line(&mut phone)
                    .expect("Input failed");

                let mut phone = phone.trim().to_string();

                if phone.is_empty(){
                    phone = old_version.phone.to_string();
                } else if !validate_number(&phone) {

                    println!("\nNumber must be 10 digits and above. Digits only\n");
                    continue;
                }


                // Accept email
                println!("\nContact Email:");
                io::stdin()
                    .read_line(&mut email)
                    .expect("Input failed");

                let mut email = email.trim().to_string();

                if email.is_empty(){
                    email = old_version.email.to_string();

                } else if !validate_email(&email) {
                    println!("\nInvalid email address\n");
                    continue;
                }
                

                // Edited version
                let new_version = Contact{
                    name: name,
                    phone: phone,
                    email: email,
                    created_at: old_version.created_at,
                };


                // Confirm Edit
                let consent: bool = loop {
                    println!("\nDo you want to save CHANGES ON this contact? \n{} => {} \n{} => {} \n{} => {}",
                        old_version.name, new_version.name, old_version.phone, new_version.phone, old_version.email, new_version.email);

                    println!("1. Yes \n2. No\n");

                    let mut consent: String = String::new();
                    io::stdin()
                        .read_line(&mut consent)
                        .expect("Input failed");

                    let consent: u8 = match consent.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("\nPlease enter a valid action number.\n");
                            continue;
                        },
                    };
                    
                    let mut feedback: bool = false;
                    if consent == 1 {feedback = true;}
                    break feedback
                };
                
                if !consent {println!("\nOperation aborted"); continue;}

                // Save edit
                contacts[index] = new_version;

                save_contacts(&contacts);

                println!("\nContact Updated!")
            }

            // Search contact by name
            5 => {
                let contacts = load_contacts();

                println!("\nEnter Contact name to search:");
                let mut name: String = String::new();

                io::stdin()
                    .read_line(&mut name)
                    .expect("Input failed");
                
                name = name.trim().to_string();

                let index = match contacts.iter().position(|cont| cont.name == name) {
                    Some(i) => i,
                    None => {println!("\nContact not found\n"); continue;},
                };

                let contact = &contacts[index];
                println!("\n{} \n{} \n{} \nContact is {} days old", 
                    contact.name, contact.phone, contact.email, calculate_contact_age(&contact.created_at).num_days())
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

fn calculate_contact_age(datetime: &DateTime<Local>) -> TimeDelta {
    Local::now() - datetime 
}