

use crate::diesel::RunQueryDsl;
use crate::juniper::{Executor, FieldResult,FieldError};
use juniper_eager_loading::{prelude::*, EagerLoading, HasOne};
use chrono::{Utc,DateTime,NaiveDate/*NaiveDateTime*/};

use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("src/graphql_schema.graphql");

use crate::db::PostgresPool;
use crate::schema::{myapp_agenda,myapp_case,auth_user,auth_assigned,/*myapp_prestation*/myapp_report,myapp_countryname,
    myapp_pathology,myapp_carbrand,myapp_contract,myapp_protocol,myapp_breakdowntype,myapp_damagetype,myapp_provider,
        myapp_providertype,myapp_field_intervention,myapp_provider_provider_field_intervention,myapp_provider_provider_providertype};
use crate::models;
//use crate::schema;

pub struct Context { 
    pub pool: PostgresPool,
}

// This impl allows us to pass in GraphQLContext as the Context for GraphQL
// objects
impl juniper::Context for Context {}

#[derive(Clone,EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]

pub struct Agenda {
    
    agenda : crate::models::Agenda,

    // these are the defaults. `#[has_one(default)]` would also work here.
    #[has_one(
        foreign_key_field = assigned_id,
        root_model_field = assigned,
        graphql_field = assigned
    )]
    assigned: HasOne<Assigned>,
    #[has_one(
        foreign_key_field = prestation_id,
        root_model_field = prestation,
        graphql_field = prestation,
    )]
    prestation: HasOne<Prestation>,
    #[has_one(
        foreign_key_field = case_id,
        root_model_field = case,
        child_primary_key_field = id,
        graphql_field = case
    )]
    case: HasOne<Case>,
    #[has_one(
        foreign_key_field = user_id,
        root_model_field = user,
        child_primary_key_field = id,
        graphql_field = user
    )]
    user: HasOne<User>,
    
}

impl AgendaFields for Agenda {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.agenda.id)
    }
    fn field_prestation(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Prestation, Walked>,
    ) -> FieldResult<&Prestation> {
        self.prestation.try_unwrap().map_err(From::from)
    }
    
    fn field_case(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Case, Walked>,
    ) -> FieldResult<&Case> {
        self.case.try_unwrap().map_err(From::from)
    }
    
    fn field_assigned(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Assigned, Walked>,
    ) -> FieldResult<&Assigned> {
        self.assigned.try_unwrap().map_err(From::from)
    }
    
    fn field_user(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<&User> {
        self.user.try_unwrap().map_err(From::from)
    }

    fn field_agenda_number(&self, _: &juniper::Executor<Context>) -> std::result::Result<&i32, juniper::FieldError> { 
         Ok(&self.agenda.agenda_number) 
    }
    fn field_contact_person(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { 
        Ok(&self.agenda.contact_person) 
    }
    fn field_date_agenda_creation(&self, _: &juniper::Executor<Context>) -> std::result::Result<chrono::DateTime<chrono::Utc>, juniper::FieldError> { 
        Ok(self.agenda.date_agenda_creation)
        
    }
    fn field_media_type_agenda(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.agenda.media_type_agenda))
        //todo!()
     }
    fn field_level_agenda(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.agenda.level_agenda))
        
    }
    fn field_telephone_number(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.agenda.telephone_number)) 
        //todo!() 
    }
    fn field_date_agenda_todo(&self, _: &juniper::Executor<Context>) -> std::result::Result<chrono::DateTime<chrono::Utc>, juniper::FieldError> { 
        Ok(self.agenda.date_agenda_todo) 
    }
    fn field_comments_agenda(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { 
        Ok(&self.agenda.comments_agenda) 
    }
    fn field_date_agenda_close(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<chrono::DateTime<chrono::Utc>>, juniper::FieldError> { 
        Ok(Some(self.agenda.date_agenda_close).unwrap())
        //todo!() 
    }
    fn field_history_agenda(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { 
        Ok(&self.agenda.history_agenda) 
    } 
    fn field_activity_agenda(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.agenda.activity_agenda))
        //todo!() 
    }  
    
}

#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Report {
    report: crate::models::Report,
    
    #[has_one(
        foreign_key_field = prestation_id,
        root_model_field = prestation,
        graphql_field = prestation,
    )]
    prestation: HasOne<Prestation>,
    #[has_one(
        foreign_key_field = case_id,
        root_model_field = case,
        child_primary_key_field = id,
        graphql_field = case
    )]
    case: HasOne<Case>,
    #[has_one(
        foreign_key_field = user_id,
        root_model_field = user,
        child_primary_key_field = id,
        graphql_field = user
    )]
    user: HasOne<User>,

}
impl ReportFields for Report{
    fn field_prestation(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Prestation, Walked>,
    ) -> FieldResult<&Prestation> {
        self.prestation.try_unwrap().map_err(From::from)
    }
    fn field_case(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Case, Walked>,
    ) -> FieldResult<&Case> {
        self.case.try_unwrap().map_err(From::from)
    }
    
    fn field_user(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<&User> {
        self.user.try_unwrap().map_err(From::from)
    }

    fn field_id(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&i32, juniper::FieldError> { 
        //todo! ()
        Ok(&self.report.id)
        
    }
    fn field_report_number(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&i32, juniper::FieldError> {
        //todo!()
        Ok(Some(&self.report.report_number).unwrap())
        //Ok(Some(&self.report.report_number)
    }
    fn field_contact(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::string::String, juniper::FieldError> {
        //todo!()
        Ok(self.report.contact.to_string()) 
        
    }
    fn field_media_type(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::string::String, juniper::FieldError> {
        //todo!()
        Ok(self.report.media_type.to_string()) 
    }
    fn field_phone_number(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        //todo!()
        Ok(self.report.phone_number.as_ref())
       
    }
    fn field_comments(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        //todo!()
        Ok(self.report.comments.as_ref())
        //Ok(Some(&self.report.report_number)
    }
    fn field_date_service_report(&self, _executor: &Executor<'_, Context>) -> std::result::Result<chrono::DateTime<chrono::Utc>, juniper::FieldError> {
        //todo!()
        Ok(self.report.date_service_report) 
        //Ok(Some(&self.case.contract_serial_number).unwrap())
        //Ok(Some(&self.report.report_number)
    }
}

#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Breakdowntype {
    breakdowntype: crate::models::Breakdowntype,
}

impl BreakdowntypeFields for Breakdowntype {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.breakdowntype.id)
    }
    fn field_breakdown(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.breakdowntype.breakdown) }
}
#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Prestation {
    prestation: crate::models::Prestation,
}

impl PrestationFields for Prestation {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.prestation.id)
    }
    fn field_libelle_prestation(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(self.prestation.as_str()) }
}
#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Damagetype {
    damagetype: crate::models::Damagetype,
}

impl DamagetypeFields for Damagetype {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.damagetype.id)
    }
    fn field_damage(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.damagetype.damage) }
}

#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Providertype {
    providertype: crate::models::Providertype,
}

impl ProvidertypeFields for Providertype {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.providertype.id)
    }
    fn field_providertype_name(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.providertype.providertype_name) }
}

#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]

pub struct Fieldintervention {
    fieldintervention: crate::models::Fieldintervention,

    #[has_one(
        foreign_key_field = country_id,
        root_model_field = country,
        graphql_field = country,
    )]
    country: HasOne<Country>,
}

