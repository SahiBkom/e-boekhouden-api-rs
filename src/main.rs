use ::e_boekhouden_api_rs::xml::*;
use ::e_boekhouden_api_rs::*;
use failure::Error;

fn main() -> Result<(), Error> {
    let mut api = Api::new_from_env()?;
    let filter = Some(CRelatieFilter {
        trefwoord: None,
        code: None,
        id: 13786118,
    });

    let relaties = api.get_relaties(filter)?;
    println!("{:?}", relaties);

    let mut relaties = relaties
        .get_relaties_result
        .unwrap()
        .relaties
        .unwrap()
        .c_relatie;

    if relaties.len() == 1 {
        if let Some(relatie) = relaties.get_mut(0) {
            relatie.notitie = Some("test 41".to_string());
            let r = api.update_relaties(relatie.clone());
            println!("Result {:?}", r);
        }
    }

    Ok(())
}
