use bevy::prelude::{Deref, DerefMut, Transform};
use dolly::{driver::RigDriver, prelude::*};

use crate::prelude::Transform2Dolly;

impl Fpv {
    pub fn from_position_target(target_transform: Transform) -> Self {
        let mut yp = YawPitch::new();
        yp.set_rotation_quat(target_transform.transform_2_dolly().rotation);
        Self(
            CameraRig::builder()
                .with(Position {
                    position: target_transform.transform_2_dolly().position,
                })
                .with(Rotation {
                    rotation: target_transform.transform_2_dolly().rotation,
                })
                .with(yp)
                .with(Smooth::new_position_rotation(1.0, 0.1))
                .build(),
        )
    }

    pub fn set_rotation(
        &mut self,
        delta_mouse: dolly::glam::Vec2,
        sensitivity: dolly::glam::Vec2,
        player_position: dolly::glam::Vec3,
        delta_time_sec: f32,
    ) {
        self.driver_mut::<YawPitch>().rotate_yaw_pitch(
            -0.1 * delta_mouse.x * sensitivity.x,
            -0.1 * delta_mouse.y * sensitivity.y,
        );
        self.driver_mut::<Position>()
            .translate(player_position * delta_time_sec * 10.0);
    }

    pub fn set_position(
        &mut self,
        player_position: bevy::math::Vec3,
        boost: f32,
        boost_mult: f32,
        lock_y: bool,
    ) -> dolly::glam::Vec3 {
        if lock_y {
            let (mut euler, a) = self.final_transform.rotation.to_axis_angle();
            euler.x = 0.;
            euler.z = 0.;
            self.final_transform.rotation = dolly::glam::Quat::from_axis_angle(euler, a);
        }
        self.final_transform.rotation
            * dolly::glam::Vec3::new(player_position.x, player_position.y, player_position.z)
                .clamp_length_max(1.0)
            * boost_mult.powf(boost)
    }
}

/// A custom camera rig which combines smoothed movement with a look-at driver.
#[derive(Debug, Deref, DerefMut)]
pub struct Fpv(CameraRig<RightHanded>);

// Turn the nested rig into a driver, so it can be used in another rig.
impl RigDriver<RightHanded> for Fpv {
    fn update(
        &mut self,
        params: dolly::rig::RigUpdateParams<RightHanded>,
    ) -> dolly::transform::Transform<RightHanded> {
        self.0.update(params.delta_time_seconds)
    }
}
