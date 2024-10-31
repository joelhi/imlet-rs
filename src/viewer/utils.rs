use bevy::render::{
    mesh::{self, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};
use bevy_egui::egui::{self, InnerResponse};

use crate::types::geometry::Vec3;

use super::raw_mesh_data::RawMeshData;

pub fn convert_vec3_to_raw<T: Copy>(vec: &[Vec3<T>]) -> Vec<[T; 3]> {
    let len = vec.len();
    let mut new_vec = Vec::<[T; 3]>::with_capacity(len);

    unsafe {
        std::ptr::copy_nonoverlapping(vec.as_ptr(), new_vec.as_mut_ptr() as *mut Vec3<T>, len);
        new_vec.set_len(len);
    }

    new_vec
}

pub fn build_mesh_from_data(mesh_data: RawMeshData) -> bevy::prelude::Mesh {
    bevy::prelude::Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        bevy::prelude::Mesh::ATTRIBUTE_POSITION,
        mesh_data.vertex_data.clone(),
    )
    .with_inserted_indices(mesh::Indices::U32(mesh_data.faces.clone()))
    .with_computed_smooth_normals()
}

pub fn faces_as_flat_u32(vec: &Vec<[usize; 3]>) -> Vec<u32> {
    let mut flat_vec = Vec::with_capacity(3 * vec.len());

    for arr in vec {
        flat_vec.push(arr[0] as u32);
        flat_vec.push(arr[2] as u32);
        flat_vec.push(arr[1] as u32);
    }

    flat_vec
}

pub fn custom_dnd_drag_source<Payload, R>(
    ui: &mut egui::Ui,
    id: egui::Id,
    payload: Payload,
    add_contents: impl FnOnce(&mut egui::Ui) -> (R, Vec<egui::Response>),
) -> egui::InnerResponse<R>
where
    Payload: std::any::Any + Send + Sync,
{
    let is_being_dragged = ui.ctx().is_being_dragged(id);

    if is_being_dragged {
        egui::DragAndDrop::set_payload(ui.ctx(), payload);

        let layer_id = egui::LayerId::new(egui::Order::Tooltip, id);
        let InnerResponse { inner, response } = ui.with_layer_id(layer_id, |ui| add_contents(ui));

        if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
            let delta = pointer_pos - response.rect.center();
            ui.ctx().transform_layer_shapes(
                layer_id,
                egui::emath::TSTransform::from_translation(delta),
            );
        }

        InnerResponse::new(inner.0, response)
    } else {
        let InnerResponse {
            inner,
            mut response,
        } = ui.scope(|ui| add_contents(ui));

        let mut hovering_anything = false;
        for res in inner.1.iter() {
            if res.hovered() {
                hovering_anything = true;
            }
        }

        if !hovering_anything {
            let drag_response = ui
                .interact(response.rect, id, egui::Sense::drag())
                .on_hover_cursor(egui::CursorIcon::Grab);
            response = response.union(drag_response);
        }

        InnerResponse::new(inner.0, response)
    }
}
