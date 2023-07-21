// Topic: Result & the question work operator
//
// Program requirements:
// * Write the body of the `authorize` function. The steps to authorize a user are:
//   1. Connect to the database
//   2. Find the employee with the `get_employee` database function
//   3. Get a keycard with the `get_keycard` database function
//   4. Determine if the keycard's `access_level` is sufficient using the `required_access_level` function
//      implemented on `ProtectedLocation`
//      * Higher `access_level` values grant more access to `ProtectedLocations`, 1000 can access 1000 and lower, 800
//      * can access 500 but nor 1000, ...
// * Run the program after writing your `authorize` function. Expected outputs:
//   - Ok(allow)
//   -Ok(deny)
//   -Err("<Name> does not have a keycard")
// * Use the question mark operator within the `authorize` function
//
// Notes:
// * Only the `authorize` function should be changed. Everything else can remain unmodified

#[derive(Clone, Copy, Debug)]
enum ProtectedLocation {
    All,
    Office,
    Warehouse,
}

impl ProtectedLocation {
    fn required_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500,
        }
    }
}

#[derive(Debug)]
struct Database;

impl Database {
    fn connect() -> Result<Self, String> {
        Ok(Database)
    }

    fn find_employee(&self, name: &str) -> Result<Employee, String> {
        match name {
            "Anita" => Ok(Employee::new("Anita")),
            "Brody" => Ok(Employee::new("Brody")),
            "Catherine" => Ok(Employee::new("Catherine")),
            _ => Err(String::from("employee nor found")),
        }
    }

    fn get_keycard(&self, employee: &Employee) -> Result<Keycard, String> {
        match employee.name.as_str() {
            "Anita" => Ok(Keycard::new(1000)),
            "Brody" => Ok(Keycard::new(500)),
            other => Err(format!("{other} doesn't have a keycard")),
        }
    }
}

#[derive(Clone, Debug)]
struct Employee {
    name: String,
}

impl Employee {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

#[derive(Debug)]
struct Keycard {
    access_level: u16,
}

impl Keycard {
    fn new(access_level: u16) -> Self {
        Self { access_level }
    }
}

#[derive(Clone, Copy, Debug)]
enum AuthorizationStatus {
    Allow,
    Deny,
}

fn authorize(
    employee_name: &str,
    location: ProtectedLocation,
) -> Result<AuthorizationStatus, String> {
    let connection = Database::connect()?;
    let found_employee = Database::find_employee(&connection, employee_name)?;
    let keycard = Database::get_keycard(&connection, &found_employee)?;

    if keycard.access_level >= location.required_access_level() {
        Ok(AuthorizationStatus::Allow)
    } else {
        Ok(AuthorizationStatus::Deny)
    }
}

fn main() {
    let anita_auth = authorize("Anita", ProtectedLocation::Warehouse);
    let brody_auth = authorize("Brody", ProtectedLocation::Office);
    let catherine_auth = authorize("Catherine", ProtectedLocation::All);

    println!("{:?}", anita_auth);
    println!("{:?}", brody_auth);
    println!("{:?}", catherine_auth);
}
