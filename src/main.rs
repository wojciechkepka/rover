use druid::widget::prelude::*;
use druid::widget::{Button, Flex, Label, List, Scroll, TextBox};
use druid::{
    im::Vector, AppLauncher, Data, Lens, LensExt, LocalizedString, PlatformError, Widget,
    WidgetExt, WindowDesc,
};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Default, Debug, Data, Lens)]
pub struct AppData {
    path: String,
    files: Files,
    err: String,
}

#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Files {
    pub files: Vector<File>,
}
impl Files {
    pub fn read_path<P: AsRef<Path>>(path: P) -> Files {
        let mut files = Vector::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    files.push_back(File::from(entry));
                }
            }
        }

        Files { files }
    }
}

#[derive(Clone, Default, Debug, Lens, Data)]
pub struct File {
    pub name: String,
    pub path: String,
    pub size: u64,
}
impl From<fs::DirEntry> for File {
    fn from(entry: fs::DirEntry) -> Self {
        File {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path().to_string_lossy().to_string(),
            size: entry.metadata().map(|m| m.len()).unwrap_or_default(),
        }
    }
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(main_ui);
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(AppData::default())
}

fn load_data(_ctx: &mut EventCtx, data: &mut AppData, _env: &Env) {
    data.files = Files::read_path(&data.path);
}

fn main_ui() -> impl Widget<AppData> {
    let path_tb = TextBox::new()
        .with_placeholder("Enter a path...")
        .with_text_size(12.0)
        .fix_width(300.0)
        .lens(AppData::path);

    let btn = Label::new("Go").padding(5.0).center().on_click(load_data);

    Flex::column()
        .with_child(Flex::row().with_child(path_tb).with_child(btn))
        .with_child(files_view())
}

fn files_view() -> impl Widget<AppData> {
    Scroll::new(List::new(file_view).lens(AppData::files.then(Files::files)))
}

fn file_view() -> impl Widget<File> {
    let name = Label::raw().with_text_size(16.0).lens(File::path);
    let size = Label::dynamic(|f: &File, _| f.size.to_string()).with_text_size(16.0);
    Flex::row().with_child(name).with_child(size)
}
