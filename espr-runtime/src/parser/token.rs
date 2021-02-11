//! Parser for tokens
//!
//! Table 2 — WSN of token definitions
//! ------------------------------------
//!
//! ```text
//! KEYWORD           = USER_DEFINED_KEYWORD | STANDARD_KEYWORD .
//!
//! USER_DEFINED_KEYWORD = "!" UPPER { UPPER | DIGIT } .
//!
//! STANDARD_KEYWORD  = UPPER { UPPER | DIGIT } .
//!
//! SIGN              = "+" | "-" .
//!
//! INTEGER           = [ SIGN ] DIGIT { DIGIT } .
//!
//! REAL              = [ SIGN ] DIGIT { DIGIT } "." { DIGIT }
//!                     [ "E" [ SIGN ] DIGIT { DIGIT } ] .
//!
//! STRING            = "'" { SPECIAL | DIGIT | SPACE | LOWER | UPPER |
//!                     HIGH_CODEPOINT |
//!                     APOSTROPHE APOSTROPHE |
//!                     REVERSE_SOLIDUS REVERSE_SOLIDUS |
//!                     CONTROL_DIRECTIVE } "'" .
//!
//! ENTITY_INSTANCE_NAME      = "#" ( DIGIT ) { DIGIT } .
//!
//! VALUE_INSTANCE_NAME       = "@" ( DIGIT ) { DIGIT } .
//!
//! CONSTANT_ENTITY_NAME      = "#" ( UPPER ) { UPPER | DIGIT } .
//!
//! CONSTANT_VALUE_NAME       = "@" ( UPPER ) { UPPER | DIGIT } .
//!
//! LHS_OCCURRENCE_NAME       = ( ENTITY_INSTANCE_NAME | VALUE_INSTANCE_NAME ) .
//!
//! RHS_OCCURRENCE_NAME       = ( ENTITY_INSTANCE_NAME | VALUE_INSTANCE_NAME |
//!                               CONSTANT_ENTITY_NAME | CONSTANT_VALUE_NAME) .
//!
//! ANCHOR_NAME       = "<" URI_FRAGMENT_IDENTIFIER ">" .
//!
//! TAG_NAME          = ( UPPER | LOWER) { UPPER | LOWER | DIGIT } .
//!
//! RESOURCE          = "<" UNIVERSAL_RESOURCE_IDENTIFIER ">" .
//!
//! ENUMERATION       = "." UPPER { UPPER | DIGIT } "." .
//!
//! HEX               = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" |
//!                     "8" | "9" | "A" | "B" | "C" | "D" | "E" | "F" .
//!
//! BINARY            = """" ( "0" | "1" | "2" | "3" ) { HEX } """" .
//!
//! SIGNATURE_CONTENT = BASE64 .
//! ```
//!
