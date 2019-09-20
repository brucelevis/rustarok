use nalgebra::{Isometry2, Vector2, Vector3};
use specs::{Entity, LazyUpdate};

use crate::common::{v2_to_v3, v3_to_v2, ElapsedTime};
use crate::components::char::{
    ActionPlayMode, CharActionIndex, CharacterStateComponent, SpriteRenderDescriptorComponent,
};
use crate::components::controller::{CharEntityId, WorldCoords};
use crate::components::skills::skill::{
    SkillDef, SkillManifestation, SkillManifestationComponent, SkillTargetType, WorldCollisions,
};
use crate::components::status::status::{
    ApplyStatusComponentPayload, ApplyStatusInAreaComponent, StatusNature,
};
use crate::components::status::stun::StunStatus;
use crate::components::{AreaAttackComponent, AttackType, DamageDisplayType, StrEffectComponent};
use crate::configs::{DevConfig, GazXplodiumChargeSkillConfigInner};
use crate::effect::StrEffectType;
use crate::runtime_assets::map::PhysicEngine;
use crate::systems::next_action_applier_sys::NextActionApplierSystem;
use crate::systems::render::render_command::RenderCommandCollector;
use crate::systems::render_sys::render_single_layer_action;
use crate::systems::sound_sys::AudioCommandCollectorComponent;
use crate::systems::{AssetResources, SystemVariables};

pub struct GazXplodiumChargeSkill;

pub const GAZ_XPLODIUM_CHARGE_SKILL: &'static GazXplodiumChargeSkill = &GazXplodiumChargeSkill;

impl SkillDef for GazXplodiumChargeSkill {
    fn get_icon_path(&self) -> &'static str {
        "data\\texture\\À¯ÀúÀÎÅÍÆäÀÌ½º\\item\\ra_detonator.bmp"
    }

    fn finish_cast(
        &self,
        caster_entity_id: CharEntityId,
        caster_pos: WorldCoords,
        skill_pos: Option<Vector2<f32>>,
        char_to_skill_dir: &Vector2<f32>,
        target_entity: Option<CharEntityId>,
        ecs_world: &mut specs::world::World,
    ) -> Option<Box<dyn SkillManifestation>> {
        Some(Box::new(GazXplodiumChargeSkillManifestation::new(
            caster_entity_id,
            caster_pos,
            skill_pos.unwrap(),
            &mut ecs_world.write_resource::<PhysicEngine>(),
            ecs_world.read_resource::<SystemVariables>().time,
            ecs_world
                .read_resource::<DevConfig>()
                .skills
                .gaz_xplodium_charge
                .inner
                .clone(),
        )))
    }

    fn get_skill_target_type(&self) -> SkillTargetType {
        SkillTargetType::Area
    }
}

struct GazXplodiumChargeSkillManifestation {
    start_pos: WorldCoords,
    end_pos: WorldCoords,
    current_pos: Vector3<f32>,
    current_target_pos: Vector3<f32>,
    caster_id: CharEntityId,
    started_at: ElapsedTime,
    configs: GazXplodiumChargeSkillConfigInner,
}

impl GazXplodiumChargeSkillManifestation {
    fn new(
        caster_id: CharEntityId,
        start_pos: WorldCoords,
        end_pos: WorldCoords,
        physics_world: &mut PhysicEngine,
        now: ElapsedTime,
        configs: GazXplodiumChargeSkillConfigInner,
    ) -> GazXplodiumChargeSkillManifestation {
        GazXplodiumChargeSkillManifestation {
            start_pos,
            end_pos,
            current_pos: Vector3::new(start_pos.x, 1.0, start_pos.y),
            started_at: now,
            caster_id,
            current_target_pos: v2_to_v3(&end_pos),
            configs,
        }
    }
}

