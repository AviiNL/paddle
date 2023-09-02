use bevy::prelude::*;
use bevy_hanabi::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_event::<SpawnParticle>()
            .add_systems(Startup, setup)
            .add_systems(Update, update);
    }
}

fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    // Create a color gradient for the particles
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::splat(1.0));
    color_gradient1.add_key(0.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
    color_gradient1.add_key(0.4, Vec4::new(1.0, 0.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::splat(0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(0.1));
    size_gradient1.add_key(0.5, Vec2::splat(0.5));
    size_gradient1.add_key(0.8, Vec2::splat(0.08));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let writer1 = ExprWriter::new();

    let accel1 = writer1.lit(Vec3::Y * -300.).expr();
    let update_accel1 = AccelModifier::new(accel1);

    let init_pos = SetPositionCone3dModifier {
        base_radius: writer1.lit(0.).expr(),
        top_radius: writer1.lit(2.).expr(),
        height: writer1.lit(5.).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer1.lit(Vec3::ZERO).expr(),
        speed: writer1.lit(100.).expr(),
    };

    let age = writer1.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer1.lit(0.5).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Create a new effect asset spawning 30 particles per second from a circle
    // and slowly fading from blue-ish to transparent over their lifetime.
    // By default the asset spawns the particles at Z=0.
    let spawner = Spawner::once(30.0.into(), false);
    let effect = effects.add(
        EffectAsset::new(32768, spawner, writer1.finish())
            .with_name("emit:rate")
            .with_property("my_accel", Vec3::new(0., -3., 0.).into())
            .init(init_pos)
            // Make spawned particles move away from the emitter origin
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(update_accel1)
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient1,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient1,
                screen_space_size: false,
            }),
    );

    // Spawn an instance of the particle effect, and override its Z layer to
    // be above the reference white square previously spawned.
    commands
        .spawn(ParticleEffectBundle {
            // Assign the Z layer so it appears in the egui inspector and can be modified at runtime
            effect: ParticleEffect::new(effect).with_z_layer_2d(Some(0.1)),
            ..default()
        })
        .insert(Name::new("effect:2d"));
}

fn update(
    mut q_effect: Query<(&mut EffectSpawner, &mut Transform)>,
    mut particle: EventReader<SpawnParticle>,
) {
    // Note: On first frame where the effect spawns, EffectSpawner is spawned during
    // CoreSet::PostUpdate, so will not be available yet. Ignore for a frame if
    // so.
    let Ok((mut spawner, mut effect_transform)) = q_effect.get_single_mut() else { return; };

    for event in particle.iter() {
        effect_transform.translation = event.position;
        effect_transform.rotation = event.rotation;
        spawner.reset();
    }
}

#[derive(Event)]
pub struct SpawnParticle {
    pub position: Vec3,
    pub rotation: Quat,
    // pub velocity: Vec3,
}
