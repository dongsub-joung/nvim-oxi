use derive_builder::Builder;
use nvim_types::{object::Object, string::String as NvimString};

use crate::lua::LuaFnMut;

/// Options passed to `Buffer::set_keymap`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default)]
pub struct SetKeymapOpts {
    #[builder(setter(custom))]
    callback: Option<LuaFnMut<(), ()>>,

    #[builder(setter(into, strip_option))]
    desc: Option<NvimString>,

    #[builder(setter(strip_option))]
    expr: Option<bool>,

    #[builder(setter(strip_option))]
    noremap: Option<bool>,

    #[builder(setter(strip_option))]
    nowait: Option<bool>,

    #[builder(setter(strip_option))]
    script: Option<bool>,

    #[builder(setter(strip_option))]
    silent: Option<bool>,

    #[builder(setter(strip_option))]
    unique: Option<bool>,
}

impl SetKeymapOpts {
    #[inline(always)]
    pub fn builder() -> SetKeymapOptsBuilder {
        SetKeymapOptsBuilder::default()
    }
}

impl SetKeymapOptsBuilder {
    pub fn callback<F>(&mut self, fun: F) -> &mut Self
    where
        F: FnMut(()) -> crate::Result<()> + 'static,
    {
        self.callback = Some(Some(fun.into()));
        self
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Default, Debug)]
pub(crate) struct KeyDict_keymap {
    callback: Object,
    desc: Object,
    expr: Object,
    noremap: Object,
    nowait: Object,
    script: Object,
    silent: Object,
    unique: Object,
}

impl<'a> From<&'a SetKeymapOpts> for KeyDict_keymap {
    fn from(opts: &SetKeymapOpts) -> Self {
        Self {
            callback: opts.callback.into(),
            desc: opts.desc.clone().into(),
            expr: opts.expr.into(),
            noremap: opts.noremap.into(),
            nowait: opts.nowait.into(),
            script: opts.script.into(),
            silent: opts.silent.into(),
            unique: opts.unique.into(),
        }
    }
}