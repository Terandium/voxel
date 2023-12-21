use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::util::Color;

use super::Face;

#[derive(Copy, Clone, Debug)]
pub struct Quad {
    // The `voxel` field represents the position of the quad in a 3D space.
    pub voxel: [usize; 3],
    // The `color` field represents the color of the quad.
    pub color: Color,
}

// Public struct `QuadGroups` with a `Default` trait.
#[derive(Default)]
pub struct QuadGroups {
    // The `groups` field is an array of vectors, each containing `Quad` structs.
    // There are 6 groups, one for each side of a voxel.
    pub groups: [Vec<Quad>; 6],
}

// Implement methods for `QuadGroups`.
impl QuadGroups {
    // Public method `iter` returns a parallel iterator over `Face` items.
    // Each `Face` item is created from a `Quad` and its corresponding side index.
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

    // Public method `is_empty` checks if all groups in `QuadGroups` are empty.
    // Returns `true` if all groups are empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.groups.par_iter().all(|group| group.is_empty())
    }
}
