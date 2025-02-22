use nom::Parser;

#[derive(Clone, PartialEq, Debug)]
pub enum Selection {
    All,
    ResName(Vec<String>),
    ResId(Vec<usize>),
    Name(Vec<String>),
    Index(Vec<usize>),
    Protein,
    Water,
    Ion,
    Backbone,
    Sidechain,
    Not(Box<Selection>),
    And(Vec<Box<Selection>>),
    Or(Vec<Box<Selection>>),
    Braket(Box<Selection>),
}

pub fn parse_selection(selection: &str) -> Result<Selection, String> {
    match nom::combinator::all_consuming(parse_expr).parse(selection) {
        Ok((_, selection)) => Ok(selection),
        Err(e) => Err(e.to_string()),
    }
}

fn parse_expr(input: &str) -> nom::IResult<&str, Selection> {
    let (input, expr) = parse_or.parse(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    Ok((input, expr))
}

fn parse_or(input: &str) -> nom::IResult<&str, Selection> {
    let (input, init) = parse_and.parse(input)?;
    let (input, rest) = nom::multi::many0(nom::sequence::preceded(
        nom::character::complete::space1,
        nom::sequence::preceded(
            nom::bytes::complete::tag("or"),
            nom::sequence::preceded(nom::character::complete::space1, parse_and),
        ),
    ))
    .parse(input)?;
    if rest.is_empty() {
        Ok((input, init))
    } else {
        let mut selections = vec![Box::new(init)];
        for sel in rest {
            selections.push(Box::new(sel));
        }
        Ok((input, Selection::Or(selections)))
    }
}

fn parse_and(input: &str) -> nom::IResult<&str, Selection> {
    let (input, init) = parse_not.parse(input)?;
    let (input, rest) = nom::multi::many0(nom::sequence::preceded(
        nom::character::complete::space1,
        nom::sequence::preceded(
            nom::bytes::complete::tag("and"),
            nom::sequence::preceded(nom::character::complete::space1, parse_not),
        ),
    ))
    .parse(input)?;
    if rest.is_empty() {
        Ok((input, init))
    } else {
        let mut selections = vec![Box::new(init)];
        for sel in rest {
            selections.push(Box::new(sel));
        }
        Ok((input, Selection::And(selections)))
    }
}

fn parse_not(input: &str) -> nom::IResult<&str, Selection> {
    let (input, nots) = nom::multi::many0(nom::sequence::preceded(
        nom::character::complete::space0,
        nom::bytes::complete::tag("not"),
    ))
    .parse(input)?;
    let (input, primary) = parse_primary.parse(input)?;
    let selection = nots
        .into_iter()
        .fold(primary, |acc, _| Selection::Not(Box::new(acc)));
    Ok((input, selection))
}

fn parse_primary(input: &str) -> nom::IResult<&str, Selection> {
    let (input, _) = nom::character::complete::space0.parse(input)?;
    nom::branch::alt((parse_braket, parse_atom)).parse(input)
}

fn parse_braket(input: &str) -> nom::IResult<&str, Selection> {
    let (input, _) = nom::character::complete::char('(').parse(input)?;
    let (input, expr) = parse_expr.parse(input)?;
    let (input, _) = nom::character::complete::char(')').parse(input)?;
    Ok((input, Selection::Braket(Box::new(expr))))
}

fn parse_atom(input: &str) -> nom::IResult<&str, Selection> {
    nom::branch::alt((
        parse_all,
        parse_protein,
        parse_sidechain,
        parse_backbone,
        parse_water,
        parse_ion,
        parse_resname,
        parse_resid,
        parse_index,
        parse_name,
    ))
    .parse(input)
}

fn parse_identifier(input: &str) -> nom::IResult<&str, &str> {
    nom::combinator::verify(nom::character::complete::alphanumeric1, |s: &str| {
        s != "and" && s != "or" && s != "not" && s != "to"
    })
    .parse(input)
}

fn parse_name(input: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::map(
        nom::sequence::preceded(
            nom::bytes::complete::tag("name"),
            nom::sequence::preceded(
                nom::character::complete::space1,
                nom::multi::separated_list1(nom::character::complete::space1, parse_identifier),
            ),
        ),
        |vec: Vec<&str>| Selection::Name(vec.into_iter().map(|s| s.to_string()).collect()),
    )
    .parse(input)
}

fn parse_resname(input: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::map(
        nom::sequence::preceded(
            nom::bytes::complete::tag("resname"),
            nom::sequence::preceded(
                nom::character::complete::space1,
                nom::multi::separated_list1(nom::character::complete::space1, parse_identifier),
            ),
        ),
        |vec: Vec<&str>| Selection::ResName(vec.into_iter().map(|s| s.to_string()).collect()),
    )
    .parse(input)
}

fn parse_resid(input: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::map(
        nom::sequence::preceded(
            nom::bytes::complete::tag("resid"),
            nom::sequence::preceded(nom::character::complete::space1, parse_numbers),
        ),
        |nums| Selection::ResId(nums),
    )
    .parse(input)
}

fn parse_index(input: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::map(
        nom::sequence::preceded(
            nom::bytes::complete::tag("index"),
            nom::sequence::preceded(nom::character::complete::space1, parse_numbers),
        ),
        |nums| Selection::Index(nums),
    )
    .parse(input)
}

fn parse_numbers(input: &str) -> nom::IResult<&str, Vec<usize>> {
    let (input, first) = parse_usize.parse(input)?;
    if let Ok((input, last)) = nom::sequence::preceded(
        nom::sequence::delimited(
            nom::character::complete::space1,
            nom::bytes::complete::tag("to"),
            nom::character::complete::space1,
        ),
        parse_usize,
    )
    .parse(input)
    {
        let range: Vec<usize> = (first..=last).collect();
        Ok((input, range))
    } else {
        let (input, rest) = nom::multi::many0(nom::sequence::preceded(
            nom::character::complete::space1,
            parse_usize,
        ))
        .parse(input)?;
        let mut nums = vec![first];
        nums.extend(rest);
        Ok((input, nums))
    }
}

fn parse_usize(input: &str) -> nom::IResult<&str, usize> {
    nom::combinator::map(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>().unwrap()
    })
    .parse(input)
}

