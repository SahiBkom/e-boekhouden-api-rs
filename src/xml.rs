use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetAdministraties")]
pub struct GetAdministraties {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetAdministratiesResponse")]
pub struct GetAdministratiesResponse {
    #[serde(rename = "GetAdministratiesResult")]
    pub get_administraties_result: Option<CResultGetAdministraties>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultGetAdministraties")]
pub struct CResultGetAdministraties {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Administraties")]
    pub administraties: Option<ArrayOfCAdministratie>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cError")]
pub struct CError {
    #[serde(rename = "LastErrorCode")]
    pub last_error_code: Option<String>,
    #[serde(rename = "LastErrorDescription")]
    pub last_error_description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "ArrayOfCAdministratie")]
pub struct ArrayOfCAdministratie {
    #[serde(rename = "cAdministratie")]
    pub c_administratie: Vec<CAdministratie>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cAdministratie")]
pub struct CAdministratie {
    #[serde(rename = "Bedrijf")]
    pub bedrijf: Option<String>,
    #[serde(rename = "Plaats")]
    pub plaats: Option<String>,
    #[serde(rename = "Guid")]
    pub guid: Option<String>,
    #[serde(rename = "StartBoekjaar")]
    pub start_boekjaar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetSaldo")]
pub struct GetSaldo {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<CSaldoFilter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cSaldoFilter")]
pub struct CSaldoFilter {
    #[serde(rename = "GbCode")]
    pub gb_code: Option<String>,
    #[serde(rename = "KostenPlaatsId")]
    pub kosten_plaats_id: i64,
    #[serde(rename = "DatumVan")]
    pub datum_van: String,
    #[serde(rename = "DatumTot")]
    pub datum_tot: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetSaldoResponse")]
pub struct GetSaldoResponse {
    #[serde(rename = "GetSaldoResult")]
    pub get_saldo_result: Option<CResultGetSaldo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultGetSaldo")]
pub struct CResultGetSaldo {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Saldo")]
    pub saldo: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "AddFactuur")]
pub struct AddFactuur {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oFact")]
    pub o_fact: Option<CFactuur>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cFactuur")]
pub struct CFactuur {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "ArrayOfCFactuurRegel")]
pub struct ArrayOfCFactuurRegel {
    #[serde(rename = "cFactuurRegel")]
    pub c_factuur_regel: Vec<CFactuurRegel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cFactuurRegel")]
pub struct CFactuurRegel {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "AddFactuurResponse")]
pub struct AddFactuurResponse {
    #[serde(rename = "AddFactuurResult")]
    pub add_factuur_result: Option<CResultAddFactuur>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultAddFactuur")]
pub struct CResultAddFactuur {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Factuurnummer")]
    pub factuurnummer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetFacturen")]
pub struct GetFacturen {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<CFactuurFilter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cFactuurFilter")]
pub struct CFactuurFilter {
    #[serde(rename = "Factuurnummer")]
    pub factuurnummer: Option<String>,
    #[serde(rename = "Relatiecode")]
    pub relatiecode: Option<String>,
    #[serde(rename = "DatumVan")]
    pub datum_van: String,
    #[serde(rename = "DatumTm")]
    pub datum_tm: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetFacturenResponse")]
pub struct GetFacturenResponse {
    #[serde(rename = "GetFacturenResult")]
    pub get_facturen_result: Option<CResultGetFacturen>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultGetFacturen")]
pub struct CResultGetFacturen {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Facturen")]
    pub facturen: Option<ArrayOfCFactuurList>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "ArrayOfCFactuurList")]
pub struct ArrayOfCFactuurList {
    #[serde(rename = "cFactuurList")]
    pub c_factuur_list: Vec<CFactuurList>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cFactuurList")]
pub struct CFactuurList {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "AddMutatie")]
pub struct AddMutatie {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oMut")]
    pub o_mut: Option<CMutatie>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cMutatie")]
pub struct CMutatie {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "ArrayOfCMutatieRegel")]
pub struct ArrayOfCMutatieRegel {
    #[serde(rename = "cMutatieRegel")]
    pub c_mutatie_regel: Vec<CMutatieRegel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cMutatieRegel")]
pub struct CMutatieRegel {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultAddMutatie")]
pub struct CResultAddMutatie {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Mutatienummer")]
    pub mutatienummer: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "AddMutatieResponse")]
pub struct AddMutatieResponse {
    #[serde(rename = "AddMutatieResult")]
    pub add_mutatie_result: Option<CResultAddMutatie>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetMutaties")]
pub struct GetMutaties {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<CMutatieFilter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cMutatieFilter")]
pub struct CMutatieFilter {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultGetMutaties")]
pub struct CResultGetMutaties {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Mutaties")]
    pub mutaties: Option<ArrayOfCMutatieList>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "ArrayOfCMutatieList")]
pub struct ArrayOfCMutatieList {
    #[serde(rename = "cMutatieList")]
    pub c_mutatie_list: Vec<CMutatieList>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cMutatieList")]
pub struct CMutatieList {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "ArrayOfCMutatieListRegel")]
pub struct ArrayOfCMutatieListRegel {
    #[serde(rename = "cMutatieListRegel")]
    pub c_mutatie_list_regel: Vec<CMutatieListRegel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cMutatieListRegel")]
pub struct CMutatieListRegel {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetMutatiesResponse")]
pub struct GetMutatiesResponse {
    #[serde(rename = "GetMutatiesResult")]
    pub get_mutaties_result: Option<CResultGetMutaties>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "AddGrootboekrekening")]
pub struct AddGrootboekrekening {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oGb")]
    pub o_gb: Option<CGrootboekrekening>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cGrootboekrekening")]
pub struct CGrootboekrekening {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultAddGrootboekrekening")]
pub struct CResultAddGrootboekrekening {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Gb_ID")]
    pub gb_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "AddGrootboekrekeningResponse")]
pub struct AddGrootboekrekeningResponse {
    #[serde(rename = "AddGrootboekrekeningResult")]
    pub add_grootboekrekening_result: Option<CResultAddGrootboekrekening>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "UpdateGrootboekrekening")]
pub struct UpdateGrootboekrekening {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oGb")]
    pub o_gb: Option<CGrootboekrekening>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "UpdateGrootboekrekeningResponse")]
pub struct UpdateGrootboekrekeningResponse {
    #[serde(rename = "UpdateGrootboekrekeningResult")]
    pub update_grootboekrekening_result: Option<CError>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetGrootboekrekeningen")]
pub struct GetGrootboekrekeningen {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<CGrootboekrekeningFilter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cGrootboekrekeningFilter")]
pub struct CGrootboekrekeningFilter {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Categorie")]
    pub categorie: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultGetGrootboekrekeningen")]
pub struct CResultGetGrootboekrekeningen {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Rekeningen")]
    pub rekeningen: Option<ArrayOfCGrootboekrekening>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "ArrayOfCGrootboekrekening")]
pub struct ArrayOfCGrootboekrekening {
    #[serde(rename = "cGrootboekrekening")]
    pub c_grootboekrekening: Vec<CGrootboekrekening>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetGrootboekrekeningenResponse")]
pub struct GetGrootboekrekeningenResponse {
    #[serde(rename = "GetGrootboekrekeningenResult")]
    pub get_grootboekrekeningen_result: Option<CResultGetGrootboekrekeningen>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "AddRelatie")]
pub struct AddRelatie {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oRel")]
    pub o_rel: Option<CRelatie>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cRelatie")]
pub struct CRelatie {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultAddRelatie")]
pub struct CResultAddRelatie {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Rel_ID")]
    pub rel_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "AddRelatieResponse")]
pub struct AddRelatieResponse {
    #[serde(rename = "AddRelatieResult")]
    pub add_relatie_result: Option<CResultAddRelatie>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "UpdateRelatie")]
pub struct UpdateRelatie {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "oRel")]
    pub o_rel: Option<CRelatie>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "UpdateRelatieResponse")]
pub struct UpdateRelatieResponse {
    #[serde(rename = "UpdateRelatieResult")]
    pub update_relatie_result: Option<CError>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetRelaties")]
pub struct GetRelaties {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<CRelatieFilter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cRelatieFilter")]
pub struct CRelatieFilter {
    #[serde(rename = "Trefwoord")]
    pub trefwoord: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "ID")]
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultGetRelaties")]
pub struct CResultGetRelaties {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Relaties")]
    pub relaties: Option<ArrayOfCRelatie>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "ArrayOfCRelatie")]
pub struct ArrayOfCRelatie {
    #[serde(rename = "cRelatie")]
    pub c_relatie: Vec<CRelatie>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetRelatiesResponse")]
pub struct GetRelatiesResponse {
    #[serde(rename = "GetRelatiesResult")]
    pub get_relaties_result: Option<CResultGetRelaties>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetOpenPosten")]
pub struct GetOpenPosten {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "OpSoort")]
    pub op_soort: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultGetOpenPosten")]
pub struct CResultGetOpenPosten {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Openposten")]
    pub openposten: Option<ArrayOfCOpenPost>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "ArrayOfCOpenPost")]
pub struct ArrayOfCOpenPost {
    #[serde(rename = "cOpenPost")]
    pub c_open_post: Vec<COpenPost>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cOpenPost")]
pub struct COpenPost {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetOpenPostenResponse")]
pub struct GetOpenPostenResponse {
    #[serde(rename = "GetOpenPostenResult")]
    pub get_open_posten_result: Option<CResultGetOpenPosten>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "OpenSession")]
pub struct OpenSession {
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "SecurityCode1")]
    pub security_code_1: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultOpenSession")]
pub struct CResultOpenSession {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "OpenSessionResponse")]
pub struct OpenSessionResponse {
    #[serde(rename = "OpenSessionResult")]
    pub open_session_result: Option<CResultOpenSession>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "CloseSession")]
pub struct CloseSession {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "CloseSessionResponse")]
pub struct CloseSessionResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "AutoLogin")]
pub struct AutoLogin {
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultAutoLogin")]
pub struct CResultAutoLogin {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Token")]
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "AutoLoginResponse")]
pub struct AutoLoginResponse {
    #[serde(rename = "AutoLoginResult")]
    pub auto_login_result: Option<CResultAutoLogin>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetKostenplaatsen")]
pub struct GetKostenplaatsen {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<CKostenplaatsFilter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cKostenplaatsFilter")]
pub struct CKostenplaatsFilter {
    #[serde(rename = "KostenplaatsID")]
    pub kostenplaats_id: Option<i64>,
    #[serde(rename = "KostenplaatsParentID")]
    pub kostenplaats_parent_id: Option<i64>,
    #[serde(rename = "Omschrijving")]
    pub omschrijving: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultGetKostenplaatsen")]
pub struct CResultGetKostenplaatsen {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Kostenplaatsen")]
    pub kostenplaatsen: Option<ArrayOfCKostenplaats>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "ArrayOfCKostenplaats")]
pub struct ArrayOfCKostenplaats {
    #[serde(rename = "cKostenplaats")]
    pub c_kostenplaats: Vec<CKostenplaats>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cKostenplaats")]
pub struct CKostenplaats {
    #[serde(rename = "KostenplaatsId")]
    pub kostenplaats_id: i64,
    #[serde(rename = "Omschrijving")]
    pub omschrijving: Option<String>,
    #[serde(rename = "KostenplaatsParentId")]
    pub kostenplaats_parent_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetKostenplaatsenResponse")]
pub struct GetKostenplaatsenResponse {
    #[serde(rename = "GetKostenplaatsenResult")]
    pub get_kostenplaatsen_result: Option<CResultGetKostenplaatsen>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetArtikelen")]
pub struct GetArtikelen {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<CArtikelFilter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cArtikelFilter")]
pub struct CArtikelFilter {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cResultGetArtikelen")]
pub struct CResultGetArtikelen {
    #[serde(rename = "ErrorMsg")]
    pub error_msg: Option<CError>,
    #[serde(rename = "Artikelen")]
    pub artikelen: Option<ArrayOfCArtikel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "ArrayOfCArtikel")]
pub struct ArrayOfCArtikel {
    #[serde(rename = "cArtikel")]
    pub c_artikel: Vec<CArtikel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "cArtikel")]
pub struct CArtikel {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "GetArtikelenResponse")]
pub struct GetArtikelenResponse {
    #[serde(rename = "GetArtikelenResult")]
    pub get_artikelen_result: Option<CResultGetArtikelen>,
}
