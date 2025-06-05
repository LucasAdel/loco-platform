use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Comprehensive Adelaide suburb coordinates database
/// Each suburb has its precise latitude and longitude
pub static ADELAIDE_SUBURBS: Lazy<HashMap<&'static str, (f64, f64)>> = Lazy::new(|| {
    let mut suburbs = HashMap::new();
    
    // Adelaide CBD and Inner Suburbs
    suburbs.insert("adelaide", (-34.9285, 138.6007));
    suburbs.insert("adelaide cbd", (-34.9285, 138.6007));
    suburbs.insert("north adelaide", (-34.9065, 138.5934));
    suburbs.insert("kent town", (-34.9206, 138.6201));
    suburbs.insert("hackney", (-34.9167, 138.6142));
    suburbs.insert("college park", (-34.9167, 138.6083));
    suburbs.insert("thorngate", (-34.9167, 138.5833));
    suburbs.insert("medindie", (-34.9000, 138.6000));
    suburbs.insert("gilberton", (-34.9000, 138.6083));
    suburbs.insert("walkerville", (-34.8917, 138.6167));
    suburbs.insert("vale park", (-34.8833, 138.6167));
    
    // Eastern Suburbs
    suburbs.insert("norwood", (-34.9206, 138.6326));
    suburbs.insert("kensington", (-34.9211, 138.6453));
    suburbs.insert("marryatville", (-34.9250, 138.6450));
    suburbs.insert("heathpool", (-34.9333, 138.6500));
    suburbs.insert("kensington park", (-34.9253, 138.6489));
    suburbs.insert("leabrook", (-34.9361, 138.6456));
    suburbs.insert("burnside", (-34.9397, 138.6444));
    suburbs.insert("linden park", (-34.9456, 138.6406));
    suburbs.insert("tusmore", (-34.9431, 138.6489));
    suburbs.insert("glen osmond", (-34.9556, 138.6339));
    suburbs.insert("mount osmond", (-34.9633, 138.6450));
    suburbs.insert("waterfall gully", (-34.9706, 138.6778));
    suburbs.insert("crafers", (-35.0047, 138.7089));
    suburbs.insert("stirling", (-35.0006, 138.7158));
    suburbs.insert("aldgate", (-35.0156, 138.7356));
    
    // Western Suburbs
    suburbs.insert("thebarton", (-34.9167, 138.5700));
    suburbs.insert("torrensville", (-34.9192, 138.5611));
    suburbs.insert("mile end", (-34.9250, 138.5683));
    suburbs.insert("hilton", (-34.9333, 138.5600));
    suburbs.insert("cowandilla", (-34.9350, 138.5475));
    suburbs.insert("brooklyn park", (-34.9283, 138.5433));
    suburbs.insert("lockleys", (-34.9283, 138.5317));
    suburbs.insert("west beach", (-34.9467, 138.5083));
    suburbs.insert("henley beach", (-34.9167, 138.4933));
    suburbs.insert("grange", (-34.9017, 138.4883));
    suburbs.insert("tennyson", (-34.8833, 138.4817));
    suburbs.insert("west lakes", (-34.8667, 138.4917));
    suburbs.insert("seaton", (-34.8917, 138.5083));
    suburbs.insert("findon", (-34.9033, 138.5317));
    suburbs.insert("fulham", (-34.9167, 138.5150));
    suburbs.insert("flinders park", (-34.9083, 138.5433));
    
    // Northern Suburbs
    suburbs.insert("prospect", (-34.8833, 138.5950));
    suburbs.insert("nailsworth", (-34.8833, 138.6083));
    suburbs.insert("sefton park", (-34.8750, 138.6083));
    suburbs.insert("blair athol", (-34.8633, 138.5983));
    suburbs.insert("kilburn", (-34.8583, 138.5917));
    suburbs.insert("enfield", (-34.8483, 138.6017));
    suburbs.insert("clearview", (-34.8450, 138.6117));
    suburbs.insert("northgate", (-34.8433, 138.6283));
    suburbs.insert("klemzig", (-34.8667, 138.6333));
    suburbs.insert("hillcrest", (-34.8500, 138.6500));
    suburbs.insert("gilles plains", (-34.8467, 138.6617));
    suburbs.insert("greenacres", (-34.8667, 138.6550));
    suburbs.insert("hampstead gardens", (-34.8683, 138.6433));
    suburbs.insert("manningham", (-34.8750, 138.6417));
    suburbs.insert("vale park", (-34.8833, 138.6167));
    suburbs.insert("mawson lakes", (-34.8111, 138.6111));
    suburbs.insert("salisbury", (-34.7639, 138.6444));
    suburbs.insert("parafield", (-34.7917, 138.6333));
    suburbs.insert("modbury", (-34.8306, 138.6833));
    suburbs.insert("tea tree gully", (-34.8250, 138.7000));
    suburbs.insert("golden grove", (-34.7833, 138.7000));
    
    // Southern Suburbs
    suburbs.insert("unley", (-34.9506, 138.6089));
    suburbs.insert("parkside", (-34.9456, 138.6122));
    suburbs.insert("fullarton", (-34.9511, 138.6289));
    suburbs.insert("highgate", (-34.9606, 138.6189));
    suburbs.insert("myrtle bank", (-34.9639, 138.6289));
    suburbs.insert("urrbrae", (-34.9706, 138.6339));
    suburbs.insert("springfield", (-34.9717, 138.6156));
    suburbs.insert("clapham", (-34.9917, 138.6033));
    suburbs.insert("colonel light gardens", (-34.9722, 138.5989));
    suburbs.insert("cumberland park", (-34.9667, 138.5950));
    suburbs.insert("daw park", (-34.9750, 138.5867));
    suburbs.insert("melrose park", (-34.9833, 138.5767));
    suburbs.insert("edwardstown", (-34.9817, 138.5717));
    suburbs.insert("south plympton", (-34.9783, 138.5550));
    suburbs.insert("glandore", (-34.9633, 138.5683));
    suburbs.insert("kurralta park", (-34.9500, 138.5633));
    suburbs.insert("netley", (-34.9467, 138.5500));
    suburbs.insert("ascot park", (-34.9917, 138.5533));
    suburbs.insert("park holme", (-34.9967, 138.5517));
    suburbs.insert("marion", (-35.0117, 138.5450));
    suburbs.insert("oaklands park", (-35.0167, 138.5450));
    suburbs.insert("warradale", (-35.0067, 138.5350));
    suburbs.insert("brighton", (-35.0167, 138.5150));
    suburbs.insert("seacliff", (-35.0367, 138.5167));
    suburbs.insert("marino", (-35.0467, 138.5117));
    suburbs.insert("hallett cove", (-35.0794, 138.5106));
    suburbs.insert("sheidow park", (-35.0667, 138.5217));
    suburbs.insert("trott park", (-35.0833, 138.5417));
    suburbs.insert("woodcroft", (-35.1167, 138.5583));
    suburbs.insert("morphett vale", (-35.1333, 138.5333));
    suburbs.insert("christies beach", (-35.1342, 138.4742));
    suburbs.insert("noarlunga", (-35.1389, 138.4917));
    suburbs.insert("port noarlunga", (-35.1531, 138.4678));
    
    // Inner Southern
    suburbs.insert("goodwood", (-34.9506, 138.5850));
    suburbs.insert("millswood", (-34.9544, 138.5922));
    suburbs.insert("wayville", (-34.9456, 138.5922));
    suburbs.insert("forestville", (-34.9506, 138.5783));
    suburbs.insert("black forest", (-34.9594, 138.5733));
    suburbs.insert("clarence gardens", (-34.9644, 138.5817));
    suburbs.insert("kings park", (-34.9556, 138.5867));
    
    // North Eastern Suburbs
    suburbs.insert("paradise", (-34.8917, 138.6667));
    suburbs.insert("campbelltown", (-34.8806, 138.6611));
    suburbs.insert("athelstone", (-34.8722, 138.7000));
    suburbs.insert("newton", (-34.8806, 138.6778));
    suburbs.insert("rostrevor", (-34.8694, 138.6944));
    suburbs.insert("magill", (-34.9139, 138.6694));
    suburbs.insert("tranmere", (-34.9028, 138.6611));
    suburbs.insert("payneham", (-34.8972, 138.6389));
    suburbs.insert("st peters", (-34.9056, 138.6222));
    suburbs.insert("marden", (-34.9000, 138.6278));
    suburbs.insert("royston park", (-34.8972, 138.6306));
    suburbs.insert("joslin", (-34.9028, 138.6250));
    suburbs.insert("maylands", (-34.9139, 138.6306));
    suburbs.insert("stepney", (-34.9111, 138.6278));
    suburbs.insert("trinity gardens", (-34.9139, 138.6389));
    suburbs.insert("st morris", (-34.9194, 138.6417));
    suburbs.insert("felixstow", (-34.8889, 138.6472));
    suburbs.insert("glynde", (-34.8917, 138.6556));
    suburbs.insert("firle", (-34.9083, 138.6472));
    
    // Hills District
    suburbs.insert("blackwood", (-35.0194, 138.6133));
    suburbs.insert("belair", (-35.0019, 138.6256));
    suburbs.insert("glenalta", (-35.0042, 138.6106));
    suburbs.insert("bellevue heights", (-35.0156, 138.6256));
    suburbs.insert("eden hills", (-35.0033, 138.6050));
    suburbs.insert("coromandel valley", (-35.0333, 138.6167));
    suburbs.insert("flagstaff hill", (-35.0483, 138.5717));
    suburbs.insert("aberfoyle park", (-35.0617, 138.5967));
    suburbs.insert("happy valley", (-35.0833, 138.5633));
    suburbs.insert("reynella", (-35.0958, 138.5450));
    suburbs.insert("old reynella", (-35.0975, 138.5383));
    suburbs.insert("morphett vale east", (-35.1200, 138.5533));
    suburbs.insert("onkaparinga hills", (-35.1333, 138.5667));
    suburbs.insert("clarendon", (-35.1167, 138.6333));
    suburbs.insert("cherry gardens", (-35.0667, 138.6500));
    suburbs.insert("ironbank", (-35.0333, 138.6667));
    suburbs.insert("longwood", (-35.0500, 138.7000));
    suburbs.insert("heathfield", (-35.0167, 138.7000));
    suburbs.insert("mylor", (-35.0333, 138.7333));
    suburbs.insert("bridgewater", (-35.0047, 138.7589));
    suburbs.insert("mount barker", (-35.0667, 138.8667));
    
    // Port Adelaide Region
    suburbs.insert("port adelaide", (-34.8478, 138.5078));
    suburbs.insert("alberton", (-34.8583, 138.5183));
    suburbs.insert("queenstown", (-34.8550, 138.5133));
    suburbs.insert("rosewater", (-34.8483, 138.5200));
    suburbs.insert("pennington", (-34.8717, 138.5217));
    suburbs.insert("ottoway", (-34.8350, 138.5317));
    suburbs.insert("north haven", (-34.7917, 138.4933));
    suburbs.insert("osborne", (-34.8083, 138.4833));
    suburbs.insert("taperoo", (-34.8133, 138.4900));
    suburbs.insert("largs bay", (-34.8250, 138.4833));
    suburbs.insert("largs north", (-34.8150, 138.4850));
    suburbs.insert("peterhead", (-34.8383, 138.4933));
    suburbs.insert("birkenhead", (-34.8400, 138.4983));
    suburbs.insert("ethelton", (-34.8333, 138.5017));
    suburbs.insert("semaphore", (-34.8394, 138.4825));
    suburbs.insert("semaphore park", (-34.8517, 138.4783));
    suburbs.insert("west lakes shore", (-34.8728, 138.4831));
    
    // Far Northern Suburbs
    suburbs.insert("elizabeth", (-34.7139, 138.6706));
    suburbs.insert("elizabeth north", (-34.7000, 138.6833));
    suburbs.insert("elizabeth south", (-34.7278, 138.6578));
    suburbs.insert("elizabeth vale", (-34.7389, 138.6639));
    suburbs.insert("elizabeth grove", (-34.7111, 138.6528));
    suburbs.insert("elizabeth downs", (-34.7028, 138.6972));
    suburbs.insert("elizabeth park", (-34.7194, 138.6806));
    suburbs.insert("elizabeth east", (-34.7222, 138.6944));
    suburbs.insert("elizabeth west", (-34.7222, 138.6472));
    suburbs.insert("davoren park", (-34.7000, 138.6556));
    suburbs.insert("smithfield", (-34.6833, 138.6889));
    suburbs.insert("smithfield plains", (-34.6778, 138.7056));
    suburbs.insert("munno para", (-34.6667, 138.7000));
    suburbs.insert("angle vale", (-34.6444, 138.6556));
    suburbs.insert("virginia", (-34.6333, 138.5667));
    suburbs.insert("waterloo corner", (-34.6944, 138.5833));
    suburbs.insert("burton", (-34.7306, 138.5889));
    suburbs.insert("direk", (-34.7111, 138.5833));
    suburbs.insert("salisbury north", (-34.7500, 138.6222));
    suburbs.insert("salisbury east", (-34.7667, 138.6667));
    suburbs.insert("salisbury south", (-34.7806, 138.6389));
    suburbs.insert("salisbury heights", (-34.7722, 138.6806));
    suburbs.insert("salisbury park", (-34.7583, 138.6083));
    suburbs.insert("salisbury plain", (-34.7833, 138.6000));
    suburbs.insert("salisbury downs", (-34.7722, 138.6194));
    suburbs.insert("paralowie", (-34.7556, 138.6083));
    suburbs.insert("brahma lodge", (-34.7833, 138.6083));
    suburbs.insert("pooraka", (-34.8250, 138.6222));
    suburbs.insert("ingle farm", (-34.8306, 138.6389));
    suburbs.insert("walkley heights", (-34.8194, 138.6556));
    suburbs.insert("northfield", (-34.8472, 138.6222));
    suburbs.insert("gawler", (-34.5972, 138.7444));
    
    suburbs
});

