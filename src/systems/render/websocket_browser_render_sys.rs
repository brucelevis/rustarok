use crate::components::BrowserClient;
use crate::configs::DevConfig;
use crate::effect::StrEffectId;
use crate::systems::render::render_command::{
    Circle3dRenderCommand, HorizontalTexture3dRenderCommand, ModelRenderCommand,
    Number3dRenderCommand, PartialCircle2dRenderCommand, Rectangle2dRenderCommand,
    Rectangle3dRenderCommand, RenderCommandCollector, Sprite3dRenderCommand, Text2dRenderCommand,
    Texture2dRenderCommand, Trimesh3dRenderCommand,
};
use crate::systems::{SystemFrameDurations, SystemVariables};
use byteorder::{LittleEndian, WriteBytesExt};
use nalgebra::{Matrix3, Matrix4, Vector2};
use specs::prelude::*;
use std::collections::VecDeque;

pub struct WebSocketBrowserRenderSystem {
    send_buffer: Vec<u8>,
}

impl WebSocketBrowserRenderSystem {
    pub fn new() -> WebSocketBrowserRenderSystem {
        WebSocketBrowserRenderSystem {
            send_buffer: Vec::<u8>::with_capacity(60 * 1024),
        }
    }
}

impl<'a> specs::System<'a> for WebSocketBrowserRenderSystem {
    type SystemData = (
        specs::ReadStorage<'a, RenderCommandCollector>,
        specs::WriteStorage<'a, BrowserClient>,
        specs::WriteExpect<'a, SystemFrameDurations>,
        specs::ReadExpect<'a, SystemVariables>,
        specs::ReadExpect<'a, DevConfig>,
    );

    fn run(
        &mut self,
        (
            render_commands_storage,
            mut browser_client_storage,
            mut system_benchmark,
            system_vars,
            dev_configs,
        ): Self::SystemData,
    ) {
        let _stopwatch = system_benchmark.start_measurement("WebSocketBrowserRenderSystem");
        if system_vars.tick % dev_configs.network.send_render_data_every_nth_frame.max(1) != 0 {
            return;
        }

        for (render_commands, browser) in
            (&render_commands_storage, &mut browser_client_storage).join()
        {
            if browser.next_send_at.has_not_passed_yet(system_vars.time) {
                continue;
            }
            self.send_buffer.clear();

            WebSocketBrowserRenderSystem::write4x4(
                &mut self.send_buffer,
                &render_commands.view_matrix,
            );
            WebSocketBrowserRenderSystem::write3x3(
                &mut self.send_buffer,
                &render_commands.normal_matrix,
            );

            WebSocketBrowserRenderSystem::send_2d_partial_circle_commands(
                &mut self.send_buffer,
                &render_commands.partial_circle_2d_commands,
            );

            WebSocketBrowserRenderSystem::send_2d_texture_commands(
                &mut self.send_buffer,
                &render_commands.texture_2d_commands,
            );

            WebSocketBrowserRenderSystem::send_2d_rectangle_commands(
                &mut self.send_buffer,
                &render_commands.rectangle_2d_commands,
            );

            WebSocketBrowserRenderSystem::send_3d_rectangle_commands(
                &mut self.send_buffer,
                &render_commands.rectangle_3d_commands,
            );

            WebSocketBrowserRenderSystem::send_3d_circle_commands(
                &mut self.send_buffer,
                &render_commands.circle_3d_commands,
            );

            WebSocketBrowserRenderSystem::send_3d_sprite_commands(
                &mut self.send_buffer,
                &render_commands.sprite_3d_commands,
            );

            WebSocketBrowserRenderSystem::send_3d_number_commands(
                &mut self.send_buffer,
                &render_commands.number_3d_commands,
            );

            WebSocketBrowserRenderSystem::send_3d_effect_commands(
                &mut self.send_buffer,
                &render_commands.effect_commands2,
            );

            WebSocketBrowserRenderSystem::send_3d_model_commands(
                &mut self.send_buffer,
                &render_commands.model_commands,
            );

            WebSocketBrowserRenderSystem::send_horizontal_texture_3d_commands(
                &mut self.send_buffer,
                &render_commands.horizontal_texture_3d_commands,
            );

            WebSocketBrowserRenderSystem::send_trimesh_3d_commands(
                &mut self.send_buffer,
                &render_commands.trimesh_3d_commands,
            );

            browser.send_message(&self.send_buffer);
            browser.update_sending_fps(system_vars.time);
        }
    }
}