impl FieldinterventionFields for Fieldintervention {
    fn field_country(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Country, Walked>,
    ) -> FieldResult<&Country> {
        self.country.try_unwrap().map_err(From::from)
    }
    
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        todo!();
        //Ok(&self.fieldintervention.id)
    }
    fn field_zipcode_field_intervention(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        todo!();
        //Ok(&self.fieldintervention.providertype_name) 
    }
    fn field_city_field_intervention(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        todo!();
        //Ok(&self.fieldintervention.providertype_name) 
    }
    fn field_radius_field_intervention(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&i32>, juniper::FieldError> { 
        todo!();
        //Ok(&self.fieldintervention.providertype_name) 
    }
    fn field_longitude_field_intervention(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<f64>, juniper::FieldError> { 
        todo!();
        //Ok(&self.fieldintervention.providertype_name) 
    }
    fn field_latitude_field_intervention(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<f64>, juniper::FieldError> { 
        todo!();
        //Ok(&self.fieldintervention.providertype_name) 
    }

}
/*
id -> Int4,
        zipcode_field_intervention -> Nullable<Varchar>,
        city_field_intervention -> Varchar,
        radius_field_intervention -> Nullable<Int4>,
        longitude_field_intervention -> Nullable<Numeric>,
        latitude_field_intervention -> Nullable<Numeric>,
        country_id -> Varchar,
*/



#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Country {
    country: crate::models::Country,
}

impl CountryFields for Country {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.country.id)
    }
    fn field_name(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.country.name) }
}

#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Pathology {
    pathology: crate::models::Pathology,
}

impl PathologyFields for Pathology {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.pathology.id)
    }
    fn field_name_of_pathology(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.pathology.name_of_pathology) }
    fn field_type_of_pathology(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.pathology.type_of_pathology) }

}

#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Carbrand {
    carbrand: crate::models::Carbrand,
}

impl CarbrandFields for Carbrand {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.carbrand.id)
    }
    fn field_brand(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.carbrand.brand) }

}

#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Protocol {
    protocol: crate::models::Protocol,
    #[has_one(
        foreign_key_field = contract_id,
        root_model_field = contract,
        graphql_field = contract,
    )]
    contract: HasOne<Contract>,
}

impl ProtocolFields for Protocol {
    fn field_contract(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Contract, Walked>,
    ) -> FieldResult<&Contract> {
        self.contract.try_unwrap().map_err(From::from)
    }

    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        //todo!();
        Ok(&self.protocol.id)
    }
    fn field_protocol_number(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(&self.protocol.protocol_number) 
    }
    fn field_description(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        //todo!()//
        Ok(&self.protocol.description) 
    }
    fn field_prefix(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(&self.protocol.prefix) 
    }
    fn field_in_progress(&self, _: &juniper::Executor<Context>) -> std::result::Result<&bool, juniper::FieldError> { 
        todo!()
        //Ok(&self.protocol.in_progress) 
    }
    fn field_validity_date(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<DateTime<Utc>>, juniper::FieldError> { 
        todo!()
        //    Ok(&self.protocol.validity_date) 
    }
    fn field_expiration_date(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<DateTime<Utc>>, juniper::FieldError> { 
        todo!()
        //        Ok(&self.protocol.expiration_date) 
    }
    fn field_customer_base(&self, _: &juniper::Executor<Context>) -> std::result::Result<&bool, juniper::FieldError> { 
        todo!()
            //        Ok(&self.protocol.expiration_date) 
    }
    fn field_contract_provision_file(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { 
        todo!()
        //        Ok(&self.protocol.expiration_date) 
    }
    fn field_guarranty(&self, _: &juniper::Executor<Context>) -> std::result::Result<&bool, juniper::FieldError> { 
        todo!()
        //        Ok(&self.protocol.expiration_date) 
    }
    fn field_other_document(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { 
        todo!()
        //        Ok(&self.protocol.expiration_date) 
    }
    fn field_alert_message(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { 
        todo!()
        //        Ok(&self.protocol.expiration_date) 
    }


}


#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Contract {
    contract: crate::models::Contract
}

impl ContractFields for Contract {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.contract.id)
    }
    fn field_name(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.contract.name) }

}



#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct User {
    user: crate::models::User,
}

impl UserFields for User {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.user.id)
    }
    fn field_password(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(self.user.password.to_string()) 
    }
    fn field_last_login(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&chrono::DateTime<chrono::Utc>>, juniper::FieldError> { 
        todo!()
        //Ok(self.user.last_login)
    }
    fn field_is_superuser(&self, _: &juniper::Executor<Context>) -> std::result::Result<bool, juniper::FieldError> { 
        Ok(self.user.is_superuser) 
    }
    fn field_username(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(self.user.username.as_ref().unwrap().to_string()) 
    }
    fn field_first_name(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        //todo!()
        Ok(self.user.first_name.as_ref()) 
    }
    fn field_last_name(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        //todo!()
        Ok(self.user.last_name.as_ref()) 
        //Ok(self.user.last_name.to_string())
    }
    fn field_email(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        //todo!()
        Ok(self.user.email.as_ref()) 
       
    }
    fn field_is_staff(&self, _: &juniper::Executor<Context>) -> std::result::Result<bool, juniper::FieldError> { 
        Ok(self.user.is_staff)
    }
    fn field_is_active(&self, _: &juniper::Executor<Context>) -> std::result::Result<bool, juniper::FieldError> { 
        Ok(self.user.is_active) 
    }
    fn field_date_joined(&self, _: &juniper::Executor<Context>) -> std::result::Result<chrono::DateTime<chrono::Utc>, juniper::FieldError> { 
        Ok(self.user.date_joined) 
    }

}
#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Assigned {
    assigned: crate::models::Assigned,
}

impl AssignedFields for Assigned {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.assigned.id)
    }
    fn field_password(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(self.assigned.password.to_string()) 
    }
    fn field_last_login(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&chrono::DateTime<chrono::Utc>>, juniper::FieldError> { 
        todo!()
        //Ok(self.user.last_login)
    }
    fn field_is_superuser(&self, _: &juniper::Executor<Context>) -> std::result::Result<bool, juniper::FieldError> { 
        Ok(self.assigned.is_superuser) 
    }
    fn field_username(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(self.assigned.username.as_ref().unwrap().to_string()) 
    }
    fn field_first_name(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        //todo!()
        Ok(self.assigned.first_name.as_ref()) 
    }
    fn field_last_name(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        //todo!()
        Ok(self.assigned.last_name.as_ref()) 
        //Ok(self.user.last_name.to_string())
    }
    fn field_email(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        //todo!()
        Ok(self.assigned.email.as_ref()) 
       
    }
    fn field_is_staff(&self, _: &juniper::Executor<Context>) -> std::result::Result<bool, juniper::FieldError> { 
        Ok(self.assigned.is_staff)
    }
    fn field_is_active(&self, _: &juniper::Executor<Context>) -> std::result::Result<bool, juniper::FieldError> { 
        Ok(self.assigned.is_active) 
    }
    fn field_date_joined(&self, _: &juniper::Executor<Context>) -> std::result::Result<chrono::DateTime<chrono::Utc>, juniper::FieldError> { 
        Ok(self.assigned.date_joined) 
    }

}


