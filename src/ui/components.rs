use bevy::ecs::component::Component;

#[derive(Component)]
pub struct OnMainMenuScreen;
#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}