impl WebSocketBrowserRenderSystem {
    fn send_2d_text_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &Vec<Text2dRenderCommand>,
    ) {

    }

    fn send_2d_rectangle_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &VecDeque<Rectangle2dRenderCommand>,
    ) {
        send_buffer
            .write_u32::<LittleEndian>(render_commands.len() as u32)
            .unwrap();
        for command in render_commands {
            for v in &command.color {
                send_buffer.write_u8(*v).unwrap();
            }
            send_buffer
                .write_f32::<LittleEndian>(command.rotation_rad)
                .unwrap();
            send_buffer
                .write_i16::<LittleEndian>(command.screen_pos[0])
                .unwrap();
            send_buffer
                .write_i16::<LittleEndian>(command.screen_pos[1])
                .unwrap();
            let packed_int: u32 = ((command.layer as u32) << 24)
                | ((command.width as u32) << 12)
                | ((command.height as u32) & 0b1111_11111111);
            send_buffer.write_u32::<LittleEndian>(packed_int).unwrap();
        }
    }

    fn write3x3(send_buffer: &mut Vec<u8>, mat: &Matrix3<f32>) {
        for v in mat.as_slice() {
            send_buffer.write_f32::<LittleEndian>(*v).unwrap();
        }
    }

    fn write4x4(send_buffer: &mut Vec<u8>, mat: &Matrix4<f32>) {
        for v in mat.as_slice() {
            send_buffer.write_f32::<LittleEndian>(*v).unwrap();
        }
    }

    fn send_2d_partial_circle_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &Vec<PartialCircle2dRenderCommand>,
    ) {
        send_buffer
            .write_u32::<LittleEndian>(render_commands.len() as u32)
            .unwrap();
        for command in render_commands {
            for v in &command.color {
                send_buffer.write_u8(*v).unwrap();
            }
            send_buffer
                .write_i16::<LittleEndian>(command.screen_pos[0])
                .unwrap();
            send_buffer
                .write_i16::<LittleEndian>(command.screen_pos[1])
                .unwrap();
            send_buffer
                .write_u16::<LittleEndian>(command.layer as u16)
                .unwrap();
            send_buffer
                .write_u16::<LittleEndian>(command.circumference_index as u16)
                .unwrap();
        }
    }

    fn send_3d_effect_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &Vec<(StrEffectId, i32, Vector2<f32>)>,
    ) {
        send_buffer
            .write_u32::<LittleEndian>(render_commands.len() as u32)
            .unwrap();
        for command in render_commands {
            let effect_id = (command.0).0 as u16;
            let key_index = command.1 as u16;
            let pos = command.2;
            send_buffer.write_u16::<LittleEndian>(effect_id).unwrap();
            send_buffer.write_u16::<LittleEndian>(key_index).unwrap();
            send_buffer.write_f32::<LittleEndian>(pos.x).unwrap();
            send_buffer.write_f32::<LittleEndian>(pos.y).unwrap();
        }
    }

    fn send_3d_model_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &Vec<ModelRenderCommand>,
    ) {
        send_buffer
            .write_u32::<LittleEndian>(render_commands.len() as u32)
            .unwrap();
        for command in render_commands {
            let packed_int: u32 =
                ((command.is_transparent as u32) << 31) | command.model_instance_index as u32;
            send_buffer.write_u32::<LittleEndian>(packed_int).unwrap();
        }
    }

    fn send_horizontal_texture_3d_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &Vec<HorizontalTexture3dRenderCommand>,
    ) {
        send_buffer
            .write_u32::<LittleEndian>(render_commands.len() as u32)
            .unwrap();
        for command in render_commands {
            for v in &command.color {
                send_buffer.write_u8(*v).unwrap();
            }
            send_buffer
                .write_f32::<LittleEndian>(command.pos.x)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.pos.y)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.rotation_rad)
                .unwrap();
            send_buffer
                .write_u32::<LittleEndian>(command.texture.0 as u32)
                .unwrap();
            send_buffer.write_f32::<LittleEndian>(command.w).unwrap();
            send_buffer.write_f32::<LittleEndian>(command.h).unwrap();
        }
    }

    fn send_trimesh_3d_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &[Vec<Trimesh3dRenderCommand>; 2],
    ) {
        let sizes = (render_commands[1].len() << 16) | render_commands[0].len();
        send_buffer.write_u32::<LittleEndian>(sizes as u32).unwrap();
        for command in &render_commands[0] {
            send_buffer
                .write_f32::<LittleEndian>(command.pos.x)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.pos.y)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.pos.z)
                .unwrap();
        }
    }

    fn send_3d_number_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &Vec<Number3dRenderCommand>,
    ) {
        send_buffer
            .write_u32::<LittleEndian>(render_commands.len() as u32)
            .unwrap();
        for command in render_commands {
            send_buffer
                .write_f32::<LittleEndian>(command.scale)
                .unwrap();

            for v in &command.color {
                send_buffer.write_u8(*v).unwrap();
            }
            send_buffer
                .write_f32::<LittleEndian>(command.pos.x)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.pos.y)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.pos.z)
                .unwrap();

            send_buffer
                .write_u32::<LittleEndian>(command.value)
                .unwrap();
        }
    }

    fn send_3d_sprite_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &Vec<Sprite3dRenderCommand>,
    ) {
        send_buffer
            .write_u32::<LittleEndian>(render_commands.len() as u32)
            .unwrap();
        for command in render_commands {
            for v in &command.color {
                send_buffer.write_u8(*v).unwrap();
            }
            for v in &command.offset {
                send_buffer.write_i16::<LittleEndian>(*v).unwrap();
            }
            send_buffer
                .write_f32::<LittleEndian>(command.pos.x)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.pos.y)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.pos.z)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.scale)
                .unwrap();
            let packed_int: u32 =
                ((command.is_vertically_flipped as u32) << 31) | command.texture.0 as u32;

            send_buffer.write_u32::<LittleEndian>(packed_int).unwrap();
        }
    }

    fn send_3d_circle_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &Vec<Circle3dRenderCommand>,
    ) {
        send_buffer
            .write_u32::<LittleEndian>(render_commands.len() as u32)
            .unwrap();
        for command in render_commands {
            for v in &command.color {
                send_buffer.write_u8(*v).unwrap();
            }
            send_buffer
                .write_f32::<LittleEndian>(command.pos.x)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.pos.y)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.pos.z)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.radius)
                .unwrap();
        }
    }

    fn send_3d_rectangle_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &Vec<Rectangle3dRenderCommand>,
    ) {
        send_buffer
            .write_u32::<LittleEndian>(render_commands.len() as u32)
            .unwrap();
        for command in render_commands {
            for v in &command.color {
                send_buffer.write_u8(*v).unwrap();
            }
            send_buffer
                .write_f32::<LittleEndian>(command.pos.x)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.pos.y)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.pos.z)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.rotation_rad)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.width)
                .unwrap();
            send_buffer
                .write_f32::<LittleEndian>(command.height)
                .unwrap();
        }
    }

    fn send_2d_texture_commands(
        send_buffer: &mut Vec<u8>,
        render_commands: &Vec<Texture2dRenderCommand>,
    ) {
        send_buffer
            .write_u32::<LittleEndian>(render_commands.len() as u32)
            .unwrap();
        for command in render_commands {
            for v in &command.color {
                send_buffer.write_u8(*v).unwrap();
            }
            send_buffer
                .write_i16::<LittleEndian>(command.offset[0])
                .unwrap();
            send_buffer
                .write_i16::<LittleEndian>(command.offset[1])
                .unwrap();

            send_buffer
                .write_f32::<LittleEndian>(command.rotation_rad)
                .unwrap();
            send_buffer
                .write_i16::<LittleEndian>(command.screen_pos[0])
                .unwrap();
            send_buffer
                .write_i16::<LittleEndian>(command.screen_pos[1])
                .unwrap();
            let packed_int: u32 = ((command.layer as u32) << 24) | command.texture.0 as u32;
            send_buffer.write_u32::<LittleEndian>(packed_int).unwrap();

            send_buffer
                .write_f32::<LittleEndian>(command.scale)
                .unwrap();
        }
    }
}
