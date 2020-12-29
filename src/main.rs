
mod israeliqueue;
pub use israeliqueue::IsraeliQueue;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    pub fn new(first_name: String, last_name: String) -> Person {
        Person {
            first_name,
            last_name,
        }
    }
}

fn person_comparator(a: &Person, b: &Person) -> bool {
    a.last_name == b.last_name
}


fn main() {
    let people = vec![
        Person::new(String::from("Bruno"), String::from("Vieira")),
        Person::new(String::from("Bruno"), String::from("Pereira")),
        Person::new(String::from("Bruno"), String::from("Ferreira")),
        Person::new(String::from("Carla"), String::from("Pereira")),
        Person::new(String::from("Thaís"), String::from("Pereira")),
        Person::new(String::from("Alex"), String::from("Ferreira")),
        Person::new(String::from("Maria"), String::from("Vieira")),
        Person::new(String::from("João"), String::from("das Couves")),
    ];

    let mut people_queue = IsraeliQueue::new();


    for person in people {
        people_queue.queue(person, Some(&person_comparator));
    }
    for person in people_queue.into_iter() {
        println!("{:?}", person);
    }
}