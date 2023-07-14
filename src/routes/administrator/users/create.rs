use bcrypt::DEFAULT_COST;
use diesel::Connection;
use lettre::Address;
use rand::{distributions::Alphanumeric, Rng};
use rocket::{form::Form, get, http::Status, post, FromForm, State};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    base_layout_context::BaseLayoutContext,
    database::Database,
    error::Error,
    index::{Generation, Generations, Index, Program, Programs},
    localization::Script,
    mail::Mail,
    user::{AccountType, Administrator, User},
};

#[derive(Serialize, Debug)]
struct StudentData {
    programs: Vec<Program>,
    generations: Vec<Generation>,
    previous: Option<FormDataStudent>,
}

#[derive(Serialize, Debug)]
struct ProfessorData {
    previous: Option<FormDataProfessor>,
}

#[derive(Serialize, Debug)]
struct AdministratorData {
    previous: Option<FormDataAdministrator>,
}

#[derive(Serialize, Debug)]
enum AccountTypeWithData {
    Student(StudentData),
    Professor(ProfessorData),
    Administrator(AdministratorData),
}

impl AccountTypeWithData {
    pub async fn new_student(
        database: &Database,
        previous: Option<FormDataStudent>,
    ) -> Result<AccountTypeWithData, Error> {
        let generations = database.run(|c| Generations::get(c)).await?.0;
        let programs = database.run(|c| Programs::get(c)).await?.0;
        Ok(AccountTypeWithData::Student(StudentData {
            programs,
            generations,
            previous,
        }))
    }

    pub async fn new_professor(
        previous: Option<FormDataProfessor>,
    ) -> Result<AccountTypeWithData, Error> {
        Ok(AccountTypeWithData::Professor(ProfessorData { previous }))
    }

    pub async fn new_administrator(
        previous: Option<FormDataAdministrator>,
    ) -> Result<AccountTypeWithData, Error> {
        Ok(AccountTypeWithData::Administrator(AdministratorData {
            previous,
        }))
    }
}

#[derive(Serialize, Debug)]
struct LayoutContext {
    #[serde(flatten)]
    base_layout_context: BaseLayoutContext,
    account_type: Option<AccountTypeWithData>,
    show_success_message: bool,
    show_invalid_email: bool,
    show_duplicate_data: bool,
}

impl LayoutContext {
    pub async fn new(language: Script, user: &User) -> Result<Self, Error> {
        Ok(Self {
            base_layout_context: BaseLayoutContext::new(language, user).await?,
            account_type: None,
            show_success_message: false,
            show_duplicate_data: false,
            show_invalid_email: false,
        })
    }

    pub fn success(mut self) -> Self {
        self.show_success_message = true;
        self
    }

    pub fn duplicate_data(mut self) -> Self {
        self.show_duplicate_data = true;
        self
    }

    pub fn invalid_email(mut self) -> Self {
        self.show_invalid_email = true;
        self
    }

    pub fn with_account_type(mut self, account_type: Option<AccountTypeWithData>) -> Self {
        self.account_type = account_type;
        self
    }
}

#[get("/users/create", rank = 2)]
pub async fn get_no_data(
    language: Script,
    administrator: Administrator<'_>,
) -> Result<Template, Status> {
    let user = administrator.0;
    Ok(Template::render(
        "routes/administrator/users/create",
        LayoutContext::new(language, user).await?,
    ))
}

#[get("/users/create?<account_type>", rank = 1)]
pub async fn get_with_account_type(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    account_type: AccountType,
) -> Result<Template, Status> {
    let user = administrator.0;

    let template_path = "routes/administrator/users/create";

    let account_type = match account_type {
        AccountType::Student => AccountTypeWithData::new_student(&database, None).await?,
        AccountType::Professor => AccountTypeWithData::new_professor(None).await?,
        AccountType::Administrator => AccountTypeWithData::new_administrator(None).await?,
    };

    let context = LayoutContext::new(language, user)
        .await?
        .with_account_type(Some(account_type));

    println!("{:?}", context);

    Ok(Template::render(template_path, context))
}

fn generate_random_password() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

#[derive(Serialize, FromForm, Debug, Clone)]
pub struct FormDataAdministrator {
    email: String,
    first_name: String,
    last_name: String,
}

