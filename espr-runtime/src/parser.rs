//! Parse ASCII exchange structure defined by ISO-10303-21
//!
//! Table 1 — WSN defining subsets of the basic alphabet
//! -----------------------------------------------------
//!
//! ```text
//! SPACE    = " " .
//! DIGIT    = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" .
//! LOWER    = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h"
//!          | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p"
//!          | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x"
//!          | "y" | "z" .
//! UPPER    = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H"
//!          | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P"
//!          | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X"
//!          | "Y" | "Z" | "_" .
//! SPECIAL  = "!" | """" | "*" | "$" | "%" | "&" | "." | "#" | "+" | ","  | "-" | "(" | ")" | "?" | "/" | ":" | ";" | "<"  | "=" | ">" | "@" | "[" | "]" | "{" | "|" | "}"  | "^" | "`" | "~" .
//! REVERSE_SOLIDUS  = "\" .
//! APOSTROPHE = "'" .
//! LATIN_CODEPOINT = SPACE | DIGIT | LOWER | UPPER | SPECIAL | REVERSE_SOLIDUS | APOSTROPHE
//! HIGH_CODEPOINT = (U+0080 to U+10FFFF, see 5.2)
//! ```
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
//! Table 3 — WSN of the exchange structure
//! ----------------------------------------
//!
//! ```text
//! EXCHANGE_FILE      = "ISO-10303-21;"
//!                      HEADER_SECTION [ ANCHOR_SECTION ]
//!                      [ REFERENCE_SECTION ] { DATA_SECTION }
//!                      "END-ISO-10303-21;" { SIGNATURE_SECTION }.
//!
//! HEADER_SECTION     = "HEADER;"
//!                      HEADER_ENTITY HEADER_ENTITY HEADER_ENTITY
//!                      [HEADER_ENTITY_LIST]
//!                      "ENDSEC;" .
//! HEADER_ENTITY_LIST = HEADER_ENTITY { HEADER_ENTITY } .
//! HEADER_ENTITY      = KEYWORD  "(" [ PARAMETER_LIST ] ")" ";" .
//!
//! PARAMETER_LIST     = PARAMETER { "," PARAMETER } .
//! PARAMETER          = TYPED_PARAMETER  |
//!                      UNTYPED_PARAMETER | OMITTED_PARAMETER  .
//! TYPED_PARAMETER    = KEYWORD "(" PARAMETER ")" .
//! UNTYPED_PARAMETER  = "$" | INTEGER | REAL | STRING | RHS_OCCURENCE_NAME
//!                      | ENUMERATION | BINARY | LIST .
//! OMITTED_PARAMETER  = "*" .
//! LIST               = "(" [ PARAMETER { "," PARAMETER } ] ")" .
//!
//! ANCHOR_SECTION     = "ANCHOR;" ANCHOR_LIST "ENDSEC;" .
//! ANCHOR_LIST        = { ANCHOR } .
//! ANCHOR             = ANCHOR_NAME "=" ANCHOR_ITEM { ANCHOR_TAG } ";" .
//! ANCHOR_ITEM        = "$" | INTEGER | REAL | STRING | ENUMERATION | BINARY
//!                      | RHS_OCCURRENCE_NAME | RESOURCE | ANCHOR_ITEM_LIST .
//! ANCHOR_ITEM_LIST   = "(" [ ANCHOR_ITEM { "," ANCHOR_ITEM } ] ")" .
//! ANCHOR_TAG         = "{" TAG_NAME ":" ANCHOR_ITEM "}" .
//!
//! REFERENCE_SECTION  = "REFERENCE;" REFERENCE_LIST "ENDSEC;" .
//! REFERENCE_LIST     = { REFERENCE } .
//! REFERENCE          = LHS_OCCURRENCE_NAME "=" RESOURCE ";" .
//!
//! DATA_SECTION       = "DATA" [ "(" PARAMETER_LIST ")" ] ";"
//!                      ENTITY_INSTANCE_LIST "ENDSEC;" .
//! ENTITY_INSTANCE_LIST = { ENTITY_INSTANCE } .
//! ENTITY_INSTANCE    = SIMPLE_ENTITY_INSTANCE | COMPLEX_ENTITY_INSTANCE .
//! SIMPLE_ENTITY_INSTANCE  = ENTITY_INSTANCE_NAME "=" SIMPLE_RECORD ";" .
//! COMPLEX_ENTITY_INSTANCE = ENTITY_INSTANCE_NAME "=" SUBSUPER_RECORD ";" .
//! SIMPLE_RECORD      = KEYWORD "(" [ PARAMETER_LIST ] ")" .
//! SUBSUPER_RECORD    = "(" SIMPLE_RECORD_LIST ")" .
//! SIMPLE_RECORD_LIST = SIMPLE_RECORD { SIMPLE_RECORD } .
//!
//! SIGNATURE_SECTION  = "SIGNATURE" SIGNATURE_CONTENT "ENDSEC;".
//! ```
//!
//! Table 4 — String control directives
//! ------------------------------------
//!
//! ```text
//! CONTROL_DIRECTIVE = PAGE | ALPHABET | EXTENDED2 | EXTENDED4 | ARBITRARY .
//! PAGE = REVERSE_SOLIDUS "S" REVERSE_SOLIDUS LATIN_CODEPOINT .
//! ALPHABET = REVERSE_SOLIDUS "P" UPPER REVERSE_SOLIDUS .
//! EXTENDED2 = REVERSE_SOLIDUS "X2" REVERSE_SOLIDUS HEX_TWO { HEX_TWO } END_EXTENDED .
//! EXTENDED4 = REVERSE_SOLIDUS "X4" REVERSE_SOLIDUS HEX_FOUR { HEX_FOUR } END_EXTENDED .
//! END_EXTENDED = REVERSE_SOLIDUS "X0" REVERSE_SOLIDUS .
//! ARBITRARY = REVERSE_SOLIDUS "X" REVERSE_SOLIDUS HEX_ONE .
//! HEX_ONE = HEX HEX .
//! HEX_TWO = HEX_ONE HEX_ONE .
//! HEX_FOUR = HEX_TWO HEX_TWO .
//! ```
