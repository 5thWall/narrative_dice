struct Pool {
    boost: u8,
    hazard: u8,
    skill: u8,
    challenge: u8,
    proficiency: u8,
    setback: u8,
}

impl Pool {
    fn new(boost: u8, hazard: u8, skill: u8, challenge: u8, proficiency: u8, setback: u8) -> Self {
        Self {
            boost,
            hazard,
            skill,
            challenge,
            proficency,
            setback,
        }
    }

    fn blank() -> Self {
        Self {
            boost: 0,
            hazard: 0,
            skill: 0,
            challenge: 0,
            proficency: 0,
            setback: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
