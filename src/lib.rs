use client::{ClientBase, OpenAI};

pub mod client;
mod consts;
mod data;

pub fn init(token: String, org: String) -> OpenAI {
    OpenAI::new(token, org)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let token: String = "api_key".to_owned();
        let org: String = "organization_name".to_owned();
        let result = init(token, org);

        assert!(!result.org.is_empty())
    }
}
