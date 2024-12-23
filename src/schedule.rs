//! System schedule definitions and plugin.
use bevy::app::{Last, MainScheduleOrder, Plugin, PreUpdate};
use bevy::ecs::schedule::{ExecutorKind, ScheduleLabel};
use bevy::prelude::Schedule;

/// Surface/buffer mutation schedule. Runs after `PreUpdate` on the main schedule.
#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct SurfaceSchedule;

/// Rendering schedule. Runs before `Last` on the main schedule.
#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct RenderSchedule;

/// `Plugin` for schedule configuration.
pub(super) struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut buffer_schedule = Schedule::new(SurfaceSchedule);
        buffer_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        app.add_schedule(buffer_schedule);

        let mut render_schedule = Schedule::new(RenderSchedule);
        render_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        app.add_schedule(render_schedule);

        let mut main_schedule = app.world_mut().resource_mut::<MainScheduleOrder>();
        main_schedule.insert_before(PreUpdate, SurfaceSchedule);
        main_schedule.insert_before(Last, RenderSchedule);
    }
}
