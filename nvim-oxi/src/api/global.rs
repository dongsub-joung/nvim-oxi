use nvim_types::{
    Array,
    Dictionary,
    Error as NvimError,
    Integer,
    Object,
    String as NvimString,
};

use super::ffi::global::*;
use super::opts::CreateCommandOpts;
use super::types::Mode;
use crate::{
    api::Buffer,
    lua::LUA_INTERNAL_CALL,
    object::{FromObject, ToObject},
    Result,
};

/// Binding to `nvim_chan_send`.
pub fn chan_send(chan: impl Into<Integer>, data: &str) -> Result<()> {
    let mut err = NvimError::new();
    let data = NvimString::from(data);
    unsafe { nvim_chan_send(chan.into(), data.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_create_buf`.
///
/// Creates a new, empty, unnamed buffer.
pub fn create_buf(is_listed: bool, is_scratch: bool) -> Result<Buffer> {
    let mut err = NvimError::new();
    let handle = unsafe { nvim_create_buf(is_listed, is_scratch, &mut err) };
    err.into_err_or_else(|| handle.into())
}

/// Binding to `nvim_create_user_command`.
pub fn create_user_command<Value>(
    name: &str,
    command: Value,
    opts: &CreateCommandOpts,
) -> Result<()>
where
    Value: ToObject,
{
    let name = NvimString::from(name);
    let command = command.to_obj()?;
    let mut err = NvimError::new();
    unsafe {
        nvim_create_user_command(
            name.non_owning(),
            command.non_owning(),
            &opts.into(),
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_del_current_line`
pub fn del_current_line() -> Result<()> {
    let mut err = NvimError::new();
    unsafe { nvim_del_current_line(&mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_del_keymap`.
pub fn del_keymap(mode: Mode, lhs: &str) -> Result<()> {
    let mode = NvimString::from(mode);
    let lhs = NvimString::from(lhs);
    let mut err = NvimError::new();
    unsafe {
        nvim_del_keymap(
            LUA_INTERNAL_CALL,
            mode.non_owning(),
            lhs.non_owning(),
            &mut err,
        )
    };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_del_mark`.
pub fn del_mark(name: char) -> Result<bool> {
    let name = NvimString::from(name);
    let mut err = NvimError::new();
    let res = unsafe { nvim_del_mark(name.non_owning(), &mut err) };
    err.into_err_or_else(|| res)
}

/// Binding to `nvim_del_user_command`.
pub fn del_user_command(name: &str) -> Result<()> {
    let name = NvimString::from(name);
    let mut err = NvimError::new();
    unsafe { nvim_del_user_command(name.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_del_var`.
///
/// Removes a global (`g:`) variable.
pub fn del_var(name: &str) -> Result<()> {
    let name = NvimString::from(name);
    let mut err = NvimError::new();
    unsafe { nvim_del_var(name.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_echo`.
///
/// Echoes a message to the Neovim message area.
pub fn echo<Text, HlGroup, Chunks>(chunks: Chunks, history: bool) -> Result<()>
where
    Text: std::fmt::Display,
    HlGroup: AsRef<str>,
    Chunks: IntoIterator<Item = (Text, Option<HlGroup>)>,
{
    let chunks = chunks
        .into_iter()
        .map(|(text, hlgroup)| {
            Array::from_iter([
                Object::from(text.to_string()),
                Object::from(hlgroup.map(|hl| hl.as_ref().to_owned())),
            ])
        })
        .collect::<Array>();

    let mut err = NvimError::new();
    let opts = Dictionary::new();
    unsafe {
        nvim_echo(chunks.non_owning(), history, opts.non_owning(), &mut err)
    };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_err_write`.
///
/// Writes a message to the Neovim error buffer. Does not append a newline
/// (`"\n"`); the message gets buffered and won't be displayed until a linefeed
/// is written.
pub fn err_write(str: &str) {
    unsafe { nvim_err_write(NvimString::from(str).non_owning()) }
}

/// Binding to `nvim_err_writeln`.
///
/// Writes a message to the Neovim error buffer. Appends a newline (`"\n"`), so
/// the buffer is flused and displayed.
pub fn err_writeln(str: &str) {
    unsafe { nvim_err_writeln(NvimString::from(str).non_owning()) }
}

// eval_statusline

/// Binding to `nvim_feedkeys`
pub fn feedkeys(keys: &str, mode: Mode, escape_ks: bool) {
    let keys = NvimString::from(keys);
    let mode = NvimString::from(mode);
    unsafe { nvim_feedkeys(keys.non_owning(), mode.non_owning(), escape_ks) }
}

// get_all_options_info

// get_api_info

// get_chan_info

/// Binding to `nvim_get_color_by_name`.
///
/// Returns the 24-bit RGB value of a `crate::api::get_color_map` color name or
/// "#rrggbb" hexadecimal string.
pub fn get_color_by_name(name: &str) -> u32 {
    let name = NvimString::from(name);
    let color = unsafe { nvim_get_color_by_name(name.non_owning()) };
    // TODO: don't panic
    color.try_into().expect("invalid argument")
}

// get_color_map

// get_commands

// get_context

/// Binding to `nvim_get_current_buf`.
///
/// Gets the current buffer.
pub fn get_current_buf() -> Buffer {
    unsafe { nvim_get_current_buf() }.into()
}

// get_current_line

// get_current_tabpage

// get_current_win

// get_hl_by_id

// get_hl_by_name

// get_hl_id_by_name

// get_keymap

// get_mark

/// Binding to `nvim_get_mode`.
pub fn get_mode() -> Dictionary {
    unsafe { nvim_get_mode() }
    // (
    //     dict.get("mode").expect("`mode` key is present"),
    //     dict.get("blocking").expect("`blocking` key is present"),
    // )
}

// get_option

// get_option_info

// get_option_value

// get_proc

// get_proc_children

// get_runtime_file

/// Binding to `nvim_get_var`.
///
/// Gets a global (`g:`) variable.
pub fn get_var<Value>(name: &str) -> Result<Value>
where
    Value: FromObject,
{
    let mut err = NvimError::new();
    let name = NvimString::from(name);
    let obj = unsafe { nvim_get_var(name.non_owning(), &mut err) };
    err.into_err_or_flatten(|| Value::from_obj(obj))
}

/// Binding to `nvim_get_vvar`.
///
/// Gets a `v:` variable.
pub fn get_vvar<Value>(name: &str) -> Result<Value>
where
    Value: FromObject,
{
    let name = NvimString::from(name);
    let mut err = NvimError::new();
    let obj = unsafe { nvim_get_vvar(name.non_owning(), &mut err) };
    err.into_err_or_flatten(|| Value::from_obj(obj))
}

// input

// input_mouse

// list_bufs

// list_chans

// list_runtime_paths

// list_tabpages

// list_uis

// list_wins

// load_context

// notify

// open_term

// out_write

// paste

// put

/// Binding to `nvim_replace_termcodes`.
///
/// Replaces terminal codes and keycodes (`<CR>`, `<Esc>`, ...) in a string
/// with the internal representation.
pub fn replace_termcodes<Codes: Into<NvimString>>(
    str: Codes,
    from_part: bool,
    do_lt: bool,
    special: bool,
) -> NvimString {
    let str = str.into();
    unsafe {
        nvim_replace_termcodes(str.non_owning(), from_part, do_lt, special)
    }
}

// select_popupmenu_item

// set_current_buf

// set_current_dir

// set_current_line

// set_current_tapage

// set_current_win

// set_hl

// set_keymap

// set_option

// set_option_value

/// Binding to `nvim_set_var`.
///
/// Sets a global (`g:`) variable.
pub fn set_var<Value>(name: &str, value: Value) -> Result<()>
where
    Value: ToObject,
{
    let name = NvimString::from(name);
    let value = value.to_obj()?;
    let mut err = NvimError::new();
    unsafe { nvim_set_var(name.non_owning(), value.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_set_vvar`.
///
/// Sets a `v:` variable, if it's not readonly.
pub fn set_vvar<Value>(name: &str, value: Value) -> Result<()>
where
    Value: ToObject,
{
    let name = NvimString::from(name);
    let value = value.to_obj()?;
    let mut err = NvimError::new();
    unsafe { nvim_set_vvar(name.non_owning(), value.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

// strwidth
