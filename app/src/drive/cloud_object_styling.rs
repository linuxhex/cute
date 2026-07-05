use crate::appearance::Appearance;
use crate::drive::DriveObjectType;
use crate::themes::theme::Fill;

pub fn warp_drive_icon_color(appearance: &Appearance, _object_type: DriveObjectType) -> Fill {
    appearance
        .theme()
        .main_text_color(appearance.theme().surface_2())
}

pub fn cute_drive_icon_color(appearance: &Appearance, _object_type: DriveObjectType) -> Fill {
    appearance
        .theme()
        .main_text_color(appearance.theme().surface_2())
}
