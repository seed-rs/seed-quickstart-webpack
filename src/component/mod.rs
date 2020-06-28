mod button;

pub use button::view as Button;

pub mod todo_input;
// pub use input::{view as Input, InputMsg, update as InputUpdate};
// pub use input::view as TodoInput;

pub mod page;
// pub use page::view as Page;

pub mod header;
// pub use header::view as Header;

mod body;
pub use body::view as Body;

mod footer;
pub use footer::view as Footer;