/// Get coordinates for a suburb name (case-insensitive)
pub fn get_suburb_coordinates(suburb: &str) -> Option<(f64, f64)> {
    let suburb_lower = suburb.to_lowercase();
    let suburb_normalized = suburb_lower.trim();
    
    // Direct lookup
    if let Some(&coords) = ADELAIDE_SUBURBS.get(suburb_normalized) {
        return Some(coords);
    }
    
    // Try without common suffixes
    let without_suffix = suburb_normalized
        .trim_end_matches(" north")
        .trim_end_matches(" south")
        .trim_end_matches(" east")
        .trim_end_matches(" west")
        .trim_end_matches(" heights")
        .trim_end_matches(" park")
        .trim_end_matches(" gardens")
        .trim_end_matches(" vale");
    
    if let Some(&coords) = ADELAIDE_SUBURBS.get(without_suffix) {
        return Some(coords);
    }
    
    // Fuzzy match - find closest suburb name
    let mut best_match = None;
    let mut best_score = 0.0;
    
    for (key, &coords) in ADELAIDE_SUBURBS.iter() {
        let score = similarity_score(suburb_normalized, key);
        if score > best_score && score > 0.8 {
            best_score = score;
            best_match = Some(coords);
        }
    }
    
    best_match
}

/// Simple string similarity score (0.0 to 1.0)
fn similarity_score(a: &str, b: &str) -> f64 {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    let max_len = a_chars.len().max(b_chars.len());
    if max_len == 0 {
        return 1.0;
    }
    
    let mut matches = 0;
    let min_len = a_chars.len().min(b_chars.len());
    
    for i in 0..min_len {
        if a_chars[i] == b_chars[i] {
            matches += 1;
        }
    }
    
    matches as f64 / max_len as f64
}