#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Case {
    case: crate::models::Case,
    #[has_one(
        foreign_key_field = country_id,
        root_model_field = country,
        graphql_field = country,
    )]
    pub country: HasOne<Country>,
    #[has_one(
        foreign_key_field = user_id,
        root_model_field = user,
        graphql_field = user
    )]
    user: HasOne<User>,
    #[has_one(
        foreign_key_field = pathology_id,
        root_model_field = pathology,
        graphql_field = pathology
    )]
    pathology: HasOne<Pathology>,
    #[has_one(
        foreign_key_field = carbrand_id,
        root_model_field = carbrand,
        graphql_field = carbrand
    )]
    pub carbrand: HasOne<Carbrand>,
    #[has_one(
        foreign_key_field = contract_id,
        root_model_field = contract,
        graphql_field = contract
    )]
    contract: HasOne<Contract>,
    #[has_one(
        foreign_key_field = protocol_id,
        root_model_field = protocol,
        graphql_field = protocol
    )]
    protocol: HasOne<Protocol>,
    #[has_one(
        foreign_key_field = breakdowntype_id,
        root_model_field = breakdowntype,
        graphql_field = breakdowntype
    )]
    breakdowntype: HasOne<Breakdowntype>,
    #[has_one(
        foreign_key_field = damagetype_id,
        root_model_field = damagetype,
        graphql_field = damagetype
    )]
    damagetype: HasOne<Damagetype>
    
}
impl CaseFields for Case{
    fn field_country(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Country, Walked>,
    ) -> FieldResult<&Country> {
        self.country.try_unwrap().map_err(From::from)
    }
    fn field_user(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<&User> {
        self.user.try_unwrap().map_err(From::from)
    }
    fn field_pathology(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Pathology, Walked>,
    ) -> FieldResult<&Pathology> {
        self.pathology.try_unwrap().map_err(From::from)
    }
    fn field_carbrand(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Carbrand, Walked>,
    ) -> FieldResult<&Carbrand> {
        self.carbrand.try_unwrap().map_err(From::from)
    }
    fn field_contract(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Contract, Walked>,
    ) -> FieldResult<&Contract> {
        self.contract.try_unwrap().map_err(From::from)
    }
    fn field_protocol(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Protocol, Walked>,
    ) -> FieldResult<&Protocol> {
        self.protocol.try_unwrap().map_err(From::from)
    }
    fn field_breakdowntype(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Breakdowntype, Walked>,
    ) -> FieldResult<&Breakdowntype> {
        self.breakdowntype.try_unwrap().map_err(From::from)
    }
    fn field_policyholder(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Country, Walked>,
    ) -> FieldResult<&Country> {
        self.country.try_unwrap().map_err(From::from)
    }
    fn field_damagetype(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Damagetype, Walked>,
    ) -> FieldResult<&Damagetype> {
        self.damagetype.try_unwrap().map_err(From::from)
    }


    fn field_id(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.case.id)
    }
    fn field_contract_serial_number(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.contract_serial_number).unwrap().as_ref())
    }
    fn field_place_of_damage(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.place_of_damage).unwrap().as_ref())
    }
    fn field_valid_from(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<chrono::NaiveDate>, juniper::FieldError> { 
        Ok(self.case.valid_from)
    }
    fn field_valid_to(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<chrono::NaiveDate>, juniper::FieldError> { 
        Ok(self.case.valid_to)
    }
    fn field_case_creation_date(&self, _executor: &Executor<'_, Context>) -> std::result::Result<chrono::DateTime<chrono::Utc>, juniper::FieldError> { 
        Ok(self.case.case_creation_date)    
    }
    fn field_insurance_company(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.insurance_company).unwrap().as_ref())

    }
    fn field_foreign_case_number(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.foreign_case_number).unwrap().as_ref())
    }
    fn field_city_case(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.city_case).unwrap().as_ref())
    }
    fn field_zipcode_case(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.zipcode_case).unwrap().as_ref())
    }
    fn field_requestor_case(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.requestor_case).unwrap().as_ref())
    }
    fn field_telephone_case(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.telephone_case).unwrap().as_ref())
    }
    fn field_distance_case(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&std::option::Option<i32>, juniper::FieldError> {
        Ok(&self.case.distance_case)       
    }
    fn field_destination_case(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.destination_case).unwrap().as_ref())
    }
    fn field_alert_message(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.alert_message).unwrap().as_ref())
    }
    fn field_contract_checking(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        //todo!()
        Ok(Some(&self.case.contract_checking))
      }
    fn field_case_file(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        //todo!()
        Ok(Some(&self.case.case_file).unwrap().as_ref())
    }
    fn field_model_name(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.model_name).unwrap().as_ref())
    }
    fn field_breakdown_description(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.breakdown_description).unwrap().as_ref())
    }
    fn field_place_immobilisation(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.place_immobilisation).unwrap().as_ref())
    }

    fn field_registration_date(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<chrono::NaiveDate>, juniper::FieldError> { 
        Ok(self.case.registration_date)
    }
    fn field_gearbox(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.gearbox))
    }

    fn field_fuel_type(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.fuel_type))
    }
    
    fn field_license_plate_number(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.license_plate_number))
    }
    
    fn field_dealership(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.dealership).unwrap().as_ref())
    }
    fn field_coverage_limit(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.coverage_limit).unwrap().as_ref())
    }
    fn field_car_serial_number(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.car_serial_number).unwrap().as_ref())
    }
    fn field_civility(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> {
        Ok(Some(&self.case.civility))
    }
    
    fn field_date_of_birth(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<chrono::NaiveDate>, juniper::FieldError> { 
        Ok(self.case.date_of_birth)
    }
    fn field_medical_center_address(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.medical_center_address).unwrap().as_ref())

    }
    fn field_pathology_type(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.pathology_type).unwrap().as_ref())

    }
    fn field_doctor(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.doctor).unwrap().as_ref())

    }
    fn field_telephone(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.telephone).unwrap().as_ref())

    }
    fn field_hospitalisation_date(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<chrono::NaiveDate>, juniper::FieldError> { 
        Ok(self.case.hospitalisation_date)
    }
    fn field_medical_information(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.medical_information).unwrap().as_ref())

    }
    fn field_medical_comments(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.medical_comments).unwrap().as_ref())


    }
    fn field_last_name(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.case.last_name)
        
    }
    fn field_first_name(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.first_name).unwrap().as_ref())

    }
    fn field_cell_phone(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.cell_phone).unwrap().as_ref())

    }
    fn field_home_phone(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.home_phone).unwrap().as_ref())

    }
    fn field_professional_phone(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.professional_phone).unwrap().as_ref())

    }
    fn field_home_address(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.home_address).unwrap().as_ref())
    }
    fn field_city(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.home_address).unwrap().as_ref())
    }
    fn field_zipcode(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.zipcode).unwrap().as_ref())
    }
    fn field_destination(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        Ok(Some(&self.case.destination).unwrap().as_ref())
    }
    fn field_return_date(&self, _executor: &Executor<'_, Context>) -> std::result::Result<std::option::Option<chrono::NaiveDate>, juniper::FieldError> { 
        Ok(self.case.return_date)
    }
    fn field_number_passagers(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&std::option::Option<i32>, juniper::FieldError> {
        Ok(&self.case.number_passagers)
    }
    //fn field_case_brand_name_id(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&i32, juniper::FieldError> {
    //    Ok(&self.case.case_brand_name_id)
    //}
    //fn field_case_breakdown_type_id(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&i32, juniper::FieldError> {
    //    Ok(&self.case.case_breakdown_type_id)
    //}
    /*
    fn field_case_client_id(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&i32, juniper::FieldError> {
        Ok(&self.case.case_client_id)
    }
    */
    /*fn field_case_pathology_id(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.case.case_pathology_id)  
    }
    */
    /*fn field_case_policyholder_id(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        Ok(&self.case.case_policyholder_id)
        
    }
    */

    /*fn field_case_protocol_id(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&i32, juniper::FieldError> {
        Ok(&self.case.case_protocol_id)
    }
    */

    /*fn field_case_type_of_damage_id(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&i32, juniper::FieldError> {
        Ok(&self.case.case_type_of_damage_id)
    }
    */
    /*fn field_case_user_id(&self, _executor: &Executor<'_, Context>) -> std::result::Result<&i32, juniper::FieldError> {
        Ok(&self.case.case_user_id)
    }
    */
}