fn parse_all(input: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::All, nom::bytes::complete::tag("all")).parse(input)
}

fn parse_protein(input: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::Protein, nom::bytes::complete::tag("protein")).parse(input)
}

fn parse_sidechain(input: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::Sidechain, nom::bytes::complete::tag("sidechain"))
        .parse(input)
}

fn parse_backbone(input: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::Backbone, nom::bytes::complete::tag("backbone")).parse(input)
}

fn parse_water(input: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::Water, nom::bytes::complete::tag("water")).parse(input)
}

fn parse_ion(input: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::Ion, nom::bytes::complete::tag("ion")).parse(input)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn all() {
        let selection = "all";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::All);

        let selection = " all ";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::All);
    }

    #[test]
    fn resname() {
        let selection = "resname ALA";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::ResName(vec!["ALA".to_string()]));
    }

    #[test]
    fn resname_multiple() {
        let selection = "resname ALA GLU";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(
            parsed,
            Selection::ResName(vec!["ALA".to_string(), "GLU".to_string()])
        );
    }

    #[test]
    fn name() {
        let selection = "name CA";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::Name(vec!["CA".to_string()]));
    }

    #[test]
    fn name_multiple() {
        let selection = "name CA CB";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(
            parsed,
            Selection::Name(vec!["CA".to_string(), "CB".to_string()])
        );

        let selection = " name  CA  CB ";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(
            parsed,
            Selection::Name(vec!["CA".to_string(), "CB".to_string()])
        );
    }

    #[test]
    fn index() {
        let selection = "index 10";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::Index(vec![10]));
    }

    #[test]
    fn index_multiple() {
        let selection = "index 10 20";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::Index(vec![10, 20]));
    }

    #[test]
    fn index_to() {
        let selection = "index 10 to 20";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::Index((10..=20).collect()));
    }

    #[test]
    fn resid() {
        let selection = "resid 10";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::ResId(vec![10]));

        let selection = "resid resname";
        let parsed = parse_selection(selection);
        assert!(parsed.is_err());
    }

    #[test]
    fn resid_multiple() {
        let selection = "resid 10 20";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::ResId(vec![10, 20]));
    }

    #[test]
    fn resid_to() {
        let selection = "resid 10 to 20";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::ResId((10..=20).collect()));

        let selection = "resid to 20";
        let parsed = parse_selection(selection);
        assert!(parsed.is_err());
    }

    #[test]
    fn and() {
        let selection = "resname ALA and resname GLU";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(
            parsed,
            Selection::And(vec![
                Box::new(Selection::ResName(vec!["ALA".to_string()])),
                Box::new(Selection::ResName(vec!["GLU".to_string()]))
            ])
        );
    }

    #[test]
    fn or() {
        let selection = "resname ALA or resname GLU";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(
            parsed,
            Selection::Or(vec![
                Box::new(Selection::ResName(vec!["ALA".to_string()])),
                Box::new(Selection::ResName(vec!["GLU".to_string()]))
            ])
        );

        let selection = "  resname   ALA  or  resname GLU ";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(
            parsed,
            Selection::Or(vec![
                Box::new(Selection::ResName(vec!["ALA".to_string()])),
                Box::new(Selection::ResName(vec!["GLU".to_string()]))
            ])
        );
    }

    #[test]
    fn not() {
        let selection = "not resname ALA";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(
            parsed,
            Selection::Not(Box::new(Selection::ResName(vec!["ALA".to_string()])))
        );
    }

    #[test]
    fn braket() {
        let selection = "(resname ALA GLU) and name CA";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(
            parsed,
            Selection::And(vec![
                Box::new(Selection::Braket(Box::new(Selection::ResName(vec![
                    "ALA".to_string(),
                    "GLU".to_string()
                ])))),
                Box::new(Selection::Name(vec!["CA".to_string()]))
            ])
        );

        let selection = "(index 10 to 20) or protein and (resname ALA)";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(
            parsed,
            Selection::Or(vec![
                Box::new(Selection::Braket(Box::new(Selection::Index(
                    (10..=20).collect()
                )))),
                Box::new(Selection::And(vec![
                    Box::new(Selection::Protein),
                    Box::new(Selection::Braket(Box::new(Selection::ResName(vec![
                        "ALA".to_string()
                    ])),))
                ]))
            ])
        );
    }

    #[test]
    fn protein() {
        let selection = "protein";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::Protein);
    }

    #[test]
    fn water() {
        let selection = "water";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::Water);
    }

    #[test]
    fn ion() {
        let selection = "ion";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::Ion);
    }

    #[test]
    fn backbone() {
        let selection = "backbone";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::Backbone);
    }

    #[test]
    fn sidechain() {
        let selection = "sidechain";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::Sidechain);
    }

    #[test]
    fn sample_eval() {
        impl Selection {
            fn eval(&self, state: &State) -> bool {
                match self {
                    Selection::All => true,
                    Selection::ResName(names) => names.iter().any(|name| name == &state.resname),
                    Selection::ResId(ids) => ids.iter().any(|id| id == &state.resid),
                    Selection::Name(names) => names.iter().any(|name| name == &state.name),
                    Selection::Index(indices) => indices.iter().any(|index| index == &state.index),
                    Selection::Not(selection) => !selection.eval(state),
                    Selection::And(selections) => selections.iter().all(|s| s.eval(state)),
                    Selection::Or(selections) => selections.iter().any(|s| s.eval(state)),
                    Selection::Braket(selection) => selection.eval(state),
                    _ => false,
                    // Selection::Protein => state.is_protein(),
                    // Selection::Water => state.is_water(),
                    // Selection::Ion => state.is_ion(),
                    // Selection::Backbone => state.is_backbone(),
                    // Selection::Sidechain => state.is_sidechain(),
                }
            }
        }

        struct State {
            index: usize,
            resname: String,
            resid: usize,
            name: String,
        }

        let expr = "(index 10 to 20) or resname ALA";
        let selection = parse_selection(expr).unwrap();

        let state1 = State {
            index: 15,
            resname: "ALA".to_string(),
            resid: 10,
            name: "CA".to_string(),
        };

        let state2 = State {
            index: 25,
            resname: "GLU".to_string(),
            resid: 20,
            name: "CA".to_string(),
        };

        assert!(selection.eval(&state1));
        assert!(!selection.eval(&state2));
    }
}
