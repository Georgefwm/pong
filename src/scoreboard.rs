use bevy::prelude::*;

#[derive(Event, Default)]
pub struct ScoreEvent;

#[derive(Resource)]
pub struct Scoreboard {
    pub human: usize,
    pub computer: usize,
}

pub fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text>,
    mut score_events: EventReader<ScoreEvent>,
) {
    if !score_events.is_empty() {
        let mut text = query.single_mut();

        for _event in score_events.read() {
            text.sections[0].value = scoreboard.human.to_string();
            text.sections[2].value = scoreboard.computer.to_string();
        }

        score_events.clear();
    }
}