#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Provider {
    provider: crate::models::Provider,
    #[has_one(
        foreign_key_field = country_id,
        root_model_field = country,
        graphql_field = country,
    )]
    country: HasOne<Country>,
}

impl ProviderFields for Provider {
    fn field_country(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Country, Walked>,
    ) -> FieldResult<&Country> {
        self.country.try_unwrap().map_err(From::from)
    }

    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        //todo!()
        Ok(&self.provider.id)
    }
    fn field_corporate_name(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(self.provider.corporate_name.to_string()) 
    }
    fn field_manager_name(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(self.provider.manager_name.to_string()) 
    }
    fn field_address_provider(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(self.provider.address_provider.to_string()) 
    }
    fn field_zip_code_provider(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::string::String, juniper::FieldError> { 
        // todo!()
        Ok(self.provider.zip_code_provider.to_string()) 
    }
    fn field_city_provider(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(self.provider.city_provider.to_string() )
    }
    fn field_date_of_visit(&self, _: &juniper::Executor<Context>) -> Result<std::option::Option<NaiveDate>, juniper::FieldError> { 
        //todo!()
        Ok(self.provider.date_of_visit) 
    }
    fn field_cell_phone(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(self.provider.cell_phone.to_string()) 
    }
    fn field_main_phone_number(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(self.provider.main_phone_number.to_string()) 
    }
    fn field_telephone2(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        //todo!()
        Ok(self.provider.telephone2.as_ref()) 
    }
    fn field_telephone3(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        //todo!()
        Ok(self.provider.telephone3.as_ref()) 
    }
    fn field_fax(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(&self.provider.fax) 
    }
    fn field_email_provider(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        //todo!()
        Ok(&self.provider.email_provider )
    }
    fn field_registered(&self, _: &juniper::Executor<Context>) -> std::result::Result<&bool, juniper::FieldError> { 
        //todo!()
        Ok(&self.provider.registered )
    }
    fn field_comments_provider(&self, _: &juniper::Executor<Context>) -> std::result::Result<std::option::Option<&std::string::String>, juniper::FieldError> { 
        //todo!()
        Ok(self.provider.comments_provider.as_ref()) 
    }


    
}
#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Providerproviderfieldintervention {
    providerproviderfieldintervention: crate::models::Providerproviderfieldintervention,
    #[has_one(
        foreign_key_field = provider_id,
        root_model_field = provider,
        graphql_field = provider,
    )]
    provider: HasOne<Provider>,
    #[has_one(
        foreign_key_field = fieldintervention_id,
        root_model_field = fieldintervention,
        graphql_field = fieldintervention,
    )]
    fieldintervention: HasOne<Fieldintervention>,
    
}

impl ProviderproviderfieldinterventionFields for Providerproviderfieldintervention {
    fn field_provider(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Provider, Walked>,
    ) -> FieldResult<&Provider> {
        self.provider.try_unwrap().map_err(From::from)
    }
    fn field_fieldintervention(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Fieldintervention, Walked>,
    ) -> FieldResult<&Fieldintervention> {
        self.fieldintervention.try_unwrap().map_err(From::from)
    }

    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        //todo!()
        Ok(&self.providerproviderfieldintervention.id)
    }
    
    


    
}
#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Providerproviderprovidertype {
    providerproviderprovidertype: crate::models::Providerproviderprovidertype,
    #[has_one(
        foreign_key_field = provider_id,
        root_model_field = provider,
        graphql_field = provider,
    )]
    provider: HasOne<Provider>,
    #[has_one(
        foreign_key_field = providertype_id,
        root_model_field = providertype,
        graphql_field = providertype,
    )]
    providertype: HasOne<Providertype>,
    
}

impl ProviderproviderprovidertypeFields for Providerproviderprovidertype {
    fn field_provider(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Provider, Walked>,
    ) -> FieldResult<&Provider> {
        self.provider.try_unwrap().map_err(From::from)
    }
    fn field_providertype(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Providertype, Walked>,
    ) -> FieldResult<&Providertype> {
        self.providertype.try_unwrap().map_err(From::from)
    }

    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        //todo!()
        Ok(&self.providerproviderprovidertype.id)
    }
    
    


    
}



pub struct Query;

impl QueryFields for Query{
     fn field_agendas(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Agenda, Walked>,
    ) -> FieldResult<Vec<Agenda>>{
        
        let ctx = executor.context();
        let agenda_models = myapp_agenda::table.load::<crate::models::Agenda>(&ctx.pool.get().unwrap())?;
        let mut agendas = Agenda::from_db_models(&agenda_models);
        Agenda::eager_load_all_children_for_each(&mut agendas,&agenda_models,ctx,trail)?;
        Ok(agendas)
    }
    fn field_agenda(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Agenda, Walked>,
        query :i32,

    ) -> FieldResult<std::option::Option<Agenda>>{
        let ctx = executor.context();
        let agenda_models = models::Agenda::get_agenda(query,&ctx)?;
        let mut agendas = Agenda::from_db_models(&agenda_models);
        Agenda::eager_load_all_children_for_each(&mut agendas,&agenda_models,ctx,trail)?;
        let result = &agendas[0] ;
        Ok(Some(result.to_owned()))
        
    }
    fn field_agendas_case(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Agenda, Walked>,
        query :String,

    ) -> FieldResult<std::option::Option::<std::vec::Vec<Agenda>>>{
        //println!("{}",query );
        let ctx = executor.context();
        let agenda_models = models::Agenda::get_agendas_case(query,&ctx)?;
        let mut agendas = Agenda::from_db_models(&agenda_models);
        Agenda::eager_load_all_children_for_each(&mut agendas,&agenda_models,ctx,trail)?;
        
        Ok(Some(agendas))

    }
    fn field_agendas_todo(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Agenda, Walked>,

    ) -> FieldResult<std::option::Option::<std::vec::Vec<Agenda>>>{
        //println!("{}",query );
        let ctx = executor.context();
        let agenda_models = models::Agenda::get_agendas_todo(&ctx)?;
        let mut agendas = Agenda::from_db_models(&agenda_models);
        Agenda::eager_load_all_children_for_each(&mut agendas,&agenda_models,ctx,trail)?;
        
        Ok(Some(agendas))
        
    }

    fn field_date(&self, _: &Executor<Context>) -> FieldResult<NaiveDate> {
            unimplemented!()
    }
    fn field_date_time(&self, _: &Executor<Context>) -> FieldResult<DateTime<Utc>> {
            unimplemented!()
    }    

    fn field_cases(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Case, Walked>,
    ) -> FieldResult<Vec<Case>>{
        let ctx = executor.context();
        let case_models = myapp_case::table.load::<crate::models::Case>(&ctx.pool.get().unwrap())?;
        let mut cases = Case::from_db_models(&case_models);
        Case::eager_load_all_children_for_each(&mut cases,&case_models,ctx,trail)?;
        Ok(cases)
    }
    // search one case
    

