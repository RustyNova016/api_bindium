use core::convert::Infallible;
use core::marker::PhantomData;

use serde_core::Serialize;
use serde_core::de::DeserializeOwned;
use serde_json_assert::assert_json_eq;

use crate::Parser;
use crate::TextParser;
use crate::api_response::ureq_response::UreqResponseInner;

/// A json parser that can be used to test endpoints. It crashes at any errors, and prints useful messages to point out where it failed
#[derive(Debug)]
pub struct TestingJsonParser<T>(PhantomData<T>)
where
    T: Sized + DeserializeOwned;

impl<T> Parser<UreqResponseInner> for TestingJsonParser<T>
where
    T: Sized + DeserializeOwned,
{
    type Output = T;
    type Error = Infallible;

    fn parse(&self, response: UreqResponseInner) -> Result<Self::Output, Self::Error> {
        let text = match TextParser.parse(response) {
            Ok(val) => {
                println!("✔️ Can parse the body into `String`");
                val
            }
            Err(err) => {
                println!("❌ Cannot parse the body into `String`");
                panic!("{err:?}");
            }
        };

        // === Deserialising check

        let value = match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(val) => {
                println!("✔️ Can parse the body into `serde_json::Value`");
                val
            }
            Err(err) => {
                println!("❌ Cannot parse the body into `serde_json::Value`");
                panic!("{err:?}");
            }
        };

        let value = match serde_json::from_value::<T>(value) {
            Ok(val) => {
                println!("✔️ Can parse the body into the value `T`");
                val
            }
            Err(err) => {
                println!("❌ Cannot parse the body into value `T`");
                panic!("{err:?}");
            }
        };

        Ok(value)
    }
}

impl<T> Default for TestingJsonParser<T>
where
    T: Sized + DeserializeOwned,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// A json parser that can be used to test endpoints. It crashes at any errors, and prints useful messages to point out where it failed
///
/// This also check if the data can be serialized back into the original response
#[derive(Debug)]
pub struct TestingJsonRoundtripParser<T>(PhantomData<T>)
where
    T: Sized + DeserializeOwned + Serialize + Clone;

impl<T> Parser<UreqResponseInner> for TestingJsonRoundtripParser<T>
where
    T: Sized + DeserializeOwned + Serialize + Clone,
{
    type Output = T;
    type Error = Infallible;

    fn parse(&self, response: UreqResponseInner) -> Result<Self::Output, Self::Error> {
        let text = match TextParser.parse(response) {
            Ok(val) => {
                println!("✔️ Can parse the body into `String`");
                val
            }
            Err(err) => {
                println!("❌ Cannot parse the body into `String`");
                panic!("{err:?}");
            }
        };

        // === Deserialising check

        let json_value = match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(val) => {
                println!("✔️ Can parse the body into `serde_json::Value`");
                val
            }
            Err(err) => {
                println!("❌ Cannot parse the body into `serde_json::Value`");
                panic!("{err:?}");
            }
        };

        let value = match serde_json::from_value::<T>(json_value.clone()) {
            Ok(val) => {
                println!("✔️ Can parse the body into the value `T`");
                val
            }
            Err(err) => {
                println!("❌ Cannot parse the body into value `T`");
                panic!("{err:?}");
            }
        };

        // === Rountrip check ===

        let reser = match serde_json::to_value(value.clone()) {
            Ok(val) => {
                println!("✔️ Can serialize the value `T` into a `serde_json::Value`");
                val
            }
            Err(err) => {
                println!("❌ Cannot serialize the value `T` into a `serde_json::Value`");
                panic!("{err:?}");
            }
        };

        assert_json_eq!(reser, json_value);
        assert_json_eq!(json_value, reser);

        Ok(value)
    }
}

impl<T> Default for TestingJsonRoundtripParser<T>
where
    T: Sized + DeserializeOwned + Serialize + Clone,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}
