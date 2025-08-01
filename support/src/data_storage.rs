
use std::io;
use std::{fs, fs::File, io::Write};
use serde::{Serialize, de::DeserializeOwned};
use bincode::{config};

pub fn save_bin<T>(path: &'static str, data: T)
where T: Serialize,
{
    let encoded = bincode::serde::encode_to_vec(data, config::standard())
                                                .expect("Failed to encode data");
    let mut file = File::create(path).expect(format!("Failed to create to file - {}", path).as_str());
    file.write_all(&encoded).expect(format!("Failed to write to file - {}", path).as_str());
}

pub fn read_bin<T>(path: &'static str) -> io::Result<T> 
where
    T: DeserializeOwned, 
{
    let bytes = fs::read(path)
                            .expect(format!("Cannot open file - {}", path)
                            .as_str());
    let (decoded, _) = bincode::serde::decode_from_slice(&bytes, config::standard())
                                                    .expect("Failed to decode data");

    Ok(decoded)
    
}

#[cfg(test)]
mod tests {
    use crate::data_storage::{save_bin, read_bin};
    use serde::{Deserialize, Serialize};
    use iso_currency::{Currency};
    
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DummyCost{
        pub cost: i32,
        pub currency: Currency,
    
    }

    impl DummyCost {
        pub fn new(cost: i32, currency: Currency) -> Self {
            Self {
                cost,
                currency: currency
            }
        }
    }

    #[test]
    fn test_save_cost(){
        let cost= DummyCost::new(6, Currency::USD);
        print!("{:?} \n", &cost);
        save_bin("../test_data/cost.bin", cost);

        let read_cost: DummyCost = read_bin("../test_data/cost.bin").expect("Failed to read cost");
        print!("After saving \n");
        print!("{:?} \n", read_cost);

    }
}