/// Validate if coordinates are within Adelaide metro area
pub fn is_valid_adelaide_location(lat: f64, lng: f64) -> bool {
    // Adelaide metro boundaries (approximate)
    lat >= -35.35 && lat <= -34.60 && lng >= 138.45 && lng <= 138.85
}

/// Fix swapped coordinates (common data entry error)
pub fn fix_swapped_coordinates(lat: f64, lng: f64) -> (f64, f64) {
    // Australian coordinates should have negative latitude and positive longitude > 100
    if lat > 0.0 && lng < 0.0 && lng > -40.0 {
        // Likely swapped
        return (lng, lat);
    }
    
    // If longitude is in latitude range for Adelaide
    if lng >= -35.35 && lng <= -34.60 && lat >= 138.45 && lat <= 138.85 {
        // Definitely swapped
        return (lng, lat);
    }
    
    (lat, lng)
}

/// Get coordinates with fallback logic
pub fn get_location_with_fallback(
    provided_lat: Option<f64>,
    provided_lng: Option<f64>,
    suburb: Option<&str>,
    postcode: Option<&str>,
) -> (f64, f64) {
    // Try provided coordinates first
    if let (Some(lat), Some(lng)) = (provided_lat, provided_lng) {
        let (lat, lng) = fix_swapped_coordinates(lat, lng);
        if is_valid_adelaide_location(lat, lng) {
            return (lat, lng);
        }
    }
    
    // Try suburb lookup
    if let Some(suburb_name) = suburb {
        if let Some(coords) = get_suburb_coordinates(suburb_name) {
            return coords;
        }
    }
    
    // Try postcode to suburb mapping
    if let Some(pc) = postcode {
        if let Some(suburb_name) = postcode_to_suburb(pc) {
            if let Some(coords) = get_suburb_coordinates(suburb_name) {
                return coords;
            }
        }
    }
    
    // Default to Adelaide CBD
    (-34.9285, 138.6007)
}

