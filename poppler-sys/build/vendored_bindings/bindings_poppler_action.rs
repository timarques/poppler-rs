/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
pub type guint8 = ::std::os::raw::c_uchar;
pub const PopplerActionType_POPPLER_ACTION_UNKNOWN: PopplerActionType = 0;
pub const PopplerActionType_POPPLER_ACTION_NONE: PopplerActionType = 1;
pub const PopplerActionType_POPPLER_ACTION_GOTO_DEST: PopplerActionType = 2;
pub const PopplerActionType_POPPLER_ACTION_GOTO_REMOTE: PopplerActionType = 3;
pub const PopplerActionType_POPPLER_ACTION_LAUNCH: PopplerActionType = 4;
pub const PopplerActionType_POPPLER_ACTION_URI: PopplerActionType = 5;
pub const PopplerActionType_POPPLER_ACTION_NAMED: PopplerActionType = 6;
pub const PopplerActionType_POPPLER_ACTION_MOVIE: PopplerActionType = 7;
pub const PopplerActionType_POPPLER_ACTION_RENDITION: PopplerActionType = 8;
pub const PopplerActionType_POPPLER_ACTION_OCG_STATE: PopplerActionType = 9;
pub const PopplerActionType_POPPLER_ACTION_JAVASCRIPT: PopplerActionType = 10;
pub type PopplerActionType = u32;
pub const PopplerDestType_POPPLER_DEST_UNKNOWN: PopplerDestType = 0;
pub const PopplerDestType_POPPLER_DEST_XYZ: PopplerDestType = 1;
pub const PopplerDestType_POPPLER_DEST_FIT: PopplerDestType = 2;
pub const PopplerDestType_POPPLER_DEST_FITH: PopplerDestType = 3;
pub const PopplerDestType_POPPLER_DEST_FITV: PopplerDestType = 4;
pub const PopplerDestType_POPPLER_DEST_FITR: PopplerDestType = 5;
pub const PopplerDestType_POPPLER_DEST_FITB: PopplerDestType = 6;
pub const PopplerDestType_POPPLER_DEST_FITBH: PopplerDestType = 7;
pub const PopplerDestType_POPPLER_DEST_FITBV: PopplerDestType = 8;
pub const PopplerDestType_POPPLER_DEST_NAMED: PopplerDestType = 9;
pub type PopplerDestType = u32;
pub const PopplerActionMovieOperation_POPPLER_ACTION_MOVIE_PLAY: PopplerActionMovieOperation = 0;
pub const PopplerActionMovieOperation_POPPLER_ACTION_MOVIE_PAUSE: PopplerActionMovieOperation = 1;
pub const PopplerActionMovieOperation_POPPLER_ACTION_MOVIE_RESUME: PopplerActionMovieOperation = 2;
pub const PopplerActionMovieOperation_POPPLER_ACTION_MOVIE_STOP: PopplerActionMovieOperation = 3;
pub type PopplerActionMovieOperation = u32;
pub const PopplerActionLayerAction_POPPLER_ACTION_LAYER_ON: PopplerActionLayerAction = 0;
pub const PopplerActionLayerAction_POPPLER_ACTION_LAYER_OFF: PopplerActionLayerAction = 1;
pub const PopplerActionLayerAction_POPPLER_ACTION_LAYER_TOGGLE: PopplerActionLayerAction = 2;
pub type PopplerActionLayerAction = u32;
pub type PopplerActionAny = _PopplerActionAny;
pub type PopplerActionGotoDest = _PopplerActionGotoDest;
pub type PopplerActionGotoRemote = _PopplerActionGotoRemote;
pub type PopplerActionLaunch = _PopplerActionLaunch;
pub type PopplerActionUri = _PopplerActionUri;
pub type PopplerActionNamed = _PopplerActionNamed;
pub type PopplerActionMovie = _PopplerActionMovie;
pub type PopplerActionRendition = _PopplerActionRendition;
pub type PopplerActionOCGState = _PopplerActionOCGState;
pub type PopplerActionJavascript = _PopplerActionJavascript;
#[repr(C)]
pub struct _PopplerDest {
    pub type_: PopplerDestType,
    pub page_num: ::std::os::raw::c_int,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
    pub top: f64,
    pub zoom: f64,
    pub named_dest: *mut gchar,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub __bindgen_padding_0: [u8; 7usize],
}
#[test]
fn bindgen_test_layout__PopplerDest() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerDest>(),
        64usize,
        concat!("Size of: ", stringify!(_PopplerDest))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerDest>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerDest))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerDest>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerDest),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerDest>())).page_num as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerDest),
            "::",
            stringify!(page_num)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerDest>())).left as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerDest),
            "::",
            stringify!(left)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerDest>())).bottom as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerDest),
            "::",
            stringify!(bottom)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerDest>())).right as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerDest),
            "::",
            stringify!(right)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerDest>())).top as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerDest),
            "::",
            stringify!(top)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerDest>())).zoom as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerDest),
            "::",
            stringify!(zoom)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerDest>())).named_dest as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerDest),
            "::",
            stringify!(named_dest)
        )
    );
}
impl _PopplerDest {
    #[inline]
    pub fn change_left(&self) -> guint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_change_left(&mut self, val: guint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn change_top(&self) -> guint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_change_top(&mut self, val: guint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn change_zoom(&self) -> guint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_change_zoom(&mut self, val: guint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        change_left: guint,
        change_top: guint,
        change_zoom: guint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let change_left: u32 = unsafe { ::std::mem::transmute(change_left) };
            change_left as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let change_top: u32 = unsafe { ::std::mem::transmute(change_top) };
            change_top as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let change_zoom: u32 = unsafe { ::std::mem::transmute(change_zoom) };
            change_zoom as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerActionLayer {
    pub action: PopplerActionLayerAction,
    pub layers: *mut GList,
}
#[test]
fn bindgen_test_layout__PopplerActionLayer() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerActionLayer>(),
        16usize,
        concat!("Size of: ", stringify!(_PopplerActionLayer))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerActionLayer>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerActionLayer))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionLayer>())).action as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionLayer),
            "::",
            stringify!(action)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionLayer>())).layers as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionLayer),
            "::",
            stringify!(layers)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerActionAny {
    pub type_: PopplerActionType,
    pub title: *mut gchar,
}
#[test]
fn bindgen_test_layout__PopplerActionAny() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerActionAny>(),
        16usize,
        concat!("Size of: ", stringify!(_PopplerActionAny))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerActionAny>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerActionAny))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionAny>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionAny),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionAny>())).title as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionAny),
            "::",
            stringify!(title)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerActionGotoDest {
    pub type_: PopplerActionType,
    pub title: *mut gchar,
    pub dest: *mut PopplerDest,
}
#[test]
fn bindgen_test_layout__PopplerActionGotoDest() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerActionGotoDest>(),
        24usize,
        concat!("Size of: ", stringify!(_PopplerActionGotoDest))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerActionGotoDest>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerActionGotoDest))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionGotoDest>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionGotoDest),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionGotoDest>())).title as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionGotoDest),
            "::",
            stringify!(title)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionGotoDest>())).dest as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionGotoDest),
            "::",
            stringify!(dest)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerActionGotoRemote {
    pub type_: PopplerActionType,
    pub title: *mut gchar,
    pub file_name: *mut gchar,
    pub dest: *mut PopplerDest,
}
#[test]
fn bindgen_test_layout__PopplerActionGotoRemote() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerActionGotoRemote>(),
        32usize,
        concat!("Size of: ", stringify!(_PopplerActionGotoRemote))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerActionGotoRemote>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerActionGotoRemote))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionGotoRemote>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionGotoRemote),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionGotoRemote>())).title as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionGotoRemote),
            "::",
            stringify!(title)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerActionGotoRemote>())).file_name as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionGotoRemote),
            "::",
            stringify!(file_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionGotoRemote>())).dest as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionGotoRemote),
            "::",
            stringify!(dest)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerActionLaunch {
    pub type_: PopplerActionType,
    pub title: *mut gchar,
    pub file_name: *mut gchar,
    pub params: *mut gchar,
}
#[test]
fn bindgen_test_layout__PopplerActionLaunch() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerActionLaunch>(),
        32usize,
        concat!("Size of: ", stringify!(_PopplerActionLaunch))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerActionLaunch>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerActionLaunch))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionLaunch>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionLaunch),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionLaunch>())).title as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionLaunch),
            "::",
            stringify!(title)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionLaunch>())).file_name as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionLaunch),
            "::",
            stringify!(file_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionLaunch>())).params as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionLaunch),
            "::",
            stringify!(params)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerActionUri {
    pub type_: PopplerActionType,
    pub title: *mut gchar,
    pub uri: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout__PopplerActionUri() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerActionUri>(),
        24usize,
        concat!("Size of: ", stringify!(_PopplerActionUri))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerActionUri>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerActionUri))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionUri>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionUri),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionUri>())).title as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionUri),
            "::",
            stringify!(title)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionUri>())).uri as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionUri),
            "::",
            stringify!(uri)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerActionNamed {
    pub type_: PopplerActionType,
    pub title: *mut gchar,
    pub named_dest: *mut gchar,
}
#[test]
fn bindgen_test_layout__PopplerActionNamed() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerActionNamed>(),
        24usize,
        concat!("Size of: ", stringify!(_PopplerActionNamed))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerActionNamed>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerActionNamed))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionNamed>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionNamed),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionNamed>())).title as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionNamed),
            "::",
            stringify!(title)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionNamed>())).named_dest as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionNamed),
            "::",
            stringify!(named_dest)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerActionMovie {
    pub type_: PopplerActionType,
    pub title: *mut gchar,
    pub operation: PopplerActionMovieOperation,
    pub movie: *mut PopplerMovie,
}
#[test]
fn bindgen_test_layout__PopplerActionMovie() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerActionMovie>(),
        32usize,
        concat!("Size of: ", stringify!(_PopplerActionMovie))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerActionMovie>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerActionMovie))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionMovie>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionMovie),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionMovie>())).title as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionMovie),
            "::",
            stringify!(title)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionMovie>())).operation as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionMovie),
            "::",
            stringify!(operation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionMovie>())).movie as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionMovie),
            "::",
            stringify!(movie)
        )
    );
}
#[repr(C)]
pub struct _PopplerActionRendition {
    pub type_: PopplerActionType,
    pub title: *mut gchar,
    pub op: gint,
    pub media: *mut PopplerMedia,
}
#[test]
fn bindgen_test_layout__PopplerActionRendition() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerActionRendition>(),
        32usize,
        concat!("Size of: ", stringify!(_PopplerActionRendition))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerActionRendition>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerActionRendition))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionRendition>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionRendition),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionRendition>())).title as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionRendition),
            "::",
            stringify!(title)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionRendition>())).op as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionRendition),
            "::",
            stringify!(op)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionRendition>())).media as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionRendition),
            "::",
            stringify!(media)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerActionOCGState {
    pub type_: PopplerActionType,
    pub title: *mut gchar,
    pub state_list: *mut GList,
}
#[test]
fn bindgen_test_layout__PopplerActionOCGState() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerActionOCGState>(),
        24usize,
        concat!("Size of: ", stringify!(_PopplerActionOCGState))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerActionOCGState>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerActionOCGState))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionOCGState>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionOCGState),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionOCGState>())).title as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionOCGState),
            "::",
            stringify!(title)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerActionOCGState>())).state_list as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionOCGState),
            "::",
            stringify!(state_list)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerActionJavascript {
    pub type_: PopplerActionType,
    pub title: *mut gchar,
    pub script: *mut gchar,
}
#[test]
fn bindgen_test_layout__PopplerActionJavascript() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerActionJavascript>(),
        24usize,
        concat!("Size of: ", stringify!(_PopplerActionJavascript))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerActionJavascript>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerActionJavascript))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionJavascript>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionJavascript),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionJavascript>())).title as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionJavascript),
            "::",
            stringify!(title)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerActionJavascript>())).script as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerActionJavascript),
            "::",
            stringify!(script)
        )
    );
}
#[repr(C)]
pub struct _PopplerAction {
    pub type_: __BindgenUnionField<PopplerActionType>,
    pub any: __BindgenUnionField<PopplerActionAny>,
    pub goto_dest: __BindgenUnionField<PopplerActionGotoDest>,
    pub goto_remote: __BindgenUnionField<PopplerActionGotoRemote>,
    pub launch: __BindgenUnionField<PopplerActionLaunch>,
    pub uri: __BindgenUnionField<PopplerActionUri>,
    pub named: __BindgenUnionField<PopplerActionNamed>,
    pub movie: __BindgenUnionField<PopplerActionMovie>,
    pub rendition: __BindgenUnionField<PopplerActionRendition>,
    pub ocg_state: __BindgenUnionField<PopplerActionOCGState>,
    pub javascript: __BindgenUnionField<PopplerActionJavascript>,
    pub bindgen_union_field: [u64; 4usize],
}
#[test]
fn bindgen_test_layout__PopplerAction() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerAction>(),
        32usize,
        concat!("Size of: ", stringify!(_PopplerAction))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerAction>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerAction))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAction>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAction),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAction>())).any as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAction),
            "::",
            stringify!(any)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAction>())).goto_dest as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAction),
            "::",
            stringify!(goto_dest)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAction>())).goto_remote as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAction),
            "::",
            stringify!(goto_remote)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAction>())).launch as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAction),
            "::",
            stringify!(launch)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAction>())).uri as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAction),
            "::",
            stringify!(uri)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAction>())).named as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAction),
            "::",
            stringify!(named)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAction>())).movie as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAction),
            "::",
            stringify!(movie)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAction>())).rendition as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAction),
            "::",
            stringify!(rendition)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAction>())).ocg_state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAction),
            "::",
            stringify!(ocg_state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAction>())).javascript as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAction),
            "::",
            stringify!(javascript)
        )
    );
}
extern "C" {
    pub fn poppler_action_get_type() -> GType;
}
extern "C" {
    pub fn poppler_action_free(action: *mut PopplerAction);
}
extern "C" {
    pub fn poppler_action_copy(action: *mut PopplerAction) -> *mut PopplerAction;
}
extern "C" {
    pub fn poppler_dest_get_type() -> GType;
}
extern "C" {
    pub fn poppler_dest_free(dest: *mut PopplerDest);
}
extern "C" {
    pub fn poppler_dest_copy(dest: *mut PopplerDest) -> *mut PopplerDest;
}
extern "C" {
    pub fn poppler_named_dest_from_bytestring(
        data: *const guint8,
        length: gsize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn poppler_named_dest_to_bytestring(
        named_dest: *const ::std::os::raw::c_char,
        length: *mut gsize,
    ) -> *mut guint8;
}
