// Nearly every Amethyst system triggers this warning, better ignore it:
#![allow(clippy::type_complexity)]
extern crate amethyst;

#[macro_use]
extern crate log;
extern crate specs_derive;

use amethyst::{
    animation::AnimationBundle,
    assets::{PrefabLoaderSystemDesc, Processor},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::SpriteRender,
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Application, GameDataBuilder,
};

mod components;
mod entities;
mod resources;
mod states;
mod systems;

use components::{AnimationId, AnimationPrefabData};
use resources::Map;
use systems::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?;
    let display_config_path = root.join("resources/display_config.ron");

    let assets_path = root.join("assets");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(root.join("resources/bindings_config.ron"))?;

    let prefab_loader_system_desc = PrefabLoaderSystemDesc::<AnimationPrefabData>::default();

    let game_data = GameDataBuilder::default()
        .with_system_desc(prefab_loader_system_desc, "scene_loader", &[])
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(input_bundle)?
        .with(Processor::<Map>::new(), "map_processor", &[])
        .with(PlayerInputSystem, "player_input_system", &[])
        .with(
            PlayerKinematicsSystem,
            "player_kinematics_system",
            &["player_input_system"],
        )
        .with(
            KinematicsSystem,
            "kinematics_system",
            &["player_kinematics_system"],
        )
        .with(CollisionSystem, "collision_system", &[])
        .with(TransformationSystem, "transformation_system", &[])
        .with(ParallaxSystem, "parallax_system", &[])
        .with(
            PlayerAnimationSystem,
            "player_animation_system",
            &["transformation_system"],
        )
        .with(
            AnimationControlSystem,
            "animation_control_system",
            &["player_animation_system"],
        )
        .with(
            DirectionSystem,
            "direction_system",
            &["transformation_system"],
        )
        .with(
            PlayerDeathSystem,
            "player_death_system",
            &["transformation_system"],
        )
        .with(
            CameraTransformationSystem,
            "camera_transformation_system",
            &["transformation_system"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.008, 0.043, 0.067, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game =
        Application::build(assets_path, states::LoadState::default())?.build(game_data)?;

    game.run();

    Ok(())
}
