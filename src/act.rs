use crate::common::BinaryReader;

pub struct ActionFile {
    pub actions: Vec<Action>,
    pub sounds: Vec<String>,
}

pub struct Action {
    pub animations: Vec<Animation>,
    pub delay: f32,
}

pub struct Animation {
    pub layers: Vec<Layer>,
    pub sound: i32,
    pub positions: Vec<[i32; 2]>,
}

pub struct Layer {
    pub pos: [i32; 2],
    pub index: i32,
    pub is_mirror: i32,
    pub scale: [f32; 2],
    pub color: [f32; 4],
    pub angle: i32,
    pub spr_type: i32,
    pub width: i32,
    pub height: i32,
}


impl ActionFile {
    pub fn load(mut buf: BinaryReader) -> ActionFile {
        let header = buf.string(2);
        if header != "AC" {
            panic!("Invalig Action header: {}", header);
        }

        let version = buf.next_u8() as f32 + buf.next_u8() as f32 / 10f32;

        let action_acount = buf.next_u16() as usize;
        buf.skip(10);

        let mut actions: Vec<Action> = (0..action_acount).map(|i| {
            Action {
                animations: ActionFile::read_animations(&mut buf, version),
                delay: 150f32,
            }
        }).collect();
        let sounds = if version >= 2.1 {
            (0..buf.next_i32()).map(|_i| {
                buf.string(40)
            }).collect()
        } else { vec![] };
        if version >= 2.2 {
            actions.iter_mut().for_each(|a| {
                a.delay = buf.next_f32() * 25f32;
            });
        }
        return ActionFile { actions, sounds };
    }

    fn read_animations(buf: &mut BinaryReader, version: f32) -> Vec<Animation> {
        let animation_count = buf.next_u32() as usize;
        (0..animation_count).map(|_i| {
            let _unknown = buf.skip(32);
            Animation {
                layers: ActionFile::read_layers(buf, version),
                sound: if version >= 2.0 { buf.next_i32() } else { -1 },
                positions: if version >= 2.3 {
                    (0..buf.next_i32()).map(|_i| {
                        buf.skip(4);
                        let pos = [buf.next_i32(), buf.next_i32()];
                        buf.skip(4);
                        pos
                    }).collect()
                } else { vec![] },
            }
        }).collect()
    }

    fn read_layers(buf: &mut BinaryReader, version: f32) -> Vec<Layer> {
        let layer_count = buf.next_u32() as usize;
        (0..layer_count).map(|_i| {
            let pos = [buf.next_i32(), buf.next_i32()];
            let index = buf.next_i32();
            let is_mirror = buf.next_i32();
            let color = if version >= 2.0 {
                [
                    buf.next_u8() as f32 / 255.0,
                    buf.next_u8() as f32 / 255.0,
                    buf.next_u8() as f32 / 255.0,
                    buf.next_u8() as f32 / 255.0,
                ]
            } else { [1.0, 1.0, 1.0, 1.0] };
            let scale = if version >= 2.0 {
                let scale_0 = buf.next_f32();
                [
                    scale_0,
                    if version <= 2.3 { scale_0 } else { buf.next_f32() }
                ]
            } else { [1.0, 1.0] };
            let angle = if version >= 2.0 { buf.next_i32() } else { 0 };
            let spr_type = if version >= 2.0 { buf.next_i32() } else { 0 };
            let width = if version >= 2.5 { buf.next_i32() } else { 0 };
            let height = if version >= 2.5 { buf.next_i32() } else { 0 };

            Layer {
                pos,
                index,
                is_mirror,
                scale,
                color,
                angle,
                spr_type,
                width,
                height,
            }
        }).collect()
    }
}