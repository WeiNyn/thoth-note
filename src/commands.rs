use crate::app::View;

pub enum Command {
    Quit,
    NextNote,
    PreviousNote,
    SwitchView(View),
    NewNote,
    SaveNote,
    DeleteNote,
    // Add more commands as needed
}