    fn field_case_duplicate(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Case, Walked>,
        query : String,
        query2: String
        // contract_serial_number_to_search = request.POST.get('contract_serial_number')
        // foreign_case_number_to_search = request.POST.get('foreign_case_number')
   

    ) -> FieldResult<std::option::Option::<std::vec::Vec<Case>>>{
        //println!("{}",query );
        //use crate::models::Case;
        let case_to_find = crate::models::CaseSearch {
            contract_serial_number: query,
            foreign_case_number: query2,
            
        };
        let ctx = executor.context();
        let case_models = models::Case::get_cases(case_to_find,&ctx)?;
        let mut cases = Case::from_db_models(&case_models);
        Case::eager_load_all_children_for_each(&mut cases,&case_models,ctx,trail)?;
        
        Ok(Some(cases))

    }
    fn field_case_search(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Case, Walked>,
        query : String,
        
        // contract_serial_number_to_search = request.POST.get('contract_serial_number')
        // foreign_case_number_to_search = request.POST.get('foreign_case_number')
   

    ) -> FieldResult<std::option::Option::<std::vec::Vec<Case>>>{
        //println!("{}",query );
        //use crate::models::Case;
        //let case_to_find = crate::models::CaseSearch {
        //    key: query,
            
            
        //};
        let ctx = executor.context();
        let case_models = models::Case::get_search_cases(query,&ctx)?;
        let mut cases = Case::from_db_models(&case_models);
        Case::eager_load_all_children_for_each(&mut cases,&case_models,ctx,trail)?;
        
        Ok(Some(cases))

    }
    fn field_case(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Case, Walked>,
        query : String,
    ) -> FieldResult<std::vec::Vec<Case>>{
        let ctx = executor.context();
        let case_models = models::Case::get_case(query,&ctx)?;
        let mut cases = Case::from_db_models(&case_models);
        Case::eager_load_all_children_for_each(&mut cases,&case_models,ctx,trail)?;
        Ok(cases)

    }
    fn field_case_search_boxed(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Case, Walked>,

        query_id : Option<String>,
        query_contract_serial_number : Option<String>,
        query_foreign_case_number : Option<String>,
        query_contract_id: Option<i32>,
        query_license_plate_number : Option<String>,
        
        query_city_case : Option<String>,
        query_protocol_id: Option<i32>,
        query_breakdowntype_id: Option<i32>,
        query_country_id: Option<String>,
        query_first_name: Option<String>,
        
        query_last_name: Option<String>,
        query_from_date : Option<String>,
        query_to_date : Option<String>,
        
    


    ) -> FieldResult<std::option::Option::<std::vec::Vec<Case>>>{
        println!("{:?}",query_contract_id );
        //use crate::models::Case;
        //let case_to_find = crate::models::CaseSearch {
        //    key: query,
        
        let query = crate::models::Params {
            id: query_id,
            contract_serial_number: query_contract_serial_number,
            foreign_case_number: query_foreign_case_number,
            contract_id: query_contract_id,
            license_plate_number: query_license_plate_number,
            city_case: query_city_case,
            protocol_id : query_protocol_id,
            breakdowntype_id : query_breakdowntype_id,
            country_id: query_country_id,
            last_name: query_last_name,
            first_name: query_first_name,
            from_date: query_from_date,
            to_date: query_to_date,
            

        };
            
        //};
        let ctx = executor.context();
        

        let case_models = models::Case::get_search_case_boxed(query,&ctx)?;
        let mut cases = Case::from_db_models(&case_models);
        Case::eager_load_all_children_for_each(&mut cases,&case_models,ctx,trail)?;
        
        Ok(Some(cases))

    }



    fn field_users(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,User, Walked>,
    ) -> FieldResult<Vec<User>>{
        let ctx = executor.context();
        let user_models = auth_user::table.load::<crate::models::User>(&ctx.pool.get().unwrap())?;
        let mut users = User::from_db_models(&user_models);
        User::eager_load_all_children_for_each(&mut users,&user_models,ctx,trail)?;
        Ok(users)
    }

    fn field_assigneds(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Assigned, Walked>,
    ) -> FieldResult<Vec<Assigned>>{
        let ctx = executor.context();
        let assigned_models = auth_assigned::table.load::<crate::models::Assigned>(&ctx.pool.get().unwrap())?;
        let mut assigneds = Assigned::from_db_models(&assigned_models);
        Assigned::eager_load_all_children_for_each(&mut assigneds,&assigned_models,ctx,trail)?;
        
        Ok(assigneds)  
    }

    fn field_prestations(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Prestation, Walked>,
    ) -> FieldResult<Vec<Prestation>>{
        let ctx = executor.context();
        //let agenda_models = models::Agenda::get_agendas_todo(&ctx)?;
        let prestation_models = models::Prestation::get_prestations(&ctx)?;
        let mut prestations = Prestation::from_db_models(&prestation_models);
        Prestation::eager_load_all_children_for_each(&mut prestations,&prestation_models,ctx,trail)?;
        Ok(prestations)  
    }
    fn field_providers(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Provider, Walked>,
    ) -> FieldResult<Vec<Provider>>{
        let ctx = executor.context();
        let provider_models = myapp_provider::table.load::<crate::models::Provider>(&ctx.pool.get().unwrap())?;
        let mut providers = Provider::from_db_models(&provider_models);
        Provider::eager_load_all_children_for_each(&mut providers,&provider_models,ctx,trail)?;
        Ok(providers)  
    }
    fn field_fieldinterventions(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Fieldintervention, Walked>,
    ) -> FieldResult<Vec<Fieldintervention>>{
        let ctx = executor.context();
        let fieldintervention_models = myapp_field_intervention::table.load::<crate::models::Fieldintervention>(&ctx.pool.get().unwrap())?;
        let mut fieldinterventions = Fieldintervention::from_db_models(&fieldintervention_models);
        Fieldintervention::eager_load_all_children_for_each(&mut fieldinterventions,&fieldintervention_models,ctx,trail)?;
        Ok(fieldinterventions)  
    }


    fn field_reports(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Report, Walked>,
    ) -> FieldResult<Vec<Report>>{
        let ctx = executor.context();
        let report_models = myapp_report::table.load::<crate::models::Report>(&ctx.pool.get().unwrap())?;
        //println!("{}", report_models);
        let mut reports = Report::from_db_models(&report_models);
        Report::eager_load_all_children_for_each(&mut reports,&report_models,ctx,trail)?;
        Ok(reports)  
    }
    fn field_reports_case(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Report, Walked>,
        query :String,

    ) -> FieldResult<std::option::Option::<std::vec::Vec<Report>>>{
        let ctx = executor.context();
        //let report_models = models::Report::get_reports_case(query,&ctx)?;
        // replacement  the ? 
        // match if any report is owned by this case
        
        // define a variable to store the result 
        let resultat; 
        let report_models = models::Report::get_reports_case(query,&ctx);
        /*
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
         
        let report = models::Report {
            id: 4, 
            report_number : 4,
            contact: String::from("someone@example.com"),
            media_type: String::from("someone@example.com"),
            phone_number: Some(String::from("someone@example.com")),
            comments: Some(String::from("someone@example.com")),
            date_service_report: dt,
            case_id: String::from("someone@example.com"),
            prestation_id: 3,
            user_id: 4,
        };
        
        */
    
    
        match report_models{
            Ok(report_models) => resultat = report_models,
            Err(_e) => resultat = Vec::<models::Report>::new()
            
        };
        //println!("resultat {:?}", resultat);
        let mut reports = Report::from_db_models(&resultat);
        Report::eager_load_all_children_for_each(&mut reports,&resultat,ctx,trail)?;
        
        Ok(Some(reports))
        //todo!()//Ok(&Some(&x))
    }
    fn field_countries(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Country, Walked>,
    ) -> FieldResult<Vec<Country>>{
        let ctx = executor.context();
        
        let country_models = myapp_countryname::table.load::<crate::models::Country>(&ctx.pool.get().unwrap())?;
        let mut countries = Country::from_db_models(&country_models);
        Country::eager_load_all_children_for_each(&mut countries,&country_models,ctx,trail)?;
        Ok(countries)  
    }
    fn field_pathologies(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Pathology, Walked>,
    ) -> FieldResult<Vec<Pathology>>{
        let ctx = executor.context();
        let pathology_models = myapp_pathology::table.load::<crate::models::Pathology>(&ctx.pool.get().unwrap())?;
        let mut pathologies = Pathology::from_db_models(&pathology_models);
        Pathology::eager_load_all_children_for_each(&mut pathologies,&pathology_models,ctx,trail)?;
        Ok(pathologies)  
    }
    

