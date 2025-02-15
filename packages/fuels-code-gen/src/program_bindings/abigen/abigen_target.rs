use std::{convert::TryFrom, str::FromStr};

use proc_macro2::Ident;

use crate::{
    error,
    error::{Error, Result},
    utils::Source,
};

use fuel_abi_types::abi::full_program::FullProgramABI;

#[derive(Debug, Clone)]
pub struct AbigenTarget {
    pub name: String,
    pub abi: String,
    pub program_type: ProgramType,
}

pub(crate) struct ParsedAbigenTarget {
    pub name: String,
    pub source: FullProgramABI,
    pub program_type: ProgramType,
}

impl TryFrom<AbigenTarget> for ParsedAbigenTarget {
    type Error = Error;

    fn try_from(value: AbigenTarget) -> Result<Self> {
        Ok(Self {
            name: value.name,
            source: parse_program_abi(&value.abi)?,
            program_type: value.program_type,
        })
    }
}

fn parse_program_abi(abi_source: &str) -> Result<FullProgramABI> {
    let source = Source::parse(abi_source).expect("failed to parse JSON ABI");
    let json_abi_str = source.get().expect("failed to parse JSON ABI from string");
    FullProgramABI::from_json_abi(&json_abi_str).map_err(|e| e.into())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramType {
    Script,
    Contract,
    Predicate,
}

impl FromStr for ProgramType {
    type Err = Error;

    fn from_str(string: &str) -> std::result::Result<Self, Self::Err> {
        let program_type = match string {
            "Script" => ProgramType::Script,
            "Contract" => ProgramType::Contract,
            "Predicate" => ProgramType::Predicate,
            _ => {
                return Err(error!(
                    "'{string}' is not a valid program type. Expected one of: 'Script', 'Contract', 'Predicate'."
                ))
            }
        };

        Ok(program_type)
    }
}

impl TryFrom<Ident> for ProgramType {
    type Error = syn::Error;

    fn try_from(ident: Ident) -> std::result::Result<Self, Self::Error> {
        ident
            .to_string()
            .as_str()
            .parse()
            .map_err(|e| Self::Error::new(ident.span(), e))
    }
}
