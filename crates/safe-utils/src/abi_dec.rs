use alloy::{
    dyn_abi::{DynSolValue, JsonAbiExt},
    hex,
    json_abi::Function,
    primitives::{I256, Sign, U256, map::HashMap},
};
use serde::Deserialize;
use std::fmt;

use crate::Result;

pub struct CalldataDecoder {
    pub calldata: String,
}

impl CalldataDecoder {
    pub fn new(calldata: String) -> Self {
        Self { calldata }
    }
}

#[derive(Debug, Clone)]
pub struct CalldataDecoded {
    pub options: Vec<CalldataDecodedUnit>,
}

#[derive(Debug, Clone)]
pub struct CalldataDecodedUnit {
    pub signature: String,
    pub arguments: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct OpenchainResponse {
    ok: bool,
    result: Option<OpenchainResult>,
}

#[derive(Deserialize, Debug)]
struct OpenchainResult {
    function: HashMap<String, Vec<OpenchainAnswer>>,
}

#[derive(Deserialize, Debug)]
struct OpenchainAnswer {
    name: String, // signature
}

impl CalldataDecoder {
    pub fn try_decode(&self) -> Result<CalldataDecoded> {
        let unit = |signature: String| -> Result<CalldataDecodedUnit> {
            let decoded_input = {
                let func = Function::parse(signature.as_str())?;
                let calldata = hex::decode(self.calldata.clone())?;
                let calldata = &calldata.as_slice()[4..];
                func.abi_decode_input(calldata, true)?
            };
            Ok(CalldataDecodedUnit {
                signature,
                arguments: decoded_input
                    .iter()
                    .map(|i| format!("{}", DynValueDisplay::new(i, false)))
                    .collect(),
            })
        };
        let signatures = lookup_sig(self.calldata.clone())?;
        let signature_units = signatures.into_iter().flat_map(unit).collect::<Vec<_>>();
        Ok(CalldataDecoded { options: signature_units })
    }
}

fn lookup_sig(calldata: String) -> Result<Vec<String>> {
    let four_bytes_string = if calldata.starts_with("0x") {
        &calldata[..10]
    } else {
        &("0x".to_string() + &calldata[..8])
    };
    const SELECTOR_LOOKUP_URL: &str = "https://api.openchain.xyz/signature-database/v1/lookup";
    let url = format!("{}?function={}&filter=false", SELECTOR_LOOKUP_URL, four_bytes_string);
    let response = reqwest::blocking::get(url)?;
    let response: OpenchainResponse = response.json()?;

    if !response.ok {
        return Err("not okay resp from openchain.xyz".into());
    }

    if let Some(result) = response.result {
        return Ok(result
            .function
            .get(four_bytes_string)
            .ok_or("key not found")?
            .iter()
            .map(|f| f.name.clone())
            .collect::<Vec<_>>());
    }

    Ok(vec![])
}

#[cfg(test)]
mod calldata_decode {
    use super::CalldataDecoder;

    #[test]
    pub fn test_decode() {
        let calldata = "0xa9059cbb000000000000000000000000e78388b4ce79068e89bf8aa7f218ef6b9ab0e9d00000000000000000000000000000000000000000000000000174b37380cea000".to_string();
        let decoder = CalldataDecoder { calldata };
        let decoded = decoder.try_decode().unwrap();
        println!("{:#?}", decoded);
    }
}

struct DynValueDisplay<'a> {
    value: &'a DynSolValue,
    formatter: DynValueFormatter,
}

impl<'a> DynValueDisplay<'a> {
    /// Creates a new [`Display`](fmt::Display) wrapper for the given value.
    #[inline]
    fn new(value: &'a DynSolValue, raw: bool) -> Self {
        Self { value, formatter: DynValueFormatter { raw } }
    }
}

impl fmt::Display for DynValueDisplay<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.formatter.value(self.value, f)
    }
}

#[inline]
pub fn to_exp_notation(value: U256, precision: usize, trim_end_zeros: bool, sign: Sign) -> String {
    let stringified = value.to_string();
    let exponent = stringified.len() - 1;
    let mut mantissa = stringified.chars().take(precision).collect::<String>();

    // optionally remove trailing zeros
    if trim_end_zeros {
        mantissa = mantissa.trim_end_matches('0').to_string();
    }

    // Place a decimal point only if needed
    // e.g. 1234 -> 1.234e3 (needed)
    //      5 -> 5 (not needed)
    if mantissa.len() > 1 {
        mantissa.insert(1, '.');
    }

    format!("{sign}{mantissa}e{exponent}")
}

pub fn format_uint_exp(num: U256) -> String {
    if num < U256::from(10_000) {
        return num.to_string()
    }

    let exp = to_exp_notation(num, 4, true, Sign::Positive);
    format!("{num} {}", format!("[{exp}]"))
}

pub fn format_int_exp(num: I256) -> String {
    let (sign, abs) = num.into_sign_and_abs();
    if abs < U256::from(10_000) {
        return format!("{sign}{abs}");
    }

    let exp = to_exp_notation(abs, 4, true, sign);
    format!("{sign}{abs} {}", format!("[{exp}]"))
}

struct DynValueFormatter {
    raw: bool,
}

impl DynValueFormatter {
    fn value(&self, value: &DynSolValue, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match value {
            DynSolValue::Address(inner) => write!(f, "{inner}"),
            DynSolValue::Function(inner) => write!(f, "{inner}"),
            DynSolValue::Bytes(inner) => f.write_str(&hex::encode_prefixed(inner)),
            DynSolValue::FixedBytes(word, size) => {
                f.write_str(&hex::encode_prefixed(&word[..*size]))
            }
            DynSolValue::Uint(inner, _) => {
                if self.raw {
                    write!(f, "{inner}")
                } else {
                    f.write_str(&format_uint_exp(*inner))
                }
            }
            DynSolValue::Int(inner, _) => {
                if self.raw {
                    write!(f, "{inner}")
                } else {
                    f.write_str(&format_int_exp(*inner))
                }
            }
            DynSolValue::Array(values) | DynSolValue::FixedArray(values) => {
                f.write_str("[")?;
                self.list(values, f)?;
                f.write_str("]")
            }
            DynSolValue::Tuple(values) => self.tuple(values, f),
            DynSolValue::String(inner) => {
                if self.raw {
                    write!(f, "{}", inner.escape_debug())
                } else {
                    write!(f, "{inner:?}") // escape strings
                }
            }
            DynSolValue::Bool(inner) => write!(f, "{inner}"),
            DynSolValue::CustomStruct { name, prop_names, tuple } => {
                if self.raw {
                    return self.tuple(tuple, f);
                }

                f.write_str(name)?;

                if prop_names.len() == tuple.len() {
                    f.write_str("({ ")?;

                    for (i, (prop_name, value)) in std::iter::zip(prop_names, tuple).enumerate() {
                        if i > 0 {
                            f.write_str(", ")?;
                        }
                        f.write_str(prop_name)?;
                        f.write_str(": ")?;
                        self.value(value, f)?;
                    }

                    f.write_str(" })")
                } else {
                    self.tuple(tuple, f)
                }
            }
        }
    }

    /// Recursively formats a comma-separated list of [`DynSolValue`]s.
    fn list(&self, values: &[DynSolValue], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, value) in values.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            self.value(value, f)?;
        }
        Ok(())
    }

    /// Formats the given values as a tuple.
    fn tuple(&self, values: &[DynSolValue], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        self.list(values, f)?;
        f.write_str(")")
    }
}