    fn field_carbrands(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Carbrand, Walked>,
    ) -> FieldResult<Vec<Carbrand>>{
        let ctx = executor.context();
        let carbrand_models = myapp_carbrand::table.load::<crate::models::Carbrand>(&ctx.pool.get().unwrap())?;
        let mut carbrands = Carbrand::from_db_models(&carbrand_models);
        
        carbrands.sort_by(|a, b| a.carbrand.id.cmp(&b.carbrand.id)); 
        
        Carbrand::eager_load_all_children_for_each(&mut carbrands,&carbrand_models,ctx,trail)?;
        Ok(carbrands)  
    }
    fn field_contracts(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Contract, Walked>,
    ) -> FieldResult<Vec<Contract>>{
        let ctx = executor.context();
        let contract_models = myapp_contract::table.load::<crate::models::Contract>(&ctx.pool.get().unwrap())?;
        let mut contracts = Contract::from_db_models(&contract_models);
        contracts.sort_by(|a, b| a.contract.name.cmp(&b.contract.name)); 
        Contract::eager_load_all_children_for_each(&mut contracts,&contract_models,ctx,trail)?;
        Ok(contracts)  
    }
    fn field_protocols(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Protocol, Walked>,
    ) -> FieldResult<Vec<Protocol>>{
        let ctx = executor.context();
        let protocol_models = myapp_protocol::table.load::<crate::models::Protocol>(&ctx.pool.get().unwrap())?;
        let mut protocols = Protocol::from_db_models(&protocol_models);
        Protocol::eager_load_all_children_for_each(&mut protocols,&protocol_models,ctx,trail)?;
        Ok(protocols)  
    }
    fn field_breakdowntypes(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Breakdowntype, Walked>,
    ) -> FieldResult<Vec<Breakdowntype>>{
        let ctx = executor.context();
        //use crate::schema::myapp_breakdowntype::dsl::*;
        let breakdowntype_models = myapp_breakdowntype::table.load::<crate::models::Breakdowntype>(&ctx.pool.get().unwrap())?;
        let mut breakdowntypes = Breakdowntype::from_db_models(&breakdowntype_models);
        Breakdowntype::eager_load_all_children_for_each(&mut breakdowntypes,&breakdowntype_models,ctx,trail)?;
        
        breakdowntypes.sort_by(|a, b| a.breakdowntype.breakdown.cmp(&b.breakdowntype.breakdown));
        
        Ok(breakdowntypes)  

        
    }
    fn field_damagetypes(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Damagetype, Walked>,
    ) -> FieldResult<Vec<Damagetype>>{
        let ctx = executor.context();
        let damagetype_models = myapp_damagetype::table.load::<crate::models::Damagetype>(&ctx.pool.get().unwrap())?;
        let mut damagetypes = Damagetype::from_db_models(&damagetype_models);
        damagetypes.sort_by(|a, b| a.damagetype.damage.cmp(&b.damagetype.damage)); 
        
        Damagetype::eager_load_all_children_for_each(&mut damagetypes,&damagetype_models,ctx,trail)?;
        Ok(damagetypes)  
    }
    fn field_providertypes(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Providertype, Walked>,
    ) -> FieldResult<Vec<Providertype>>{
        let ctx = executor.context();
        let providertype_models = myapp_providertype::table.load::<crate::models::Providertype>(&ctx.pool.get().unwrap())?;
        let mut providertypes = Providertype::from_db_models(&providertype_models);
        Providertype::eager_load_all_children_for_each(&mut providertypes,&providertype_models,ctx,trail)?;
        Ok(providertypes)  
    }
    fn field_providerproviderfieldinterventions(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Providerproviderfieldintervention, Walked>,
    ) -> FieldResult<Vec<Providerproviderfieldintervention>>{
        let ctx = executor.context();
        let providerproviderfieldintervention_models = myapp_provider_provider_field_intervention::table.load::<crate::models::Providerproviderfieldintervention>(&ctx.pool.get().unwrap())?;
        let mut providerproviderfieldinterventions = Providerproviderfieldintervention::from_db_models(&providerproviderfieldintervention_models);
        Providerproviderfieldintervention::eager_load_all_children_for_each(&mut providerproviderfieldinterventions,&providerproviderfieldintervention_models,ctx,trail)?;
        Ok(providerproviderfieldinterventions)  
    }
    fn field_providerproviderprovidertypes(
        &self, 
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Providerproviderprovidertype, Walked>,
    ) -> FieldResult<Vec<Providerproviderprovidertype>>{
        let ctx = executor.context();
        let providerproviderprovidertype_models = myapp_provider_provider_providertype::table.load::<crate::models::Providerproviderprovidertype>(&ctx.pool.get().unwrap())?;
        let mut providerproviderprovidertypes = Providerproviderprovidertype::from_db_models(&providerproviderprovidertype_models);
        Providerproviderprovidertype::eager_load_all_children_for_each(&mut providerproviderprovidertypes,&providerproviderprovidertype_models,ctx,trail)?;
        Ok(providerproviderprovidertypes)  
    }

}

pub struct Mutation;

impl MutationFields for Mutation{
    
