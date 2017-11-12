extern crate serde_bencode;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;

mod info;
mod metainfo;
pub use info::File;
pub use info::Info;
pub use metainfo::Metainfo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
