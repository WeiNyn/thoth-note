use crate::app::View;

pub enum Command {
    Quit,
    NextNote,
    PreviousNote,
    SwitchView(View),
    NewNote,
    SaveNote,
    DeleteNote,
    ScrollDown,
    ScrollUp,
    RenameNote,
    SubmitRename,
    CancelRename,
}
