#[cfg(test)]
mod tests {
    use tp1_fork_join_108225::sites_information::{
        parsing_error::ParsingError, sites_collection::SitesCollection,
    };

    use serde_json::Value;

    #[test]
    fn we_can_load_one_site() {
        let mut sites = SitesCollection::new();
        let result = sites.load_sites("tests/testing_data/un_archivo");
        assert!(result.is_ok());
        let parsed: Value = sites.generate_json_information("108225");
        assert_eq!(parsed["padron"], "108225");
    }

    #[test]
    fn we_can_load_multiple_sites() {
        let mut sites = SitesCollection::new();
        let result: Result<(), ParsingError> = sites.load_sites("tests/testing_data/dos_archivos");
        assert!(result.is_ok());
        let parsed: Value = sites.generate_json_information("108225");
        assert_eq!(parsed["padron"], "108225");
    }
}