    fn field_new_damagetype( &self, 
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_,Damagetype, Walked>,
        id_damagetype: i32,
        libelle: String
    ) -> std::result::Result<Damagetype, FieldError>{
        let ctx = executor.context();
        use diesel::prelude::*;
        use crate::schema::myapp_damagetype::dsl::*;
        let mut _result = myapp_damagetype.filter(id.eq(&id_damagetype)).load::<models::Damagetype>(&ctx.pool.get().unwrap());
        
        let _number_of_records =
                match _result{
                    Ok(_result) =>  _result.len(),
                    Err(_e) =>  0
        };
        
        //println!("result country {:?}",&_number_of_records);
        
        let new_damagetype = crate::models::Damagetype { id: id_damagetype, damage: libelle };
        let record_of_damagetype_table : models::Damagetype;
        
        if _number_of_records == 0{
            let new_damagetype_to_insert: crate::models::Damagetype = diesel::insert_into(crate::schema::myapp_damagetype::table)
                .values(&new_damagetype)
                .get_result(&ctx.pool.get().unwrap())
                .expect("Error saving new post");
                record_of_damagetype_table = new_damagetype_to_insert; 
            
            }else{
                let damagetype_to_update = diesel::update(myapp_damagetype.filter(id.eq(new_damagetype.id)))
                    .set(damage.eq(new_damagetype.damage))
                    .get_result(&ctx.pool.get().unwrap())
                    .expect("Error saving new post");
                record_of_damagetype_table = damagetype_to_update;
            }

        Ok(Damagetype{damagetype: record_of_damagetype_table})  
        
    }
    fn field_new_carbrand( &self, 
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_,Carbrand, Walked>,
        id_carbrand: i32,
        libelle: String
    ) -> std::result::Result<Carbrand, FieldError>{
        let ctx = executor.context();
        use diesel::prelude::*;
        use crate::schema::myapp_carbrand::dsl::*;
        let mut _result = myapp_carbrand.filter(id.eq(&id_carbrand)).load::<models::Carbrand>(&ctx.pool.get().unwrap());
        
        let _number_of_records =
                match _result{
                    Ok(_result) =>  _result.len(),
                    Err(_e) =>  0
        };
        
        //println!("result country {:?}",&_number_of_records);
        
        let new_carbrand = crate::models::Carbrand { id: id_carbrand, brand: libelle };
        let record_of_carbrand_table : models::Carbrand;
        
        if _number_of_records == 0{
            let new_carbrand_to_insert: crate::models::Carbrand = diesel::insert_into(crate::schema::myapp_carbrand::table)
                .values(&new_carbrand)
                .get_result(&ctx.pool.get().unwrap())
                .expect("Error saving new post");
                record_of_carbrand_table = new_carbrand_to_insert; 
            
            } else {
                let carbrand_to_update = diesel::update(myapp_carbrand.filter(id.eq(new_carbrand.id)))
                    .set(brand.eq(new_carbrand.brand))
                    .get_result(&ctx.pool.get().unwrap())
                    .expect("Error saving new post");
                record_of_carbrand_table = carbrand_to_update;
            }

        Ok(Carbrand{carbrand: record_of_carbrand_table})  
        
    }
    fn field_new_prestation( &self, 
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_,Prestation, Walked>,
        id_prestation: i32,
        libelle: String
    ) -> std::result::Result<Prestation, FieldError>{
        let ctx = executor.context();
        use diesel::prelude::*;
        use crate::schema::myapp_prestation::dsl::{id, libelle_prestation, myapp_prestation};
        let mut _result = myapp_prestation.filter(id.eq(&id_prestation)).load::<models::Prestation>(&ctx.pool.get().unwrap());
        
        let _number_of_records =
                match _result{
                    Ok(_result) =>  _result.len(),
                    Err(_e) =>  0
        };
        
        //println!("result country {:?}",&_number_of_records);
        
        let new_prestation = crate::models::Prestation { id: id_prestation, libelle_prestation: libelle };
        let record_of_prestation_table : models::Prestation;
        
        if _number_of_records == 0{
            let new_prestation_to_insert: crate::models::Prestation = diesel::insert_into(crate::schema::myapp_prestation::table)
                .values(&new_prestation)
                .get_result(&ctx.pool.get().unwrap())
                .expect("Error saving new post");
                record_of_prestation_table = new_prestation_to_insert; 
            
            }else{
                let prestation_to_update = diesel::update(myapp_prestation.filter(id.eq(new_prestation.id)))
                    .set(libelle_prestation.eq(new_prestation.libelle_prestation))
                    .get_result(&ctx.pool.get().unwrap())
                    .expect("Error saving new post");
                record_of_prestation_table = prestation_to_update;
            }

        Ok(Prestation{prestation: record_of_prestation_table})  
    }
        
    
    fn field_new_country( &self, 
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_,Country, Walked>,
        id_country: String,
        libelle: String
    ) -> std::result::Result<Country, FieldError>{
        let ctx = executor.context();
        use diesel::prelude::*;
        use crate::schema::myapp_countryname::dsl::*;
        let mut _result = myapp_countryname.filter(id.eq(&id_country)).load::<models::Country>(&ctx.pool.get().unwrap());
        
        let _number_of_records =
                match _result{
                    Ok(_result) =>  _result.len(),
                    Err(_e) =>  0
        };
        
        //println!("result country {:?}",&_number_of_records);
        
        let new_country = crate::models::Country { id: id_country, name: libelle };
        let record_of_countryname_table : models::Country;
        
        if _number_of_records == 0{
            let new_country_to_insert: crate::models::Country = diesel::insert_into(crate::schema::myapp_countryname::table)
                .values(&new_country)
                .get_result(&ctx.pool.get().unwrap())
                .expect("Error saving new post");
                record_of_countryname_table = new_country_to_insert; 
            
            }else{
                let country_to_update = diesel::update(myapp_countryname.filter(id.eq(new_country.id)))
                    .set(name.eq(new_country.name))
                    .get_result(&ctx.pool.get().unwrap())
                    .expect("Error saving new post");
                record_of_countryname_table = country_to_update;
            }

        Ok(Country{country: record_of_countryname_table})  
    }
    fn field_new_providertype( &self, 
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_,Providertype, Walked>,
        id_providertype: i32,
        libelle: String
    ) -> std::result::Result<Providertype, FieldError>{
        let ctx = executor.context();
        use diesel::prelude::*;
        use crate::schema::myapp_providertype::dsl::*;
        let mut _result = myapp_providertype.filter(id.eq(&id_providertype)).load::<models::Providertype>(&ctx.pool.get().unwrap());
        
        let _number_of_records =
                match _result{
                    Ok(_result) =>  _result.len(),
                    Err(_e) =>  0
        };
        
        //println!("result country {:?}",&_number_of_records);
        
        let new_providertype = crate::models::NewProvidertype { id: id_providertype, providertype_name: libelle };
        let record_of_providertype_table : models::Providertype;
        
        if _number_of_records == 0 {
            let new_providertype_to_insert: crate::models::Providertype = diesel::insert_into(crate::schema::myapp_providertype::table)
                .values(&new_providertype)
                .get_result(&ctx.pool.get().unwrap())
                .expect("Error saving new post");
                record_of_providertype_table = new_providertype_to_insert; 
            
            }else{
                let providertype_to_update = diesel::update(myapp_providertype.filter(id.eq(new_providertype.id)))
                    .set(providertype_name.eq(new_providertype.providertype_name))
                    .get_result(&ctx.pool.get().unwrap())
                    .expect("Error saving new post");
                record_of_providertype_table = providertype_to_update;
            }

        Ok(Providertype{providertype: record_of_providertype_table})  
    }
    fn field_new_case( &self, 
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_,Case, Walked>, 
        new_id: String,
        new_contract_serial_number: Option<String>,
        new_place_of_damage: Option<String>,
        new_valid_from: Option<String>,
        new_valid_to: Option<String>,
        new_case_creation_date: String,
        new_insurance_company: Option<String>,
        new_foreign_case_number: Option<String>,
        new_city_case: Option<String>, 
        new_zipcode_case: Option<String>,
        new_requestor_case: Option<String>,
        new_telephone_case: Option<String>,
        new_distance_case: Option<i32>,
        new_destination_case: Option<String>,
        new_alert_message: Option<String>,
        new_contract_checking: String,
        new_case_file: Option<String>,
        new_model_name: Option<String>,
        new_breakdown_description: Option<String>,
        new_place_immobilisation: Option<String>,
        new_registration_date: Option<String>,
        new_gearbox: String,
        new_fuel_type: String,
        new_license_plate_number: String,
        new_dealership: Option<String>, 
        new_coverage_limit: Option<String>,
        new_car_serial_number: Option<String>,
        new_civility: String,
        new_date_of_birth: Option<String>,
        new_medical_center_address: Option<String>,
        new_pathology_type: Option<String>,
        new_doctor: Option<String>, 
        new_telephone: Option<String>,
        new_hospitalisation_date: Option<String>,
        new_medical_information: Option<String>,
        new_medical_comments: Option<String>,
        new_last_name: String,
        new_first_name: Option<String>,
        new_cell_phone: Option<String>,
        new_home_phone: Option<String>,
        new_professional_phone: Option<String>,
        new_home_address: Option<String>,
        new_city: Option<String>,
        new_zipcode: Option<String>,
        new_destination: Option<String>,
        new_return_date: Option<String>,
        new_number_passagers: Option<i32>,
        new_carbrand_id: i32,
        new_breakdowntype_id: i32,
        new_contract_id: i32,
        new_country_id: String,
        new_pathology_id: String,
        new_policyholder_id: String,
        new_protocol_id: i32,
        new_damagetype_id: i32,
        new_user_id: i32
    ) -> std::result::Result<Case, FieldError>{
        let ctx = executor.context();
        use diesel::prelude::*;
        use crate::schema::myapp_case::dsl::*;
        let mut _result = myapp_case.filter(id.eq(&id)).load::<models::Case>(&ctx.pool.get().unwrap());
        
        let _number_of_records =
                match _result{
                    Ok(_result) =>  _result.len(),
                    Err(_e) =>  0
        };
        //let dt = NaiveDate::from_ymd(2020, 3, 1).and_hms_milli(0, 0, 0, 0);
        //let time = NaiveTime::from_hms_milli(0, 0, 0, 0);
        let new_date_valid_from = Some(NaiveDate::parse_from_str(&new_valid_from.unwrap(), "%Y-%m-%d").unwrap());
        let new_date_valid_to = Some(NaiveDate::parse_from_str(&new_valid_to.unwrap(), "%Y-%m-%d").unwrap());
        
        let new_registration_date = Some(NaiveDate::parse_from_str(&new_registration_date.unwrap(), "%Y-%m-%d").unwrap());
        
        let new_date_of_birth = Some(NaiveDate::parse_from_str(&new_date_of_birth.unwrap(), "%Y-%m-%d").unwrap());
        
        let new_hospitalisation_date = Some(NaiveDate::parse_from_str(&new_hospitalisation_date.unwrap(), "%Y-%m-%d").unwrap());
        let new_return_date = Some(NaiveDate::parse_from_str(&new_return_date.unwrap(), "%Y-%m-%d").unwrap());
        
        
        //let new_case_naive_date_creation_date = NaiveDate::parse_from_str(&new_valid_to.unwrap(), "%Y-%m-%d").unwrap();
        //let new_case_naive_date_creation_date_time  = new_case_naive_date_creation_date.and_hms(9, 10, 11);
        
        //let dt = new_case_naive_date_creation_date_time.from_timestamp(61, 0, Utc);

        use chrono::prelude::{Local};

        let utc: DateTime<Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
        let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`

        //println!("result country {:?}",&_number_of_records);
        
        let new_case = crate::models::NewCase { 
            id: new_id, 
            contract_serial_number: new_contract_serial_number,
            place_of_damage : new_place_of_damage,
            valid_from: new_date_valid_from,
            valid_to: new_date_valid_to,
            case_creation_date: utc,
            insurance_company: new_insurance_company,
            foreign_case_number: new_foreign_case_number,
            city_case: new_city_case,
            zipcode_case: new_zipcode_case,
            requestor_case : new_requestor_case, 
            telephone_case: new_telephone_case,
            distance_case: new_distance_case,
            destination_case: new_destination_case,
            alert_message: new_alert_message,
            contract_checking: new_contract_checking, 
            case_file: new_case_file,
            model_name: new_model_name,
            breakdown_description: new_breakdown_description,
            place_immobilisation: new_place_immobilisation,
            registration_date: new_registration_date,
            gearbox: new_gearbox,
            fuel_type: new_fuel_type,
            license_plate_number: new_license_plate_number,
            dealership: new_dealership,
            coverage_limit: new_coverage_limit,
            car_serial_number: new_car_serial_number,
            civility: new_civility,
            date_of_birth: new_date_of_birth,
            medical_center_address: new_medical_center_address,
            pathology_type: new_pathology_type,
            doctor: new_doctor,
            telephone: new_telephone,
            hospitalisation_date: new_hospitalisation_date,
            medical_information: new_medical_information,
            medical_comments: new_medical_comments,
            last_name: new_last_name,
            first_name: new_first_name,
            cell_phone: new_cell_phone,
            home_phone: new_home_phone,
            professional_phone: new_professional_phone,
            home_address: new_home_address,
            city: new_city,
            zipcode: new_zipcode,
            destination: new_destination,
            return_date: new_return_date,
            number_passagers: new_number_passagers,
            carbrand_id: new_carbrand_id,
            breakdowntype_id: new_breakdowntype_id,
            contract_id: new_contract_id,
            country_id: new_country_id,
            pathology_id: new_pathology_id,
            policyholder_id: new_policyholder_id,
            protocol_id: new_protocol_id,
            damagetype_id: new_damagetype_id,
            user_id: new_user_id,
            
        };
        
        let record_of_case_table : models::Case;
        let case_to_update : models::Case;
        
        
            
        
        
        
        Ok(Case{case: record_of_case_table,carbrand} )
            
        
    }
    

}


    
   

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
/*let case_to_find = crate::models::Case {
    id: query,
    
    contract_serial_number: Some(query2),
    place_of_damage: Some(String::from("")),//Option<String>,
    valid_from: Some(NaiveDate::from_ymd(2017, 11, 12)),//Option<NaiveDate>,
    valid_to: Some(NaiveDate::from_ymd(2017, 11, 12)),//Option<NaiveDate>,
    case_creation_date: Utc::now(),//,"12/12/2010",
    //case_creation_date: Utc::now(), //DateTime("12/12/2010"),
    //DateTime<Utc>,
    insurance_company: Some(String::from("")),//Option<String>,
    foreign_case_number: Some(String::from("")),//Option<String>,
    city_case: Some(String::from("")),//Option<String>,
    zipcode_case: Some(String::from("")), //Option<String>,
    requestor_case: Some(String::from("")),//Option<String>,
    telephone_case: Some(String::from("")),//Option<String>,
    distance_case: Some(0),//Option<i32>,
    destination_case: Some(String::from("")), //Option<String>,
    alert_message: Some(String::from("")), //Option<String>,
    contract_checking: String::from(""),//"ee",
    case_file: Some(String::from("")),//Option<String>,
    model_name: Some(String::from("")),//Option<String>,
    breakdown_description: Some(String::from("")), //Option<String>,
    place_immobilisation: Some(String::from("")),//,Option<String>,
    registration_date: Some(NaiveDate::from_ymd(2017, 11, 12)),//Option<NaiveDate>,
    gearbox: String::from(""),//"1",
    fuel_type: String::from("") ,//"1",
    license_plate_number: String::from(""),//"3",
    dealership: Some(String::from("")),//Option<String>,
    coverage_limit: Some(String::from("")),//Option<String>,
    car_serial_number: Some(String::from("")),//Option<String>,
    civility: String::from(""),//"A",
    date_of_birth: Some(NaiveDate::from_ymd(2017, 11, 12)),//Some(),//Option<NaiveDate>,
    medical_center_address: Some(String::from("")) , //Option<String>,
    pathology_type: Some(String::from("")),//Option<String>,
    doctor: Some(String::from("")),//Option<String>,
    telephone: Some(String::from("")),//Option<String>,
    hospitalisation_date: Some(NaiveDate::from_ymd(2017, 11, 12)),//Option<NaiveDate>,
    medical_information: Some(String::from("")),//Option<String>,
    medical_comments: Some(String::from("")),//Option<String>,
    last_name: String::from(""),//
    first_name: Some(String::from("")) , //Option<String>,
    cell_phone: Some(String::from("")),//Option<String>,
    home_phone: Some(String::from("")),//Option<String>,
    professional_phone: Some(String::from("")),//Option<String>,
    home_address: Some(String::from("")),//Option<String>,
    city: Some(String::from("")),//Option<String>,
    zipcode: Some(String::from("")), //Option<String>,
    destination:Some(String::from("")),// Option<String>,
    return_date: Some(NaiveDate::from_ymd(2017, 11, 12)),//Option<NaiveDate>,
    number_passagers: Some(0),//Option<i32>,
    carbrand_id: 1,
    breakdowntype_id: 1,
    contract_id: 1,
    country_id: String::from(""),
    pathology_id: String::from(""),
    policyholder_id: String::from(""),
    protocol_id: 1,
    damagetype_id: 1,
    user_id: 1,
};
*/