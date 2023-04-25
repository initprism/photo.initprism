pub fn to_location_identfier_string(from: &str) -> String {
    let mut identifier = from.clone().to_string();

    identifier.retain(|c| c != ' ' && c != '_');

    if identifier == "Singapore" || identifier == "HongKong" {
        identifier.push_str("City");
    }

    identifier
}

