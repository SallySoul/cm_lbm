use crate::*;
use nalgebra::vector;
use vtkio::model::*;

/// I use unstructured grid becuase I couldn't be bothered
/// to double check VTK's indexing for structured grids.
/// This class takes grid dimensions and makes something ready to
/// write out as vtu file.
/// Other point attributes can be tacked on.
/// This is not cheap to construct, FYI.
pub struct VTKGrid {
    points: IOBuffer,
    cells: Cells,
    point_attributes: Vec<Attribute>,
}

impl VTKGrid {
    pub fn new(grid_dimensions: &AABB3) -> VTKGrid {
        // Collect the grid points as vertices in mesh
        let buffer_size = box_buffer_size(grid_dimensions);
        let mut points = Vec::with_capacity(3 * buffer_size);
        for coord in coord_iter(*grid_dimensions) {
            points.push(coord[0] as f32);
            points.push(coord[1] as f32);
            points.push(coord[2] as f32);
        }

        // Assemble Hexahedron elements from grid points
        let n_cells = cell_count(grid_dimensions);
        let mut connectivity = Vec::with_capacity(n_cells);
        let mut offsets = Vec::with_capacity(n_cells);
        let mut cell_types = Vec::with_capacity(n_cells);
        let mut offset = 8;
        for cell_coord in cell_coord_iter(*grid_dimensions) {
            let n_1 = cell_coord + vector![0, 0, 1];
            let n_2 = cell_coord + vector![0, 1, 0];
            let n_3 = cell_coord + vector![1, 0, 0];
            let n_4 = cell_coord + vector![0, 1, 1];
            let n_5 = cell_coord + vector![1, 1, 0];
            let n_6 = cell_coord + vector![1, 0, 1];
            let n_7 = cell_coord + vector![1, 1, 1];

            let vertices = [&cell_coord, &n_3, &n_6, &n_1, &n_2, &n_5, &n_7, &n_4];
            for v in vertices {
                let index = coord_to_linear_in_box(v, grid_dimensions) as u64;
                connectivity.push(index);
            }

            offsets.push(offset);
            cell_types.push(CellType::Hexahedron);
            offset += 8;
        }

        VTKGrid {
            points: IOBuffer::F32(points),
            cells: Cells {
                cell_verts: VertexNumbers::XML {
                    connectivity,
                    offsets,
                },
                types: cell_types,
            },
            point_attributes: Vec::new(),
        }
    }

    /// Adds a field to the grid points
    pub fn add_attribute(&mut self, name: String, num_comp: u32, values: Vec<f32>) {
        self.point_attributes
            .push(Attribute::DataArray(DataArrayBase {
                name,
                elem: ElementType::Scalars {
                    num_comp,
                    lookup_table: None,
                },
                data: IOBuffer::F32(values),
            }));
    }

    /// Consume and write to specified vtu file
    pub fn write(self, path: &str) {
        Vtk {
            version: Version { major: 1, minor: 0 },
            title: String::new(),
            byte_order: ByteOrder::LittleEndian,
            file_path: None,
            data: DataSet::inline(UnstructuredGridPiece {
                points: self.points,
                cells: self.cells,
                data: Attributes {
                    point: self.point_attributes,
                    cell: vec![],
                },
            }),
        }
        .export(path)
        .unwrap();
    }
}
