use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAdministraties {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAdministratiesResponse {
    #[serde(rename = "GetAdministratiesResult")]
    pub get_administraties_result: Option<cResultGetAdministraties>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultGetAdministraties {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Administraties")]
    pub administraties: Option<ArrayOfCAdministratie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cError {
    #[serde(rename = "LastErrorCode")]
    pub last_error_code: Option<String>,
    #[serde(rename = "LastErrorDescription")]
    pub last_error_description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayOfCAdministratie {
    #[serde(rename = "cAdministratie")]
    pub c_administratie: Vec<cAdministratie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cAdministratie {
    #[serde(rename = "Bedrijf")]
    pub bedrijf: Option<String>,
    #[serde(rename = "Plaats")]
    pub plaats: Option<String>,
    #[serde(rename = "Guid")]
    pub guid: Option<String>,
    #[serde(rename = "StartBoekjaar")]
    pub start_boekjaar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSaldo {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<cSaldoFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cSaldoFilter {
    #[serde(rename = "GbCode")]
    pub gb_code: Option<String>,
    #[serde(rename = "KostenPlaatsId")]
    pub kosten_plaats_id: i64,
    #[serde(rename = "DatumVan")]
    pub datum_van: String,
    #[serde(rename = "DatumTot")]
    pub datum_tot: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSaldoResponse {
    #[serde(rename = "GetSaldoResult")]
    pub get_saldo_result: Option<cResultGetSaldo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultGetSaldo {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Saldo")]
    pub saldo: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddFactuur {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oFact")]
    pub o_fact: Option<cFactuur>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cFactuur {
    #[serde(rename = "Factuurnummer")]
    pub factuurnummer: Option<String>,
    #[serde(rename = "Relatiecode")]
    pub relatiecode: Option<String>,
    #[serde(rename = "Datum")]
    pub datum: String,
    #[serde(rename = "Betalingstermijn")]
    pub betalingstermijn: i64,
    #[serde(rename = "Factuursjabloon")]
    pub factuursjabloon: Option<String>,
    #[serde(rename = "PerEmailVerzenden")]
    pub per_email_verzenden: bool,
    #[serde(rename = "EmailOnderwerp")]
    pub email_onderwerp: Option<String>,
    #[serde(rename = "EmailBericht")]
    pub email_bericht: Option<String>,
    #[serde(rename = "EmailVanAdres")]
    pub email_van_adres: Option<String>,
    #[serde(rename = "EmailVanNaam")]
    pub email_van_naam: Option<String>,
    #[serde(rename = "AutomatischeIncasso")]
    pub automatische_incasso: bool,
    #[serde(rename = "IncassoIBAN")]
    pub incasso_iban: Option<String>,
    #[serde(rename = "IncassoMachtigingSoort")]
    pub incasso_machtiging_soort: Option<String>,
    #[serde(rename = "IncassoMachtigingID")]
    pub incasso_machtiging_id: Option<String>,
    #[serde(rename = "IncassoMachtigingDatumOndertekening")]
    pub incasso_machtiging_datum_ondertekening: String,
    #[serde(rename = "IncassoMachtigingFirst")]
    pub incasso_machtiging_first: bool,
    #[serde(rename = "IncassoRekeningNummer")]
    pub incasso_rekening_nummer: Option<String>,
    #[serde(rename = "IncassoTnv")]
    pub incasso_tnv: Option<String>,
    #[serde(rename = "IncassoPlaats")]
    pub incasso_plaats: Option<String>,
    #[serde(rename = "IncassoOmschrijvingRegel1")]
    pub incasso_omschrijving_regel_1: Option<String>,
    #[serde(rename = "IncassoOmschrijvingRegel2")]
    pub incasso_omschrijving_regel_2: Option<String>,
    #[serde(rename = "IncassoOmschrijvingRegel3")]
    pub incasso_omschrijving_regel_3: Option<String>,
    #[serde(rename = "InBoekhoudingPlaatsen")]
    pub in_boekhouding_plaatsen: bool,
    #[serde(rename = "BoekhoudmutatieOmschrijving")]
    pub boekhoudmutatie_omschrijving: Option<String>,
    #[serde(rename = "Regels")]
    pub regels: Option<ArrayOfCFactuurRegel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayOfCFactuurRegel {
    #[serde(rename = "cFactuurRegel")]
    pub c_factuur_regel: Vec<cFactuurRegel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cFactuurRegel {
    #[serde(rename = "Aantal")]
    pub aantal: f64,
    #[serde(rename = "Eenheid")]
    pub eenheid: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Omschrijving")]
    pub omschrijving: Option<String>,
    #[serde(rename = "PrijsPerEenheid")]
    pub prijs_per_eenheid: f64,
    #[serde(rename = "BTWCode")]
    pub btw_code: Option<String>,
    #[serde(rename = "TegenrekeningCode")]
    pub tegenrekening_code: Option<String>,
    #[serde(rename = "KostenplaatsID")]
    pub kostenplaats_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddFactuurResponse {
    #[serde(rename = "AddFactuurResult")]
    pub add_factuur_result: Option<cResultAddFactuur>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultAddFactuur {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Factuurnummer")]
    pub factuurnummer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFacturen {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<cFactuurFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cFactuurFilter {
    #[serde(rename = "Factuurnummer")]
    pub factuurnummer: Option<String>,
    #[serde(rename = "Relatiecode")]
    pub relatiecode: Option<String>,
    #[serde(rename = "DatumVan")]
    pub datum_van: String,
    #[serde(rename = "DatumTm")]
    pub datum_tm: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFacturenResponse {
    #[serde(rename = "GetFacturenResult")]
    pub get_facturen_result: Option<cResultGetFacturen>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultGetFacturen {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Facturen")]
    pub facturen: Option<ArrayOfCFactuurList>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayOfCFactuurList {
    #[serde(rename = "cFactuurList")]
    pub c_factuur_list: Vec<cFactuurList>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cFactuurList {
    #[serde(rename = "Factuurnummer")]
    pub factuurnummer: Option<String>,
    #[serde(rename = "Datum")]
    pub datum: String,
    #[serde(rename = "Betalingstermijn")]
    pub betalingstermijn: i64,
    #[serde(rename = "TotaalExclBTW")]
    pub totaal_excl_btw: f64,
    #[serde(rename = "TotaalBTW")]
    pub totaal_btw: f64,
    #[serde(rename = "TotaalInclBTW")]
    pub totaal_incl_btw: f64,
    #[serde(rename = "TotaalOpenstaand")]
    pub totaal_openstaand: f64,
    #[serde(rename = "URLPDFBestand")]
    pub urlpdf_bestand: Option<String>,
    #[serde(rename = "Regels")]
    pub regels: Option<ArrayOfCFactuurRegel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddMutatie {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oMut")]
    pub o_mut: Option<cMutatie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cMutatie {
    #[serde(rename = "MutatieNr")]
    pub mutatie_nr: i64,
    #[serde(rename = "Soort")]
    pub soort: String,
    #[serde(rename = "Datum")]
    pub datum: String,
    #[serde(rename = "Rekening")]
    pub rekening: Option<String>,
    #[serde(rename = "RelatieCode")]
    pub relatie_code: Option<String>,
    #[serde(rename = "Factuurnummer")]
    pub factuurnummer: Option<String>,
    #[serde(rename = "Boekstuk")]
    pub boekstuk: Option<String>,
    #[serde(rename = "Omschrijving")]
    pub omschrijving: Option<String>,
    #[serde(rename = "Betalingstermijn")]
    pub betalingstermijn: Option<String>,
    #[serde(rename = "Betalingskenmerk")]
    pub betalingskenmerk: Option<String>,
    #[serde(rename = "InExBTW")]
    pub in_ex_btw: Option<String>,
    #[serde(rename = "MutatieRegels")]
    pub mutatie_regels: Option<ArrayOfCMutatieRegel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayOfCMutatieRegel {
    #[serde(rename = "cMutatieRegel")]
    pub c_mutatie_regel: Vec<cMutatieRegel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cMutatieRegel {
    #[serde(rename = "BedragInvoer")]
    pub bedrag_invoer: f64,
    #[serde(rename = "BedragExclBTW")]
    pub bedrag_excl_btw: f64,
    #[serde(rename = "BedragBTW")]
    pub bedrag_btw: f64,
    #[serde(rename = "BedragInclBTW")]
    pub bedrag_incl_btw: f64,
    #[serde(rename = "BTWCode")]
    pub btw_code: Option<String>,
    #[serde(rename = "BTWPercentage")]
    pub btw_percentage: f64,
    #[serde(rename = "TegenrekeningCode")]
    pub tegenrekening_code: Option<String>,
    #[serde(rename = "KostenplaatsID")]
    pub kostenplaats_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultAddMutatie {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Mutatienummer")]
    pub mutatienummer: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddMutatieResponse {
    #[serde(rename = "AddMutatieResult")]
    pub add_mutatie_result: Option<cResultAddMutatie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMutaties {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<cMutatieFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cMutatieFilter {
    #[serde(rename = "MutatieNr")]
    pub mutatie_nr: i64,
    #[serde(rename = "MutatieNrVan")]
    pub mutatie_nr_van: Option<i64>,
    #[serde(rename = "MutatieNrTm")]
    pub mutatie_nr_tm: Option<i64>,
    #[serde(rename = "Factuurnummer")]
    pub factuurnummer: Option<String>,
    #[serde(rename = "DatumVan")]
    pub datum_van: String,
    #[serde(rename = "DatumTm")]
    pub datum_tm: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultGetMutaties {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Mutaties")]
    pub mutaties: Option<ArrayOfCMutatieList>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayOfCMutatieList {
    #[serde(rename = "cMutatieList")]
    pub c_mutatie_list: Vec<cMutatieList>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cMutatieList {
    #[serde(rename = "MutatieNr")]
    pub mutatie_nr: i64,
    #[serde(rename = "Soort")]
    pub soort: String,
    #[serde(rename = "Datum")]
    pub datum: String,
    #[serde(rename = "Rekening")]
    pub rekening: Option<String>,
    #[serde(rename = "RelatieCode")]
    pub relatie_code: Option<String>,
    #[serde(rename = "Factuurnummer")]
    pub factuurnummer: Option<String>,
    #[serde(rename = "Boekstuk")]
    pub boekstuk: Option<String>,
    #[serde(rename = "Omschrijving")]
    pub omschrijving: Option<String>,
    #[serde(rename = "Betalingstermijn")]
    pub betalingstermijn: Option<String>,
    #[serde(rename = "InExBTW")]
    pub in_ex_btw: Option<String>,
    #[serde(rename = "MutatieRegels")]
    pub mutatie_regels: Option<ArrayOfCMutatieListRegel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayOfCMutatieListRegel {
    #[serde(rename = "cMutatieListRegel")]
    pub c_mutatie_list_regel: Vec<cMutatieListRegel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cMutatieListRegel {
    #[serde(rename = "BedragInvoer")]
    pub bedrag_invoer: f64,
    #[serde(rename = "BedragExclBTW")]
    pub bedrag_excl_btw: f64,
    #[serde(rename = "BedragBTW")]
    pub bedrag_btw: f64,
    #[serde(rename = "BedragInclBTW")]
    pub bedrag_incl_btw: f64,
    #[serde(rename = "BTWCode")]
    pub btw_code: Option<String>,
    #[serde(rename = "BTWPercentage")]
    pub btw_percentage: f64,
    #[serde(rename = "Factuurnummer")]
    pub factuurnummer: Option<String>,
    #[serde(rename = "TegenrekeningCode")]
    pub tegenrekening_code: Option<String>,
    #[serde(rename = "KostenplaatsID")]
    pub kostenplaats_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMutatiesResponse {
    #[serde(rename = "GetMutatiesResult")]
    pub get_mutaties_result: Option<cResultGetMutaties>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddGrootboekrekening {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oGb")]
    pub o_gb: Option<cGrootboekrekening>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cGrootboekrekening {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Omschrijving")]
    pub omschrijving: Option<String>,
    #[serde(rename = "Categorie")]
    pub categorie: Option<String>,
    #[serde(rename = "Groep")]
    pub groep: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultAddGrootboekrekening {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Gb_ID")]
    pub gb_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddGrootboekrekeningResponse {
    #[serde(rename = "AddGrootboekrekeningResult")]
    pub add_grootboekrekening_result: Option<cResultAddGrootboekrekening>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGrootboekrekening {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oGb")]
    pub o_gb: Option<cGrootboekrekening>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGrootboekrekeningResponse {
    #[serde(rename = "UpdateGrootboekrekeningResult")]
    pub update_grootboekrekening_result: Option<cError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGrootboekrekeningen {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<cGrootboekrekeningFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cGrootboekrekeningFilter {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Categorie")]
    pub categorie: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultGetGrootboekrekeningen {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Rekeningen")]
    pub rekeningen: Option<ArrayOfCGrootboekrekening>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayOfCGrootboekrekening {
    #[serde(rename = "cGrootboekrekening")]
    pub c_grootboekrekening: Vec<cGrootboekrekening>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGrootboekrekeningenResponse {
    #[serde(rename = "GetGrootboekrekeningenResult")]
    pub get_grootboekrekeningen_result: Option<cResultGetGrootboekrekeningen>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddRelatie {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oRel")]
    pub o_rel: Option<cRelatie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cRelatie {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "AddDatum")]
    pub add_datum: String,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Bedrijf")]
    pub bedrijf: Option<String>,
    #[serde(rename = "Contactpersoon")]
    pub contactpersoon: Option<String>,
    #[serde(rename = "Geslacht")]
    pub geslacht: Option<String>,
    #[serde(rename = "Adres")]
    pub adres: Option<String>,
    #[serde(rename = "Postcode")]
    pub postcode: Option<String>,
    #[serde(rename = "Plaats")]
    pub plaats: Option<String>,
    #[serde(rename = "Land")]
    pub land: Option<String>,
    #[serde(rename = "Adres2")]
    pub adres_2: Option<String>,
    #[serde(rename = "Postcode2")]
    pub postcode_2: Option<String>,
    #[serde(rename = "Plaats2")]
    pub plaats_2: Option<String>,
    #[serde(rename = "Land2")]
    pub land_2: Option<String>,
    #[serde(rename = "Telefoon")]
    pub telefoon: Option<String>,
    #[serde(rename = "GSM")]
    pub gsm: Option<String>,
    #[serde(rename = "FAX")]
    pub fax: Option<String>,
    #[serde(rename = "Email")]
    pub email: Option<String>,
    #[serde(rename = "Site")]
    pub site: Option<String>,
    #[serde(rename = "Notitie")]
    pub notitie: Option<String>,
    #[serde(rename = "Bankrekening")]
    pub bankrekening: Option<String>,
    #[serde(rename = "Girorekening")]
    pub girorekening: Option<String>,
    #[serde(rename = "BTWNummer")]
    pub btw_nummer: Option<String>,
    #[serde(rename = "Aanhef")]
    pub aanhef: Option<String>,
    #[serde(rename = "IBAN")]
    pub iban: Option<String>,
    #[serde(rename = "BIC")]
    pub bic: Option<String>,
    #[serde(rename = "BP")]
    pub bp: Option<String>,
    #[serde(rename = "Def1")]
    pub def_1: Option<String>,
    #[serde(rename = "Def2")]
    pub def_2: Option<String>,
    #[serde(rename = "Def3")]
    pub def_3: Option<String>,
    #[serde(rename = "Def4")]
    pub def_4: Option<String>,
    #[serde(rename = "Def5")]
    pub def_5: Option<String>,
    #[serde(rename = "Def6")]
    pub def_6: Option<String>,
    #[serde(rename = "Def7")]
    pub def_7: Option<String>,
    #[serde(rename = "Def8")]
    pub def_8: Option<String>,
    #[serde(rename = "Def9")]
    pub def_9: Option<String>,
    #[serde(rename = "Def10")]
    pub def_10: Option<String>,
    #[serde(rename = "LA")]
    pub la: Option<String>,
    #[serde(rename = "Gb_ID")]
    pub gb_id: i64,
    #[serde(rename = "GeenEmail")]
    pub geen_email: i64,
    #[serde(rename = "NieuwsbriefgroepenCount")]
    pub nieuwsbriefgroepen_count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultAddRelatie {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Rel_ID")]
    pub rel_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddRelatieResponse {
    #[serde(rename = "AddRelatieResult")]
    pub add_relatie_result: Option<cResultAddRelatie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRelatie {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oRel")]
    pub o_rel: Option<cRelatie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRelatieResponse {
    #[serde(rename = "UpdateRelatieResult")]
    pub update_relatie_result: Option<cError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRelaties {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<cRelatieFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cRelatieFilter {
    #[serde(rename = "Trefwoord")]
    pub trefwoord: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "ID")]
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultGetRelaties {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Relaties")]
    pub relaties: Option<ArrayOfCRelatie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayOfCRelatie {
    #[serde(rename = "cRelatie")]
    pub c_relatie: Vec<cRelatie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRelatiesResponse {
    #[serde(rename = "GetRelatiesResult")]
    pub get_relaties_result: Option<cResultGetRelaties>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOpenPosten {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "OpSoort")]
    pub op_soort: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultGetOpenPosten {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Openposten")]
    pub openposten: Option<ArrayOfCOpenPost>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayOfCOpenPost {
    #[serde(rename = "cOpenPost")]
    pub c_open_post: Vec<cOpenPost>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cOpenPost {
    #[serde(rename = "MutDatum")]
    pub mut_datum: String,
    #[serde(rename = "MutFactuur")]
    pub mut_factuur: Option<String>,
    #[serde(rename = "RelCode")]
    pub rel_code: Option<String>,
    #[serde(rename = "RelBedrijf")]
    pub rel_bedrijf: Option<String>,
    #[serde(rename = "Bedrag")]
    pub bedrag: f64,
    #[serde(rename = "Voldaan")]
    pub voldaan: f64,
    #[serde(rename = "Openstaand")]
    pub openstaand: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOpenPostenResponse {
    #[serde(rename = "GetOpenPostenResult")]
    pub get_open_posten_result: Option<cResultGetOpenPosten>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenSession {
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "SecurityCode1")]
    pub security_code_1: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultOpenSession {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenSessionResponse {
    #[serde(rename = "OpenSessionResult")]
    pub open_session_result: Option<cResultOpenSession>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseSession {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseSessionResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoLogin {
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultAutoLogin {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Token")]
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoLoginResponse {
    #[serde(rename = "AutoLoginResult")]
    pub auto_login_result: Option<cResultAutoLogin>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetKostenplaatsen {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<cKostenplaatsFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cKostenplaatsFilter {
    #[serde(rename = "KostenplaatsID")]
    pub kostenplaats_id: Option<i64>,
    #[serde(rename = "KostenplaatsParentID")]
    pub kostenplaats_parent_id: Option<i64>,
    #[serde(rename = "Omschrijving")]
    pub omschrijving: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultGetKostenplaatsen {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Kostenplaatsen")]
    pub kostenplaatsen: Option<ArrayOfCKostenplaats>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayOfCKostenplaats {
    #[serde(rename = "cKostenplaats")]
    pub c_kostenplaats: Vec<cKostenplaats>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cKostenplaats {
    #[serde(rename = "KostenplaatsId")]
    pub kostenplaats_id: i64,
    #[serde(rename = "Omschrijving")]
    pub omschrijving: Option<String>,
    #[serde(rename = "KostenplaatsParentId")]
    pub kostenplaats_parent_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetKostenplaatsenResponse {
    #[serde(rename = "GetKostenplaatsenResult")]
    pub get_kostenplaatsen_result: Option<cResultGetKostenplaatsen>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetArtikelen {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<cArtikelFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cArtikelFilter {
    #[serde(rename = "ArtikelID")]
    pub artikel_id: Option<i64>,
    #[serde(rename = "ArtikelOmschrijving")]
    pub artikel_omschrijving: Option<String>,
    #[serde(rename = "ArtikelCode")]
    pub artikel_code: Option<String>,
    #[serde(rename = "GroepOmschrijving")]
    pub groep_omschrijving: Option<String>,
    #[serde(rename = "GroepCode")]
    pub groep_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cResultGetArtikelen {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<cError>,
    #[serde(rename = "Artikelen")]
    pub artikelen: Option<ArrayOfCArtikel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayOfCArtikel {
    #[serde(rename = "cArtikel")]
    pub c_artikel: Vec<cArtikel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct cArtikel {
    #[serde(rename = "ArtikelID")]
    pub artikel_id: i64,
    #[serde(rename = "ArtikelOmschrijving")]
    pub artikel_omschrijving: Option<String>,
    #[serde(rename = "ArtikelCode")]
    pub artikel_code: Option<String>,
    #[serde(rename = "GroepOmschrijving")]
    pub groep_omschrijving: Option<String>,
    #[serde(rename = "GroepCode")]
    pub groep_code: Option<String>,
    #[serde(rename = "Eenheid")]
    pub eenheid: Option<String>,
    #[serde(rename = "InkoopprijsExclBTW")]
    pub inkoopprijs_excl_btw: f64,
    #[serde(rename = "VerkoopprijsExclBTW")]
    pub verkoopprijs_excl_btw: f64,
    #[serde(rename = "VerkoopprijsInclBTW")]
    pub verkoopprijs_incl_btw: f64,
    #[serde(rename = "BTWCode")]
    pub btw_code: Option<String>,
    #[serde(rename = "TegenrekeningCode")]
    pub tegenrekening_code: Option<String>,
    #[serde(rename = "BtwPercentage")]
    pub btw_percentage: f64,
    #[serde(rename = "KostenplaatsID")]
    pub kostenplaats_id: i64,
    #[serde(rename = "Actief")]
    pub actief: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetArtikelenResponse {
    #[serde(rename = "GetArtikelenResult")]
    pub get_artikelen_result: Option<cResultGetArtikelen>,
}