impl SkillManifestation for GazXplodiumChargeSkillManifestation {
    fn update(
        &mut self,
        self_entity_id: Entity,
        all_collisions_in_world: &WorldCollisions,
        system_vars: &mut SystemVariables,
        entities: &specs::Entities,
        char_storage: &mut specs::WriteStorage<CharacterStateComponent>,
        physics_world: &mut PhysicEngine,
        updater: &mut specs::Write<LazyUpdate>,
    ) {
        let now = system_vars.time;

        let travel_duration_percentage = system_vars.time.percentage_between(
            self.started_at,
            self.started_at
                .add_seconds(self.configs.missile_travel_duration_seconds),
        );
        if travel_duration_percentage < 1.0 {
            let dir = v3_to_v2(&self.current_target_pos) - self.start_pos;
            self.current_pos = v2_to_v3(&(self.start_pos + dir * travel_duration_percentage));
            let increase_height = (travel_duration_percentage + 0.5).min(1.0) * 10.0;
            let decrease_height = (travel_duration_percentage - 0.5).max(0.0) * 20.0;
            self.current_pos.y = increase_height - decrease_height;
        } else {
            let end_time = self
                .started_at
                .add_seconds(self.configs.missile_travel_duration_seconds)
                .add_seconds(self.configs.detonation_duration);
            if end_time.has_already_passed(system_vars.time) {
                if let Some(caster) = char_storage.get(self.caster_id.0) {
                    let area_shape =
                        Box::new(ncollide2d::shape::Ball::new(self.configs.explosion_area));
                    let area_isom = Isometry2::new(self.end_pos, 0.0);
                    system_vars.area_attacks.push(AreaAttackComponent {
                        area_shape: area_shape.clone(),
                        area_isom: area_isom.clone(),
                        source_entity_id: self.caster_id,
                        typ: AttackType::SpellDamage(
                            self.configs.damage,
                            DamageDisplayType::SingleNumber,
                        ),
                        except: None,
                    });
                    system_vars
                        .apply_area_statuses
                        .push(ApplyStatusInAreaComponent {
                            source_entity_id: self.caster_id,
                            status: ApplyStatusComponentPayload::from_secondary(Box::new(
                                StunStatus {
                                    caster_entity_id: self.caster_id,
                                    started: system_vars.time,
                                    until: system_vars
                                        .time
                                        .add_seconds(self.configs.stun_duration_seconds),
                                },
                            )),
                            area_shape,
                            area_isom,
                            except: None,
                            nature: StatusNature::Harmful,
                            caster_team: caster.team,
                        });
                    updater.insert(
                        entities.create(),
                        StrEffectComponent {
                            effect_id: StrEffectType::Explosion.into(),
                            pos: self.end_pos,
                            start_time: system_vars.time,
                            die_at: None,
                            play_mode: ActionPlayMode::Once,
                        },
                    );
                }
                updater.remove::<SkillManifestationComponent>(self_entity_id);
            }
        }
    }

    fn render(
        &self,
        now: ElapsedTime,
        tick: u64,
        assets: &AssetResources,
        render_commands: &mut RenderCommandCollector,
        audio_command_collector: &mut AudioCommandCollectorComponent,
    ) {
        let missile_landed = self
            .started_at
            .add_seconds(self.configs.missile_travel_duration_seconds)
            .has_already_passed(now);
        let dir = NextActionApplierSystem::determine_dir(
            &v3_to_v2(&self.current_target_pos),
            &v3_to_v2(&self.current_pos),
        );
        let anim = SpriteRenderDescriptorComponent {
            action_index: CharActionIndex::Idle as usize,
            animation_started: self
                .started_at
                .add_seconds(self.configs.missile_travel_duration_seconds),
            animation_ends_at: ElapsedTime(0.0),
            forced_duration: None,
            direction: dir,
            fps_multiplier: 1.0,
        };
        render_single_layer_action(
            now,
            &anim,
            &assets.sprites.ginseng_bullet,
            &self.current_pos,
            [0, 0],
            false,
            1.0,
            if missile_landed {
                ActionPlayMode::PlayThenHold
            } else {
                ActionPlayMode::FixFrame(0)
            },
            &[255, 255, 255, 255],
            render_commands,
        );
        if missile_landed {
            let detonation_duration_perc = now.percentage_between(
                self.started_at
                    .add_seconds(self.configs.missile_travel_duration_seconds),
                self.started_at
                    .add_seconds(self.configs.missile_travel_duration_seconds)
                    .add_seconds(self.configs.detonation_duration),
            );
            let number = 4 - (detonation_duration_perc / 0.25) as usize;
            // render countdown number
            let anim = SpriteRenderDescriptorComponent {
                action_index: CharActionIndex::Idle as usize,
                animation_started: ElapsedTime(0.0),
                animation_ends_at: ElapsedTime(0.0),
                forced_duration: None,
                direction: number,
                fps_multiplier: 1.0,
            };
            render_single_layer_action(
                now,
                &anim,
                &assets.sprites.timefont,
                &Vector3::new(self.end_pos.x, 2.0, self.end_pos.y),
                [0, 0],
                false,
                0.5,
                ActionPlayMode::FixFrame(0),
                &[255, 255, 255, 255],
                render_commands,
            );

            // render area
            render_commands
                .horizontal_texture_3d()
                .pos(&self.end_pos)
                .rotation_rad(now.0 % 6.28)
                .fix_size(self.configs.explosion_area * 2.0)
                .add(&assets.sprites.magic_target);
        }
    }
}
