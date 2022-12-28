struct Omni;

impl PartialEq<bool> for Omni {
    fn eq(&self, _: &bool) -> bool {
        true
    }
}

const OMNIBOOL: Omni = Omni {};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals_to_both() {
        assert!(
            OMNIBOOL == true && OMNIBOOL == false,
            "OMNIBOOL should be equal to both"
        )
    }
}
