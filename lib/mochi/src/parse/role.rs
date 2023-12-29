use std::str::FromStr;
use pest::Parser;

use crate::{Role, MochiParser, Rule, util, error::{UnexpectedTokenError, UnexpectedEnvironentError, PropertyTypeError, ProgrammerError, UnknownPropertyError}, discord};

impl FromStr for Role {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = MochiParser::parse(Rule::environment, s)?;

        let mut environment_token = util::unwrap_rule_as_token(parser.into_iter().next(), Rule::environment)?.into_inner();

        let env_type = util::unwrap_rule_as_token(environment_token.next(), Rule::env_type)?;
        if env_type.as_str() != "role" {
            return Err(Box::new(UnexpectedTokenError {
                expected: "role".to_string(),
                found: env_type.as_str().to_string()
            }));
        }

        /* Environment Name */
        let env_name = util::unwrap_rule_as_token(environment_token.next(), Rule::string)?;
        let trimmed_name = util::unwrap_rule_as_token(env_name.into_inner().next(), Rule::text)?;

        /* Environment Properties */
        let mut role = Role {
            name: trimmed_name.as_str().to_string(),
            color: 0,
            separate: false,
            mentionable: false,
            permissions: 0,
        };

        /* Environment Properties */
        for token in environment_token {
            if token.as_rule() != Rule::property {
                return Err(Box::new(UnexpectedEnvironentError {
                    context: "role".to_string()
                }));
            }

            let mut property = token.into_inner();
            let property_name = util::unwrap_rule_as_token(property.next(), Rule::identifier)?;
            let property_value = property.next();

            match property_name.as_str() {
                "color" => {
                    let color_value = util::unwrap_rule_as_token_or_err(property_value.clone(), Rule::color, PropertyTypeError{
                        property: "color".to_string(),
                        environment: "role".to_string(),
                        expected: "hex-color (#abcdef)".to_string(),
                        found: property_value.map_or("None".to_string(), |r| r.as_str().to_string() ),
                    })?;
                    let color_value = color_value.as_str().trim_start_matches("#");
                    let color_value = u32::from_str_radix(color_value, 16)?;
                    role.color = color_value;
                },
                "separate" => {
                    let separate_value = util::unwrap_rule_as_token_or_err(property_value.clone(), Rule::bool, PropertyTypeError{
                        property: "separate".to_string(),
                        environment: "role".to_string(),
                        expected: "bool".to_string(),
                        found: property_value.map_or("None".to_string(), |r| r.as_str().to_string() ),
                    })?;
                    if separate_value.as_str() == "true" {
                        role.separate = true;
                    }
                },
                "mentionable" => {
                    let mentionable_value = util::unwrap_rule_as_token_or_err(property_value.clone(), Rule::bool, PropertyTypeError{
                        property: "mentionable".to_string(),
                        environment: "role".to_string(),
                        expected: "bool".to_string(),
                        found: property_value.map_or("None".to_string(), |r| r.as_str().to_string() ),
                    })?;
                    if mentionable_value.as_str() == "true" {
                        role.mentionable = true;
                    }
                },
                "permissions" => {
                    if property_value.is_none() {
                        return Err(Box::new(ProgrammerError));
                    }
                    let unwrapped_property = property_value.unwrap();
                    let unwrapped_rule = unwrapped_property.as_rule();

                    if unwrapped_rule == Rule::integer {
                        let number_str = unwrapped_property.as_str();
                        let number = number_str.parse::<u64>()?;
                        // A bit 面倒くさい, but it's there for validation.
                        let permission_vec = discord::separate_permissions(number);
                        role.permissions = discord::parse_permissions_from_perm_vec(permission_vec);
                        continue;
                    }
                    
                    if unwrapped_rule == Rule::array {
                        let mut permissions = Vec::new();
                        for permission in unwrapped_property.clone().into_inner() {
                            if permission.as_rule() != Rule::identifier {
                                return Err(Box::new(PropertyTypeError{
                                    property: "permissions".to_string(),
                                    environment: "role".to_string(),
                                    expected: "identifier-array".to_string(),
                                    found: permission.as_str().to_string(),
                                }));
                            }
                            let permission = permission.as_str();
                            permissions.push(permission);
                        }
                        role.permissions = discord::parse_permissions_from_vec(permissions);
                        continue;
                    }

                    return Err(Box::new(PropertyTypeError {
                        property: "permissions".to_string(),
                        environment: "role".to_string(),
                        expected: "number or identifier-array".to_string(),
                        found: format!("{:?}", unwrapped_rule),
                    }));
                },
                _ => return Err(Box::new(UnknownPropertyError {
                    property: property_name.as_str().to_string(),
                    environment: "role".to_string(),
                })),
            }
        }

        Ok(role)
    }
}