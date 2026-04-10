use crate::item_database::ItemDefinition;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyValuesError {
    #[error("Unexpected end of input")]
    UnexpectedEof,
    #[error("Expected '{0}' at position {1}")]
    Expected(char, usize),
    #[error("Invalid token at position {0}")]
    InvalidToken(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KvValue {
    String(String),
    Section(Vec<(String, KvValue)>),
}

impl KvValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            KvValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_section(&self) -> Option<&[(String, KvValue)]> {
        match self {
            KvValue::Section(entries) => Some(entries),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&KvValue> {
        self.as_section()
            .and_then(|entries| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v))
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(|v| v.as_str())
    }
}

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: 0,
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.input[self.pos];
            if ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\r' {
                self.pos += 1;
            } else if self.pos + 1 < self.input.len()
                && ch == b'/'
                && self.input[self.pos + 1] == b'/'
            {
                while self.pos < self.input.len() && self.input[self.pos] != b'\n' {
                    self.pos += 1;
                }
            } else {
                break;
            }
        }
    }

    fn read_quoted_string(&mut self) -> Result<String, KeyValuesError> {
        if self.pos >= self.input.len() || self.input[self.pos] != b'"' {
            return Err(KeyValuesError::Expected('"', self.pos));
        }
        self.pos += 1;

        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] != b'"' {
            if self.input[self.pos] == b'\\' && self.pos + 1 < self.input.len() {
                self.pos += 2;
            } else {
                self.pos += 1;
            }
        }

        if self.pos >= self.input.len() {
            return Err(KeyValuesError::UnexpectedEof);
        }

        let s = String::from_utf8_lossy(&self.input[start..self.pos]).to_string();
        self.pos += 1; // skip closing quote
        Ok(s)
    }

    fn parse_value(&mut self) -> Result<KvValue, KeyValuesError> {
        self.skip_whitespace_and_comments();
        if self.pos >= self.input.len() {
            return Err(KeyValuesError::UnexpectedEof);
        }

        if self.input[self.pos] == b'{' {
            self.parse_section()
        } else if self.input[self.pos] == b'"' {
            Ok(KvValue::String(self.read_quoted_string()?))
        } else {
            Err(KeyValuesError::InvalidToken(self.pos))
        }
    }

    fn parse_section(&mut self) -> Result<KvValue, KeyValuesError> {
        if self.pos >= self.input.len() || self.input[self.pos] != b'{' {
            return Err(KeyValuesError::Expected('{', self.pos));
        }
        self.pos += 1;

        let mut entries = Vec::new();
        loop {
            self.skip_whitespace_and_comments();
            if self.pos >= self.input.len() {
                return Err(KeyValuesError::UnexpectedEof);
            }
            if self.input[self.pos] == b'}' {
                self.pos += 1;
                break;
            }

            let key = self.read_quoted_string()?;
            let value = self.parse_value()?;
            entries.push((key, value));
        }

        Ok(KvValue::Section(entries))
    }

    fn parse_root(&mut self) -> Result<KvValue, KeyValuesError> {
        self.skip_whitespace_and_comments();
        let _root_key = self.read_quoted_string()?;
        self.skip_whitespace_and_comments();
        self.parse_section()
    }
}

/// Parse a KeyValues/VDF string into a tree structure
pub fn parse_kv(input: &str) -> Result<KvValue, KeyValuesError> {
    let mut parser = Parser::new(input);
    parser.parse_root()
}

/// Parse items_game.txt and extract all cosmetic item definitions
pub fn parse_items_game(input: &str) -> Result<Vec<ItemDefinition>, KeyValuesError> {
    let root = parse_kv(input)?;
    let mut items = Vec::new();

    let prefabs_section = root.get("prefabs");

    let items_section = match root.get("items") {
        Some(section) => section,
        None => return Ok(items),
    };

    if let Some(entries) = items_section.as_section() {
        for (key, value) in entries {
            let def_index: u32 = match key.parse() {
                Ok(idx) => idx,
                Err(_) => continue,
            };

            let name = value.get_str("name").unwrap_or("unknown").to_string();

            let prefab_name = value.get_str("prefab").unwrap_or("");
            let is_wearable = prefab_name.contains("wearable")
                || prefab_name.contains("default_item")
                || value.get("item_slot").is_some();

            if !is_wearable {
                continue;
            }

            let item_slot = resolve_field(value, prefabs_section, "item_slot").unwrap_or_default();

            let hero_name = extract_hero_name(value, prefabs_section).unwrap_or_default();

            let rarity = resolve_field(value, prefabs_section, "item_rarity")
                .unwrap_or_else(|| "common".to_string());

            let quality = value
                .get_str("item_quality")
                .and_then(|q| q.parse().ok())
                .unwrap_or(4);

            items.push(ItemDefinition {
                def_index,
                name,
                item_slot,
                hero_name,
                rarity,
                quality,
            });
        }
    }

    Ok(items)
}

