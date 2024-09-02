// errors_mng.
pub trait Error: Debug + Display {
	fn description(&self) -> &str {

	}

	fn cause(&self) -> Option<&dyn Error> {
		
	}
}