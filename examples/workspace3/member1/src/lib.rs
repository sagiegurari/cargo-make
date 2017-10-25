pub fn ignore_coverage_1(param: bool) -> Result<(), &'static str> {
    if param {
        Ok(())
    } else {
        Err("ignore_this") // kcov-ignore
    }
}

pub fn ignore_coverage_2(param: bool) -> Result<(), &'static str> {
    if param || true {
        Ok(())
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::{ignore_coverage_1, ignore_coverage_2};

    #[test]
    fn it_works() {
        assert_eq!(Ok(()), ignore_coverage_1(true));
    }

    #[test]
    fn it_still_works() {
        assert_eq!(Ok(()), ignore_coverage_2(true));
    }
}