fn resolve_field(item: &KvValue, prefabs: Option<&KvValue>, field: &str) -> Option<String> {
    if let Some(val) = item.get_str(field) {
        return Some(val.to_string());
    }

    if let Some(prefab_name) = item.get_str("prefab") {
        if let Some(prefabs_section) = prefabs {
            for name in prefab_name.split_whitespace() {
                if let Some(prefab) = prefabs_section.get(name) {
                    if let Some(val) = prefab.get_str(field) {
                        return Some(val.to_string());
                    }
                }
            }
        }
    }

    None
}

fn extract_hero_name(item: &KvValue, prefabs: Option<&KvValue>) -> Option<String> {
    if let Some(used_by) = item.get("used_by_heroes") {
        if let Some(entries) = used_by.as_section() {
            if let Some((hero, _)) = entries.first() {
                return Some(hero.clone());
            }
        }
    }

    if let Some(prefab_name) = item.get_str("prefab") {
        if let Some(prefabs_section) = prefabs {
            for name in prefab_name.split_whitespace() {
                if let Some(prefab) = prefabs_section.get(name) {
                    if let Some(used_by) = prefab.get("used_by_heroes") {
                        if let Some(entries) = used_by.as_section() {
                            if let Some((hero, _)) = entries.first() {
                                return Some(hero.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_kv() {
        let input = r#"
"root"
{
    "key1"  "value1"
    "key2"  "value2"
}
"#;
        let result = parse_kv(input).unwrap();
        assert_eq!(result.get_str("key1"), Some("value1"));
        assert_eq!(result.get_str("key2"), Some("value2"));
    }

    #[test]
    fn test_parse_nested_kv() {
        let input = r#"
"root"
{
    "section"
    {
        "nested_key"    "nested_value"
    }
}
"#;
        let result = parse_kv(input).unwrap();
        let section = result.get("section").unwrap();
        assert_eq!(section.get_str("nested_key"), Some("nested_value"));
    }

    #[test]
    fn test_parse_with_comments() {
        let input = r#"
"root"
{
    // This is a comment
    "key1"  "value1"
    // Another comment
    "key2"  "value2"
}
"#;
        let result = parse_kv(input).unwrap();
        assert_eq!(result.get_str("key1"), Some("value1"));
        assert_eq!(result.get_str("key2"), Some("value2"));
    }

    #[test]
    fn test_parse_items_game() {
        let input = r#"
"items_game"
{
    "prefabs"
    {
        "wearable"
        {
            "item_slot"     "weapon"
            "item_rarity"   "common"
        }
    }
    "items"
    {
        "4000"
        {
            "name"          "Blade of Voth Domosh"
            "prefab"        "wearable"
            "used_by_heroes"
            {
                "npc_dota_hero_legion_commander"    "1"
            }
        }
        "4001"
        {
            "name"          "Arcana Helm"
            "prefab"        "wearable"
            "item_slot"     "head"
            "item_rarity"   "arcana"
            "used_by_heroes"
            {
                "npc_dota_hero_phantom_assassin"    "1"
            }
        }
        "not_a_number"
        {
            "name"          "should be skipped"
        }
    }
}
"#;
        let items = parse_items_game(input).unwrap();
        assert_eq!(items.len(), 2);

        assert_eq!(items[0].def_index, 4000);
        assert_eq!(items[0].name, "Blade of Voth Domosh");
        assert_eq!(items[0].item_slot, "weapon");
        assert_eq!(items[0].hero_name, "npc_dota_hero_legion_commander");
        assert_eq!(items[0].rarity, "common");

        assert_eq!(items[1].def_index, 4001);
        assert_eq!(items[1].item_slot, "head");
        assert_eq!(items[1].rarity, "arcana");
    }
}
