extern crate pnet;

use pnet::datalink;

#[derive(Debug)]
struct Interface {
    name: String,
}

#[derive(Debug)]
struct InterfaceBuilder<NameType> {
    name: NameType,
}

impl InterfaceBuilder<()> {
    pub fn new() -> Self {
        InterfaceBuilder {
            name: (),
        }
    }
}

impl InterfaceBuilder<String> {
    pub fn build(self) -> Interface {
        Interface{ name: self.name }
    }
}

impl<NameType> InterfaceBuilder<NameType> {
    pub fn name<S: Into<String>>(self, name: S) -> InterfaceBuilder<String> {
        InterfaceBuilder {name: name.into()}
    }
}

fn main() {
    let interface_names = vec!["lo0", "en0", "en1"];

    let interfaces = interface_names
        .into_iter()
        .map(|name| InterfaceBuilder::new().name(name).build())
        .collect::<Vec<Interface>>();
    println!("interfaces: {:?}", interfaces);

    let interfaces_device = datalink::interfaces();
    println!("interfaces_device: {:?}", interfaces_device);
//    println!("{}", interfaces[0].to_string());
//    interface_names.into_iter()


    // datalink::channelで



//    println!("interface = {:?}", interface.build());
}