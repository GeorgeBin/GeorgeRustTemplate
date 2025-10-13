use crate::protos::multi1::Person;
use crate::protos::multi2::AddressBook;
use protobuf::Message;

pub fn test_protos() {
    println!("=== protobuf 序列化 ===");

    let mut person = Person::new();
    person.age = 12;
    person.name = "George".to_string();
    person.email = "George@gmail.com".to_string();
    println!("person= {person}");

    // 序列化
    let serialized = person.write_to_bytes().unwrap();
    println!("序列化后的字节数: {}", serialized.len());
    println!("序列化数据: {:?}", serialized);

    // 反序列化
    let deserialized = Person::parse_from_bytes(&serialized).unwrap();
    println!("反序列化成功: {}", deserialized.name == "Charlie");

    println!("=== protobuf 嵌套 ===");

    let mut address_book = AddressBook::new();
    address_book.bookname = "Hello".to_string();
    address_book.people.push(person);
    println!("address_book= {address_book}");

    // 序列化
    let serialized2 = address_book.write_to_bytes().unwrap();
    println!("序列化后的字节数: {}", serialized2.len());
    println!("序列化数据: {:?}", serialized2);

    // 反序列化
    let deserialized2 = AddressBook::parse_from_bytes(&serialized2).unwrap();
    println!("反序列化成功: {}", deserialized2.bookname == "Hello");
}