/// Map postcodes to primary suburbs
fn postcode_to_suburb(postcode: &str) -> Option<&'static str> {
    match postcode {
        "5000" => Some("adelaide"),
        "5006" => Some("north adelaide"),
        "5061" => Some("unley"),
        "5062" => Some("mitcham"),
        "5063" => Some("parkside"),
        "5064" => Some("prospect"),
        "5065" => Some("norwood"),
        "5066" => Some("burnside"),
        "5067" => Some("kensington"),
        "5068" => Some("kensington park"),
        "5069" => Some("st peters"),
        "5070" => Some("paradise"),
        "5072" => Some("rostrevor"),
        "5073" => Some("tranmere"),
        "5074" => Some("campbelltown"),
        "5075" => Some("athelstone"),
        "5076" => Some("modbury"),
        "5081" => Some("walkerville"),
        "5082" => Some("prospect"),
        "5083" => Some("nailsworth"),
        "5084" => Some("blair athol"),
        "5085" => Some("enfield"),
        "5086" => Some("greenacres"),
        "5087" => Some("klemzig"),
        "5088" => Some("holden hill"),
        "5089" => Some("clearview"),
        "5090" => Some("northgate"),
        "5091" => Some("modbury"),
        "5092" => Some("modbury north"),
        "5093" => Some("tea tree gully"),
        "5094" => Some("parafield"),
        "5095" => Some("mawson lakes"),
        "5096" => Some("ingle farm"),
        "5097" => Some("golden grove"),
        "5098" => Some("pooraka"),
        "5106" => Some("parafield"),
        "5107" => Some("parafield gardens"),
        "5108" => Some("salisbury"),
        "5109" => Some("salisbury heights"),
        "5110" => Some("salisbury east"),
        "5112" => Some("elizabeth"),
        "5113" => Some("elizabeth north"),
        "5114" => Some("smithfield"),
        "5115" => Some("munno para"),
        "5116" => Some("gawler west"),
        "5117" => Some("angle vale"),
        "5118" => Some("gawler"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_suburb_coordinates() {
        assert_eq!(get_suburb_coordinates("norwood"), Some((-34.9206, 138.6326)));
        assert_eq!(get_suburb_coordinates("NORWOOD"), Some((-34.9206, 138.6326)));
        assert_eq!(get_suburb_coordinates("Norwood"), Some((-34.9206, 138.6326)));
        assert_eq!(get_suburb_coordinates("unknown_suburb"), None);
    }

    #[test]
    fn test_coordinate_validation() {
        assert!(is_valid_adelaide_location(-34.9285, 138.6007)); // Adelaide CBD
        assert!(!is_valid_adelaide_location(-33.8688, 151.2093)); // Sydney
        assert!(!is_valid_adelaide_location(-37.8136, 144.9631)); // Melbourne
    }

    #[test]
    fn test_fix_swapped_coordinates() {
        assert_eq!(fix_swapped_coordinates(138.6007, -34.9285), (-34.9285, 138.6007));
        assert_eq!(fix_swapped_coordinates(-34.9285, 138.6007), (-34.9285, 138.6007));
    }
}