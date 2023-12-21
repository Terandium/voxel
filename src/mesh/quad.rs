use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use super::{voxel::Color, Face};

/// A quad is a rectangle in the voxel grid.
#[derive(Copy, Clone, Debug)]
pub struct Quad {
    pub voxel: [usize; 3],
    pub width: u8,
    pub height: u8,
    pub color: Color,
}

/// A group of quads that can be rendered together.
#[derive(Default)]
pub struct QuadGroups {
    pub groups: [Vec<Quad>; 6],
}

impl QuadGroups {
    pub fn iter(&self) -> impl ParallelIterator<Item = Face> {
        self.groups
            .par_iter()
            .enumerate()
            .flat_map(|(index, quads)| quads.par_iter().map(move |quad| (index, quad)))
            .map(|(index, quad)| Face {
                side: index.into(),
                quad,
            })
    }
}