#[post("/users/create?account_type=Administrator", data = "<form>", rank = 2)]
pub async fn post_administrator(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    mail: &State<Mail>,
    form: Form<FormDataAdministrator>,
) -> Result<Template, Status> {
    let user = administrator.0;

    let context = LayoutContext::new(language, user)
        .await?
        .with_account_type(Some(
            AccountTypeWithData::new_administrator(Some(form.clone())).await?,
        ));
    let template_path = "routes/administrator/users/create";

    let address = match Address::try_from(form.email.clone()) {
        Ok(address) => address,
        Err(_) => return Ok(Template::render(template_path, context.invalid_email())),
    };

    let plain_password: String = generate_random_password();
    let first_name = Some(form.first_name.clone()).filter(|s| s.is_empty());
    let last_name = Some(form.last_name.clone()).filter(|s| s.is_empty());

    let password = bcrypt::hash(plain_password.clone(), DEFAULT_COST).map_err(Error::from)?;

    let builder = User::builder(form.email.clone(), password)
        .with_first_name(first_name)
        .with_last_name(last_name)
        .with_account_type(AccountType::Administrator);

    match database.run(move |c| builder.build().create(c)).await {
        Ok(_) => (),
        Err(Error::DatabaseDuplicateEntry) => {
            return Ok(Template::render(template_path, context.duplicate_data()))
        }
        Err(e) => return Err(e.into()),
    }

    mail.send_invite(address, &plain_password).await?;

    Ok(Template::render(template_path, context.success()))
}

#[derive(Serialize, FromForm, Debug, Clone)]
pub struct FormDataProfessor {
    email: String,
    first_name: String,
    last_name: String,
}

#[post("/users/create?account_type=Professor", data = "<form>", rank = 1)]
pub async fn post_professor(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    mail: &State<Mail>,
    form: Form<FormDataProfessor>,
) -> Result<Template, Status> {
    let user = administrator.0;

    let context = LayoutContext::new(language, user)
        .await?
        .with_account_type(Some(
            AccountTypeWithData::new_professor(Some(form.clone())).await?,
        ));
    let template_path = "routes/administrator/users/create";

    let address = match Address::try_from(form.email.clone()) {
        Ok(address) => address,
        Err(_) => return Ok(Template::render(template_path, context.invalid_email())),
    };

    let plain_password: String = generate_random_password();
    let first_name = Some(form.first_name.clone()).filter(|s| s.is_empty());
    let last_name = Some(form.last_name.clone()).filter(|s| s.is_empty());

    let password = bcrypt::hash(plain_password.clone(), DEFAULT_COST).map_err(Error::from)?;

    let builder = User::builder(form.email.clone(), password)
        .with_first_name(first_name)
        .with_last_name(last_name)
        .with_account_type(AccountType::Professor);

    match database.run(move |c| builder.build().create(c)).await {
        Ok(_) => (),
        Err(Error::DatabaseDuplicateEntry) => {
            return Ok(Template::render(template_path, context.duplicate_data()))
        }
        Err(e) => return Err(e.into()),
    }

    mail.send_invite(address, &plain_password).await?;

    Ok(Template::render(template_path, context.success()))
}

#[derive(FromForm, Debug, Serialize, Clone)]
pub struct FormDataStudent {
    email: String,
    first_name: String,
    last_name: String,
    program: String,
    generation: u32,
    index_number: u32,
}

#[post("/users/create?account_type=Student", data = "<form>", rank = 0)]
pub async fn post_student(
    language: Script,
    administrator: Administrator<'_>,
    database: Database,
    mail: &State<Mail>,
    form: Form<FormDataStudent>,
) -> Result<Template, Status> {
    let user = administrator.0;

    let account_type = AccountTypeWithData::new_student(&database, Some(form.clone())).await?;

    let context = LayoutContext::new(language, user)
        .await?
        .with_account_type(Some(account_type));
    let template_path = "routes/administrator/users/create";

    let address = match Address::try_from(form.email.clone()) {
        Ok(address) => address,
        Err(_) => return Ok(Template::render(template_path, context.invalid_email())),
    };

    let plain_password: String = generate_random_password();
    let first_name = Some(form.first_name.clone()).filter(|s| s.is_empty());
    let last_name = Some(form.last_name.clone()).filter(|s| s.is_empty());

    let password = bcrypt::hash(plain_password.clone(), DEFAULT_COST).map_err(Error::from)?;

    let builder = User::builder(form.email.clone(), password)
        .with_first_name(first_name)
        .with_last_name(last_name)
        .with_account_type(AccountType::Student);

    let index_number = form.index_number;

    match database
        .run(move |c| {
            c.transaction(|c| {
                builder.build().create(c)?;
                let new_user = User::get_by_email(c, &form.email.clone())?;
                let program = Program::get_by_short_name(c, &form.program)?;
                let generation = Generation::get_by_year(c, form.generation)?;
                Index::create(c, program.id, generation.id, index_number, new_user.id)
            })
        })
        .await
    {
        Ok(_) => (),
        Err(Error::DatabaseDuplicateEntry) => {
            return Ok(Template::render(template_path, context.duplicate_data()))
        }
        Err(e) => return Err(e.into()),
    }

    mail.send_invite(address, &plain_password).await?;

    Ok(Template::render(template_path, context.success()))
}
