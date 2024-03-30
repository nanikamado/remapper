use env_logger::Env;
use kiri::evdev_keys::*;
use kiri::{
    AddLayer, Key, KeyConfig, KeyConfigRun, KeyInput, PairRemapEntry, RemapLayer, SingleRemapEntry,
};
use std::iter;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum StateGeta {
    Normal,
    JpInput,
    JpInputWithModifiers,
}

const THRESHOLD: u32 = 50;

#[allow(clippy::type_complexity)]
fn mk_config() -> RemapLayer<StateGeta> {
    use StateGeta::*;
    let singeta_config: &[(&[Key], &[Key])] = &[
        (&[KEY_A], &[KEY_N, KEY_O]),
        (&[KEY_S], &[KEY_T, KEY_O]),
        (&[KEY_D], &[KEY_K, KEY_A]),
        (&[KEY_F], &[KEY_N, KEY_N]),
        (&[KEY_G], &[KEY_X, KEY_T, KEY_U]),
        (&[KEY_D, KEY_H], &[KEY_H, KEY_E]),
        (&[KEY_D, KEY_J], &[KEY_A]),
        (&[KEY_D, KEY_SEMICOLON], &[KEY_E]),
        (&[KEY_D, KEY_N], &[KEY_S, KEY_E]),
        (&[KEY_D, KEY_M], &[KEY_N, KEY_E]),
        (&[KEY_D, KEY_COMMA], &[KEY_B, KEY_E]),
        (&[KEY_D, KEY_DOT], &[KEY_P, KEY_U]),
        (&[KEY_D, KEY_SLASH], &[KEY_V, KEY_U]),
        (&[KEY_D, KEY_Y], &[KEY_W, KEY_I]),
        (&[KEY_D, KEY_U], &[KEY_P, KEY_A]),
        (&[KEY_D, KEY_I], &[KEY_Y, KEY_O]),
        (&[KEY_D, KEY_O], &[KEY_M, KEY_I]),
        (&[KEY_D, KEY_P], &[KEY_W, KEY_E]),
        (&[KEY_D, KEY_LEFTBRACE], &[KEY_U, KEY_X, KEY_O]),
        (&[KEY_H], &[KEY_K, KEY_U]),
        (&[KEY_J], &[KEY_U]),
        (&[KEY_K], &[KEY_I]),
        (&[KEY_L], &[KEY_S, KEY_H, KEY_I]),
        (&[KEY_SEMICOLON], &[KEY_N, KEY_A]),
        (&[KEY_I, KEY_1], &[KEY_X, KEY_Y, KEY_U]),
        (&[KEY_I, KEY_2], &[KEY_B, KEY_Y, KEY_A]),
        (&[KEY_I, KEY_3], &[KEY_B, KEY_Y, KEY_U]),
        (&[KEY_I, KEY_4], &[KEY_B, KEY_Y, KEY_O]),
        (&[KEY_I, KEY_A], &[KEY_H, KEY_Y, KEY_O]),
        (&[KEY_I, KEY_F], &[KEY_K, KEY_Y, KEY_O]),
        (&[KEY_I, KEY_G], &[KEY_C, KEY_H, KEY_O]),
        (&[KEY_I, KEY_Q], &[KEY_H, KEY_Y, KEY_U]),
        (&[KEY_I, KEY_W], &[KEY_S, KEY_Y, KEY_U]),
        (&[KEY_I, KEY_E], &[KEY_S, KEY_Y, KEY_O]),
        (&[KEY_I, KEY_R], &[KEY_K, KEY_Y, KEY_U]),
        (&[KEY_I, KEY_T], &[KEY_C, KEY_H, KEY_U]),
        (&[KEY_I, KEY_Z], &[KEY_H, KEY_Y, KEY_A]),
        (&[KEY_I, KEY_C], &[KEY_S, KEY_H, KEY_A]),
        (&[KEY_I, KEY_V], &[KEY_K, KEY_Y, KEY_A]),
        (&[KEY_I, KEY_B], &[KEY_C, KEY_H, KEY_A]),
        (&[KEY_K, KEY_1], &[KEY_X, KEY_A]),
        (&[KEY_K, KEY_2], &[KEY_X, KEY_I]),
        (&[KEY_K, KEY_3], &[KEY_X, KEY_U]),
        (&[KEY_K, KEY_4], &[KEY_X, KEY_E]),
        (&[KEY_K, KEY_5], &[KEY_X, KEY_O]),
        (&[KEY_K, KEY_A], &[KEY_H, KEY_O]),
        (&[KEY_K, KEY_S], &[KEY_J, KEY_I]),
        (&[KEY_K, KEY_D], &[KEY_R, KEY_E]),
        (&[KEY_K, KEY_F], &[KEY_M, KEY_O]),
        (&[KEY_K, KEY_G], &[KEY_Y, KEY_U]),
        (&[KEY_K, KEY_Q], &[KEY_F, KEY_A]),
        (&[KEY_K, KEY_W], &[KEY_G, KEY_O]),
        (&[KEY_K, KEY_E], &[KEY_F, KEY_U]),
        (&[KEY_K, KEY_R], &[KEY_F, KEY_I]),
        (&[KEY_K, KEY_T], &[KEY_F, KEY_E]),
        (&[KEY_K, KEY_Z], &[KEY_D, KEY_U]),
        (&[KEY_K, KEY_X], &[KEY_Z, KEY_O]),
        (&[KEY_K, KEY_C], &[KEY_B, KEY_O]),
        (&[KEY_K, KEY_V], &[KEY_M, KEY_U]),
        (&[KEY_K, KEY_B], &[KEY_F, KEY_O]),
        (&[KEY_L, KEY_1], &[KEY_X, KEY_Y, KEY_A]),
        (&[KEY_L, KEY_2], &[KEY_M, KEY_Y, KEY_A]),
        (&[KEY_L, KEY_3], &[KEY_M, KEY_Y, KEY_U]),
        (&[KEY_L, KEY_4], &[KEY_M, KEY_Y, KEY_O]),
        (&[KEY_L, KEY_5], &[KEY_W, KEY_A]),
        (&[KEY_L, KEY_A], &[KEY_W, KEY_O]),
        (&[KEY_L, KEY_S], &[KEY_S, KEY_A]),
        (&[KEY_L, KEY_D], &[KEY_O]),
        (&[KEY_L, KEY_F], &[KEY_R, KEY_I]),
        (&[KEY_L, KEY_G], &[KEY_Z, KEY_U]),
        (&[KEY_L, KEY_Q], &[KEY_D, KEY_I]),
        (&[KEY_L, KEY_W], &[KEY_M, KEY_E]),
        (&[KEY_L, KEY_E], &[KEY_K, KEY_E]),
        (&[KEY_L, KEY_R], &[KEY_T, KEY_E, KEY_X, KEY_I]),
        (&[KEY_L, KEY_T], &[KEY_D, KEY_E, KEY_X, KEY_I]),
        (&[KEY_L, KEY_Z], &[KEY_Z, KEY_E]),
        (&[KEY_L, KEY_X], &[KEY_Z, KEY_A]),
        (&[KEY_L, KEY_C], &[KEY_G, KEY_I]),
        (&[KEY_L, KEY_V], &[KEY_R, KEY_O]),
        (&[KEY_L, KEY_B], &[KEY_N, KEY_U]),
        (&[KEY_N], &[KEY_T, KEY_E]),
        (&[KEY_M], &[KEY_T, KEY_A]),
        (&[KEY_COMMA], &[KEY_D, KEY_E]),
        (&[KEY_DOT], &[KEY_DOT]),
        (&[KEY_SLASH], &[KEY_B, KEY_U]),
        (&[KEY_O, KEY_1], &[KEY_X, KEY_Y, KEY_O]),
        (&[KEY_O, KEY_2], &[KEY_P, KEY_Y, KEY_A]),
        (&[KEY_O, KEY_3], &[KEY_P, KEY_Y, KEY_U]),
        (&[KEY_O, KEY_4], &[KEY_P, KEY_Y, KEY_O]),
        (&[KEY_O, KEY_A], &[KEY_R, KEY_Y, KEY_O]),
        (&[KEY_O, KEY_F], &[KEY_G, KEY_Y, KEY_O]),
        (&[KEY_O, KEY_G], &[KEY_N, KEY_Y, KEY_O]),
        (&[KEY_O, KEY_Q], &[KEY_R, KEY_Y, KEY_U]),
        (&[KEY_O, KEY_W], &[KEY_J, KEY_U]),
        (&[KEY_O, KEY_E], &[KEY_J, KEY_O]),
        (&[KEY_O, KEY_R], &[KEY_G, KEY_Y, KEY_U]),
        (&[KEY_O, KEY_T], &[KEY_N, KEY_Y, KEY_U]),
        (&[KEY_O, KEY_Z], &[KEY_R, KEY_Y, KEY_A]),
        (&[KEY_O, KEY_C], &[KEY_J, KEY_A]),
        (&[KEY_O, KEY_V], &[KEY_G, KEY_Y, KEY_A]),
        (&[KEY_O, KEY_B], &[KEY_N, KEY_Y, KEY_A]),
        (&[KEY_Q], &[KEY_MINUS]),
        (&[KEY_W], &[KEY_N, KEY_I]),
        (&[KEY_E], &[KEY_H, KEY_A]),
        (&[KEY_R], &[KEY_COMMA]),
        (&[KEY_T], &[KEY_C, KEY_H, KEY_I]),
        (&[KEY_S, KEY_H], &[KEY_B, KEY_I]),
        (&[KEY_S, KEY_J], &[KEY_R, KEY_A]),
        (&[KEY_S, KEY_SEMICOLON], &[KEY_S, KEY_O]),
        (&[KEY_S, KEY_N], &[KEY_W, KEY_A]),
        (&[KEY_S, KEY_M], &[KEY_D, KEY_A]),
        (&[KEY_S, KEY_COMMA], &[KEY_P, KEY_I]),
        (&[KEY_S, KEY_DOT], &[KEY_P, KEY_O]),
        (&[KEY_S, KEY_SLASH], &[KEY_T, KEY_Y, KEY_E]),
        (&[KEY_S, KEY_Y], &[KEY_S, KEY_Y, KEY_E]),
        (&[KEY_S, KEY_U], &[KEY_P, KEY_E]),
        (&[KEY_S, KEY_I], &[KEY_D, KEY_O]),
        (&[KEY_S, KEY_O], &[KEY_Y, KEY_A]),
        (&[KEY_S, KEY_P], &[KEY_J, KEY_E]),
        (&[KEY_Y], &[KEY_G, KEY_U]),
        (&[KEY_U], &[KEY_B, KEY_A]),
        (&[KEY_I], &[KEY_K, KEY_O]),
        (&[KEY_O], &[KEY_G, KEY_A]),
        (&[KEY_P], &[KEY_H, KEY_I]),
        (&[KEY_LEFTBRACE], &[KEY_G, KEY_E]),
        (&[KEY_Z], &[KEY_S, KEY_U]),
        (&[KEY_X], &[KEY_M, KEY_A]),
        (&[KEY_C], &[KEY_K, KEY_I]),
        (&[KEY_V], &[KEY_R, KEY_U]),
        (&[KEY_B], &[KEY_T, KEY_U]),
    ];
    let mut singeta_config: Vec<(&[StateGeta], &[Key], &[Key], Option<StateGeta>)> = singeta_config
        .iter()
        .map(|(i, o)| -> (&[StateGeta], _, _, _) { (&[JpInput], *i, *o, None) })
        .collect();
    let key_config_r: &[(&[StateGeta], &[Key], &[Key], Option<StateGeta>)] = &[
        (&[JpInput], &[KEY_R, KEY_G], &[KEY_SLASH], None),
        (
            &[JpInput],
            &[KEY_H, KEY_J],
            &[KEY_RIGHTBRACE, KEY_BACKSLASH, KEY_RIGHT],
            None,
        ),
        (&[Normal], &[KEY_J, KEY_K], &[KEY_RIGHTBRACE], None),
        (&[Normal], &[KEY_D, KEY_SEMICOLON], &[KEY_END], None),
        (&[Normal], &[KEY_A, KEY_K], &[KEY_HOME], None),
        (&[Normal], &[KEY_F, KEY_SEMICOLON], &[KEY_END], None),
        (&[Normal], &[KEY_A, KEY_J], &[KEY_HOME], None),
        (
            &[Normal, JpInput, JpInputWithModifiers],
            &[KEY_F15],
            &[KEY_GRAVE],
            None,
        ),
    ];
    let key_config_r = {
        let mut k = key_config_r.to_vec();
        k.append(&mut singeta_config);
        k
    };
    let pair_keys_with_modifiers_config: &[(&[StateGeta], [Key; 2], Vec<_>, Option<StateGeta>)] = &[
        (
            &[Normal, JpInput],
            [KEY_J, KEY_N],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_SLASH),
                KeyInput::release(KEY_SLASH),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[Normal, JpInput],
            [KEY_H, KEY_B],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_6),
                KeyInput::release(KEY_6),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[Normal, JpInput],
            [KEY_F, KEY_V],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_1),
                KeyInput::release(KEY_1),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[Normal, JpInput],
            [KEY_F, KEY_B],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_1),
                KeyInput::release(KEY_1),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[JpInput],
            [KEY_F, KEY_G],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_8),
                KeyInput::release(KEY_8),
                KeyInput::press(KEY_9),
                KeyInput::release(KEY_9),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[Normal],
            [KEY_D, KEY_F],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_8),
                KeyInput::release(KEY_8),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[Normal],
            [KEY_F, KEY_G],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_9),
                KeyInput::release(KEY_9),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[Normal],
            [KEY_K, KEY_L],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_RO),
                KeyInput::release(KEY_RO),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[Normal],
            [KEY_E, KEY_O],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_7),
                KeyInput::release(KEY_7),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[Normal],
            [KEY_F, KEY_J],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_2),
                KeyInput::release(KEY_2),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[Normal],
            [KEY_D, KEY_K],
            vec![
                KeyInput::press(KEY_LEFTSHIFT),
                KeyInput::press(KEY_1),
                KeyInput::release(KEY_1),
                KeyInput::release(KEY_LEFTSHIFT),
            ],
            None,
        ),
        (
            &[Normal],
            [KEY_D, KEY_S],
            vec![
                KeyInput::press(KEY_NUMERIC_0),
                KeyInput::press(KEY_LEFTMETA),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
                KeyInput::release(KEY_LEFTMETA),
            ],
            Some(JpInput),
        ),
        (
            &[JpInput],
            [KEY_D, KEY_S],
            vec![
                KeyInput::press(KEY_KATAKANAHIRAGANA),
                KeyInput::release(KEY_KATAKANAHIRAGANA),
                KeyInput::press(KEY_LEFTMETA),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
                KeyInput::release(KEY_LEFTMETA),
                KeyInput::press(KEY_NUMERIC_1),
            ],
            Some(Normal),
        ),
    ];
    let modifiers = [
        KEY_LEFTCTRL,
        KEY_LEFTMETA,
        KEY_LEFTALT,
        KEY_LEFTSHIFT,
        KEY_RIGHTCTRL,
        KEY_RIGHTMETA,
        KEY_RIGHTALT,
        KEY_RIGHTSHIFT,
    ];
    let modifiers_trans = modifiers
        .iter()
        .flat_map(|key| {
            [
                (JpInput, KeyInput::press(*key), Some(JpInputWithModifiers)),
                (JpInputWithModifiers, KeyInput::release(*key), Some(JpInput)),
            ]
            .map(|(c, i, t)| SingleRemapEntry {
                condition: c,
                input: i,
                output: vec![i],
                transition: t.unwrap_or(c),
            })
        })
        .collect::<Vec<_>>();
    RemapLayer {
        pair_remap_entries: key_config_r
            .iter()
            .filter(|(_, i, _, _)| i.len() == 2)
            .flat_map(|(cs, i, o, t)| {
                cs.iter().flat_map(move |c| {
                    PairRemapEntry {
                        condition: *c,
                        input: [KeyInput::press(i[0]), KeyInput::press(i[1])],
                        output: o
                            .iter()
                            .flat_map(|key| [KeyInput::press(*key), KeyInput::release(*key)])
                            .collect(),
                        transition: t.unwrap_or(*c),
                        threshold: THRESHOLD,
                    }
                    .order_insensitive()
                })
            })
            .chain(
                pair_keys_with_modifiers_config
                    .iter()
                    .flat_map(|(cs, i, o, t)| {
                        cs.iter().flat_map(move |c| {
                            PairRemapEntry {
                                condition: *c,
                                input: i.map(KeyInput::press),
                                output: o.clone(),
                                transition: t.unwrap_or(*c),
                                threshold: THRESHOLD,
                            }
                            .order_insensitive()
                        })
                    }),
            )
            .collect(),
        single_remap_entries: key_config_r
            .iter()
            .filter(|(_, i, _, _)| i.len() == 1)
            .flat_map(|(cs, i, o, t)| {
                cs.iter()
                    .map(move |c| SingleRemapEntry {
                        condition: *c,
                        input: KeyInput::press(i[0]),
                        output: (*o)
                            .iter()
                            .flat_map(|key| [KeyInput::press(*key), KeyInput::release(*key)])
                            .collect::<Vec<_>>(),
                        transition: t.unwrap_or(*c),
                    })
                    .chain(cs.iter().map(move |c| SingleRemapEntry {
                        condition: *c,
                        input: KeyInput::release(i[0]),
                        output: Vec::new(),
                        transition: t.unwrap_or(*c),
                    }))
            })
            .chain(modifiers_trans)
            .collect(),
        layer_name: "big config",
        initial_state: Normal,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum StateGrave {
    Normal,
    Grave,
    Grave1,
    Grave2,
}

fn config_grave_arrow() -> RemapLayer<StateGrave> {
    use StateGrave::*;
    let grave_side: [(_, _, &[Key], _); 8] = [
        (Grave, KEY_J, &[KEY_LEFTMETA], KEY_PAGEUP),
        (Grave, KEY_L, &[KEY_LEFTMETA], KEY_PAGEDOWN),
        (Grave1, KEY_J, &[KEY_LEFTMETA, KEY_LEFTSHIFT], KEY_PAGEUP),
        (Grave1, KEY_L, &[KEY_LEFTMETA, KEY_LEFTSHIFT], KEY_PAGEDOWN),
        (Grave2, KEY_J, &[KEY_LEFTMETA, KEY_LEFTSHIFT], KEY_LEFT),
        (Grave2, KEY_L, &[KEY_LEFTMETA, KEY_LEFTSHIFT], KEY_RIGHT),
        (Grave2, KEY_I, &[KEY_LEFTMETA, KEY_LEFTSHIFT], KEY_UP),
        (Grave2, KEY_K, &[KEY_LEFTMETA, KEY_LEFTSHIFT], KEY_DOWN),
    ];
    let grave_side = grave_side.iter().map(|&(c, i, o1, o2)| SingleRemapEntry {
        condition: c,
        input: KeyInput::press(i),
        output: o1
            .iter()
            .map(|o1| KeyInput::press(*o1))
            .chain(vec![KeyInput::press(o2), KeyInput::release(o2)])
            .chain(o1.iter().map(|o1| KeyInput::release(*o1)))
            .collect(),
        transition: c,
    });
    let single_remap_entries: &[(&[StateGrave], KeyInput, &[KeyInput], StateGrave)] = &[
        (&[Normal, Grave], KeyInput::press(KEY_GRAVE), &[], Grave),
        (
            &[Grave, Grave1, Grave2],
            KeyInput::press(KEY_1),
            &[],
            Grave1,
        ),
        (
            &[Grave, Grave1, Grave2],
            KeyInput::press(KEY_2),
            &[],
            Grave2,
        ),
        (
            &[Normal, Grave, Grave1, Grave2],
            KeyInput::release(KEY_GRAVE),
            &[],
            Normal,
        ),
        (&[Grave1], KeyInput::release(KEY_1), &[], Grave),
        (&[Grave1], KeyInput::release(KEY_2), &[], Grave),
    ];
    let single_hotkeys = single_remap_entries.iter().flat_map(|(c, i, o, t)| {
        c.iter().map(move |c| SingleRemapEntry {
            condition: *c,
            input: *i,
            output: o.to_vec(),
            transition: *t,
        })
    });
    RemapLayer {
        pair_remap_entries: Vec::new(),
        single_remap_entries: single_hotkeys.into_iter().chain(grave_side).collect(),
        layer_name: "grave arrows",
        initial_state: Normal,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum StateCapsLock {
    Normal,
    CL,
    Clw,
    Cle,
    Clr,
    Clf,
    ClTab,
}

fn config_caps_lock_arrow() -> RemapLayer<StateCapsLock> {
    use StateCapsLock::*;
    let capslock_side: [(_, _, &[_], _); 33] = [
        (CL, KEY_I, &[], KEY_UP),
        (CL, KEY_J, &[], KEY_LEFT),
        (CL, KEY_K, &[], KEY_DOWN),
        (CL, KEY_L, &[], KEY_RIGHT),
        (CL, KEY_GRAVE, &[], KEY_F15),
        (CL, KEY_ENTER, &[KEY_LEFTCTRL], KEY_S),
        (CL, KEY_N, &[KEY_LEFTCTRL], KEY_C),
        (CL, KEY_M, &[KEY_LEFTCTRL], KEY_V),
        (CL, KEY_U, &[KEY_LEFTCTRL], KEY_Z),
        (CL, KEY_O, &[KEY_LEFTCTRL], KEY_Y),
        (CL, KEY_DOT, &[KEY_LEFTCTRL], KEY_DOT),
        (CL, KEY_P, &[KEY_LEFTCTRL], KEY_P),
        (CL, KEY_COMMA, &[KEY_LEFTCTRL], KEY_F8),
        (CL, KEY_H, &[], KEY_ESC),
        (CL, KEY_LEFTBRACE, &[KEY_LEFTCTRL, KEY_LEFTALT], KEY_MINUS),
        (CL, KEY_RIGHTBRACE, &[KEY_LEFTSHIFT, KEY_LEFTCTRL], KEY_RO),
        (CL, KEY_BACKSLASH, &[KEY_LEFTSHIFT, KEY_LEFTCTRL], KEY_N),
        (Cle, KEY_I, &[KEY_LEFTCTRL], KEY_UP),
        (Cle, KEY_J, &[KEY_LEFTCTRL], KEY_LEFT),
        (Cle, KEY_K, &[KEY_LEFTCTRL], KEY_DOWN),
        (Cle, KEY_L, &[KEY_LEFTCTRL], KEY_RIGHT),
        (Clr, KEY_I, &[KEY_LEFTMETA], KEY_I),
        (Clr, KEY_J, &[KEY_LEFTMETA], KEY_J),
        (Clr, KEY_K, &[KEY_LEFTMETA], KEY_K),
        (Clr, KEY_L, &[KEY_LEFTMETA], KEY_L),
        (Clf, KEY_J, &[], KEY_HOME),
        (Clf, KEY_L, &[], KEY_END),
        (Clf, KEY_I, &[KEY_LEFTCTRL], KEY_F10),
        (Clf, KEY_K, &[KEY_LEFTCTRL], KEY_F9),
        (Clw, KEY_J, &[KEY_LEFTCTRL], KEY_PAGEUP),
        (Clw, KEY_L, &[KEY_LEFTCTRL], KEY_PAGEDOWN),
        (ClTab, KEY_J, &[KEY_LEFTCTRL], KEY_PAGEUP),
        (ClTab, KEY_L, &[KEY_LEFTCTRL], KEY_PAGEDOWN),
    ];
    let capslock_side = capslock_side
        .iter()
        .map(|&(c, i, o1, o2)| SingleRemapEntry {
            condition: c,
            input: KeyInput::press(i),
            output: o1
                .iter()
                .map(|o1| KeyInput::press(*o1))
                .chain(vec![KeyInput::press(o2), KeyInput::release(o2)])
                .chain(o1.iter().map(|o1| KeyInput::release(*o1)))
                .collect(),
            transition: c,
        });
    let single_remap_entries: &[(&[StateCapsLock], KeyInput, &[KeyInput], StateCapsLock)] = &[
        (&[Normal, CL], KeyInput::press(KEY_CAPSLOCK), &[], CL),
        (
            &[CL, Cle, Clr, Clf, Clw, ClTab],
            KeyInput::press(KEY_E),
            &[],
            Cle,
        ),
        (
            &[CL, Cle, Clr, Clf, Clw, ClTab],
            KeyInput::press(KEY_R),
            &[],
            Clr,
        ),
        (
            &[CL, Cle, Clr, Clf, Clw, ClTab],
            KeyInput::press(KEY_F),
            &[],
            Clf,
        ),
        (
            &[CL, Cle, Clr, Clf, Clw, ClTab],
            KeyInput::press(KEY_W),
            &[],
            Clw,
        ),
        (
            &[CL, Cle, Clr, Clf, Clw, ClTab],
            KeyInput::press(KEY_TAB),
            &[],
            ClTab,
        ),
        (&[Cle], KeyInput::press(KEY_CAPSLOCK), &[], Cle),
        (&[Clr], KeyInput::press(KEY_CAPSLOCK), &[], Clr),
        (&[Clf], KeyInput::press(KEY_CAPSLOCK), &[], Clf),
        (&[Clw], KeyInput::press(KEY_CAPSLOCK), &[], Clw),
        (&[ClTab], KeyInput::press(KEY_CAPSLOCK), &[], ClTab),
        (
            &[Normal, CL, Cle, Clr, Clf, Clw, ClTab],
            KeyInput::release(KEY_CAPSLOCK),
            &[],
            Normal,
        ),
        (&[Cle], KeyInput::release(KEY_E), &[], CL),
        (&[Clr], KeyInput::release(KEY_R), &[], CL),
        (&[Clf], KeyInput::release(KEY_F), &[], CL),
        (&[Clw], KeyInput::release(KEY_W), &[], CL),
        (&[ClTab], KeyInput::release(KEY_TAB), &[], CL),
    ];
    let single_hotkeys = single_remap_entries.iter().flat_map(|(c, i, o, t)| {
        c.iter().map(move |c| SingleRemapEntry {
            condition: *c,
            input: *i,
            output: o.to_vec(),
            transition: *t,
        })
    });
    RemapLayer {
        pair_remap_entries: Vec::new(),
        single_remap_entries: single_hotkeys.chain(capslock_side).collect(),
        layer_name: "caps lock arrows",
        initial_state: Normal,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum StateSands {
    JpInput,
    Normal,
    Space,
    Shift,
}

fn config_sands() -> RemapLayer<StateSands> {
    use StateSands::*;
    #[allow(clippy::type_complexity)]
    let config: &[(&[StateSands], KeyInput, &[KeyInput], Option<StateSands>)] = &[
        (
            &[Normal, Space, Shift, JpInput],
            KeyInput::press(KEY_NUMERIC_0),
            &[],
            Some(JpInput),
        ),
        (
            &[Normal, Space, Shift, JpInput],
            KeyInput::press(KEY_NUMERIC_1),
            &[],
            Some(Normal),
        ),
        (
            &[Normal],
            KeyInput::press(KEY_SPACE),
            &[KeyInput::press(KEY_LEFTSHIFT)],
            Some(Space),
        ),
        (&[Space, Shift], KeyInput::press(KEY_SPACE), &[], None),
        (
            &[Space],
            KeyInput::release(KEY_SPACE),
            &[
                KeyInput::release(KEY_LEFTSHIFT),
                KeyInput::press(KEY_SPACE),
                KeyInput::release(KEY_SPACE),
            ],
            Some(Normal),
        ),
        (
            &[Shift],
            KeyInput::release(KEY_SPACE),
            &[KeyInput::release(KEY_LEFTSHIFT)],
            Some(Normal),
        ),
    ];
    let config = config.iter().flat_map(|(cs, i, o, t)| {
        cs.iter().map(move |c| SingleRemapEntry {
            condition: *c,
            input: *i,
            output: o.to_vec(),
            transition: t.unwrap_or(*c),
        })
    });
    let config2 = all_keys()
        .filter(|k| *k != KEY_SPACE)
        .map(|k| SingleRemapEntry {
            condition: Space,
            input: KeyInput::press(k),
            output: vec![KeyInput::press(k)],
            transition: Shift,
        });
    RemapLayer {
        pair_remap_entries: Vec::new(),
        single_remap_entries: config.chain(config2).collect(),
        layer_name: "SandS",
        initial_state: Normal,
    }
}

fn config_simple_remap() -> RemapLayer<()> {
    let key_config_r: &[(Key, Key)] = &[(KEY_HENKAN, KEY_ENTER), (KEY_MUHENKAN, KEY_BACKSPACE)];
    RemapLayer {
        pair_remap_entries: Vec::new(),
        single_remap_entries: key_config_r
            .iter()
            .map(|(i, o)| SingleRemapEntry {
                condition: (),
                input: KeyInput::press(*i),
                output: vec![KeyInput::press(*o)],
                transition: (),
            })
            .chain(key_config_r.iter().map(|(i, o)| SingleRemapEntry {
                condition: (),
                input: KeyInput::release(*i),
                output: vec![KeyInput::release(*o)],
                transition: (),
            }))
            .collect(),
        layer_name: "simple remap",
        initial_state: (),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum StateShiftRelease {
    Normal,
    Shift,
    ContinuousShift,
}

fn config_shift_release() -> RemapLayer<StateShiftRelease> {
    use StateShiftRelease::*;
    let ps = all_keys()
        .filter(|k| *k != KEY_LEFTSHIFT && *k != KEY_SPACE)
        .map(|k| PairRemapEntry {
            condition: ContinuousShift,
            input: [KeyInput::press(k), KeyInput::release(KEY_LEFTSHIFT)],
            output: vec![KeyInput::release(KEY_LEFTSHIFT), KeyInput::press(k)],
            transition: Normal,
            threshold: 80,
        })
        .collect();
    let ss = all_keys()
        .filter(|k| *k != KEY_LEFTSHIFT)
        .map(|k| SingleRemapEntry {
            condition: Shift,
            input: KeyInput::press(k),
            output: vec![KeyInput::press(k)],
            transition: ContinuousShift,
        });
    RemapLayer {
        pair_remap_entries: ps,
        single_remap_entries: ss
            .chain(iter::once(SingleRemapEntry {
                condition: Normal,
                input: KeyInput::press(KEY_LEFTSHIFT),
                output: vec![KeyInput::press(KEY_LEFTSHIFT)],
                transition: Shift,
            }))
            .chain(iter::once(SingleRemapEntry {
                condition: Shift,
                input: KeyInput::release(KEY_LEFTSHIFT),
                output: vec![KeyInput::release(KEY_LEFTSHIFT)],
                transition: Normal,
            }))
            .chain(iter::once(SingleRemapEntry {
                condition: ContinuousShift,
                input: KeyInput::release(KEY_LEFTSHIFT),
                output: vec![KeyInput::release(KEY_LEFTSHIFT)],
                transition: Normal,
            }))
            .collect(),
        layer_name: "shift release",
        initial_state: Normal,
    }
}

// fn config_suppress_chattering() -> RemapLayer<()> {
//     let pair_remap_entries = all_keys()
//         .map(|k| PairRemapEntry {
//             condition: (),
//             input: [KeyInput::release(k), KeyInput::press(k)],
//             output: Vec::new(),
//             transition: (),
//             threshold: 20,
//         })
//         .collect();
//     RemapLayer {
//         pair_remap_entries,
//         single_remap_entries: Vec::new(),
//         layer_name: "suppress chattering",
//         initial_state: (),
//     }
// }

fn config_gc() -> RemapLayer<()> {
    let garbages = &[KEY_NUMERIC_0, KEY_NUMERIC_1];
    RemapLayer {
        pair_remap_entries: Vec::new(),
        single_remap_entries: garbages
            .iter()
            .map(|k| SingleRemapEntry {
                condition: (),
                input: KeyInput::press(*k),
                output: Vec::new(),
                transition: (),
            })
            .collect(),
        layer_name: "gc",
        initial_state: (),
    }
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .format_timestamp_millis()
        .init();
    KeyConfig::default()
        // .add_layer(config_suppress_chattering())
        .add_layer(config_simple_remap())
        .add_layer(config_caps_lock_arrow())
        .add_layer(config_grave_arrow())
        .add_layer(mk_config())
        .add_layer(config_sands())
        .add_layer(config_shift_release())
        .add_layer(config_gc())
        .run();
}
