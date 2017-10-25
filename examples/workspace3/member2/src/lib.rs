pub fn ignore_coverage_region(param: bool) -> Result<(), &'static str> {
    if param {
        Ok(())
    } else {
        // kcov-ignore-start
        let _ = 1 + 1;
        Err("ignore_this")
        // kcov-ignore-end
    }
}

#[cfg(test)]
mod tests {
    use super::ignore_coverage_region;

    #[test]
    fn it_works() {
        assert_eq!(Ok(()), ignore_coverage_region(true));
    }
}
