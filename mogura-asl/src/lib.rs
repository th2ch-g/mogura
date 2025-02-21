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

fn parse_expr(inputs: &str) -> nom::IResult<&str, Selection> {
    nom::branch::alt((
        parse_all,
        parse_protein,
        parse_sidechain,
        parse_backbone,
        parse_water,
        parse_ion,
    ))
    .parse(inputs)
}

fn parse_all(inputs: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::All, nom::bytes::complete::tag("all")).parse(inputs)
}

fn parse_protein(inputs: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::Protein, nom::bytes::complete::tag("protein")).parse(inputs)
}

fn parse_sidechain(inputs: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::Sidechain, nom::bytes::complete::tag("sidechain"))
        .parse(inputs)
}

fn parse_backbone(inputs: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::Backbone, nom::bytes::complete::tag("backbone")).parse(inputs)
}

fn parse_water(inputs: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::Water, nom::bytes::complete::tag("water")).parse(inputs)
}

fn parse_ion(inputs: &str) -> nom::IResult<&str, Selection> {
    nom::combinator::value(Selection::Ion, nom::bytes::complete::tag("ion")).parse(inputs)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn all() {
        let selection = "all";
        let parsed = parse_selection(selection).unwrap();
        assert_eq!(parsed, Selection::All);
    }

    // #[test]
    // fn resname() {
    //     let selection = "resname ALA";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(parsed, Selection::ResName(vec!["ALA".to_string()]));
    // }
    //
    // #[test]
    // fn resname_multiple() {
    //     let selection = "resname ALA GLU";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(
    //         parsed,
    //         Selection::ResName(vec!["ALA".to_string(), "GLU".to_string()])
    //     );
    // }
    //
    // #[test]
    // fn name() {
    //     let selection = "name CA";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(parsed, Selection::Name(vec!["CA".to_string()]));
    // }
    //
    // #[test]
    // fn name_multiple() {
    //     let selection = "name CA CB";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(
    //         parsed,
    //         Selection::Name(vec!["CA".to_string(), "CB".to_string()])
    //     );
    // }
    //
    // #[test]
    // fn index() {
    //     let selection = "index 10";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(parsed, Selection::Index(vec![10]));
    // }
    //
    // #[test]
    // fn index_multiple() {
    //     let selection = "index 10 20";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(parsed, Selection::Index(vec![10, 20]));
    // }
    //
    // #[test]
    // fn index_to() {
    //     let selection = "index 10 to 20";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(parsed, Selection::Index((10..=20).collect()));
    // }
    //
    // #[test]
    // fn resid() {
    //     let selection = "resid 10";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(parsed, Selection::ResId(vec![10]));
    // }
    //
    // #[test]
    // fn resid_multiple() {
    //     let selection = "resid 10 20";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(parsed, Selection::ResId(vec![10, 20]));
    // }
    //
    // #[test]
    // fn resid_to() {
    //     let selection = "resid 10 to 20";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(parsed, Selection::ResId((10..=20).collect()));
    // }
    //
    // #[test]
    // fn and() {
    //     let selection = "resname ALA and resname GLU";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(
    //         parsed,
    //         Selection::And(vec![
    //             Box::new(Selection::ResName(vec!["ALA".to_string()])),
    //             Box::new(Selection::ResName(vec!["GLU".to_string()]))
    //         ])
    //     );
    // }
    //
    // #[test]
    // fn or() {
    //     let selection = "resname ALA or resname GLU";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(
    //         parsed,
    //         Selection::Or(vec![
    //             Box::new(Selection::ResName(vec!["ALA".to_string()])),
    //             Box::new(Selection::ResName(vec!["GLU".to_string()]))
    //         ])
    //     );
    // }
    //
    // #[test]
    // fn not() {
    //     let selection = "not resname ALA";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(
    //         parsed,
    //         Selection::Not(Box::new(Selection::ResName(vec!["ALA".to_string()])))
    //     );
    // }
    //
    // #[test]
    // fn braket() {
    //     let selection = "(resname ALA GLU) and name CA";
    //     let parsed = parse_selection(selection).unwrap();
    //     assert_eq!(
    //         parsed,
    //         Selection::And(vec![
    //             Box::new(Selection::Braket(Selection::ResName(vec![
    //                 "ALA".to_string(),
    //                 "GLU".to_string()
    //             ]))),
    //             Box::new(Selection::Name(vec!["CA".to_string()]))
    //         ])
    //     );
    // }
    //

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
}
