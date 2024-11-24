use crate::dimensions::*;
use crate::kernel;
use crate::wgpu_util;
use vtkio::model::*;

pub fn coord_to_linear(x: i32, y: i32, rows: i32) -> u64 {
    (x * rows + y) as u64
}

pub struct VTKBuilder2D {
    points: Vec<f32>,
    connectivity: Vec<u64>,
    offsets: Vec<u64>,
    cell_types: Vec<CellType>,
    point_attributes: Vec<Attribute>,
}

impl VTKBuilder2D {
    pub fn new(lattice_dimensions: &LatticeDimensions) -> Self {
        let rows = lattice_dimensions.rows;
        let n_points = lattice_dimensions.total as usize;
        let n_cells = ((lattice_dimensions.rows - 1) * (lattice_dimensions.cols - 1)) as usize;
        let mut points = Vec::with_capacity(n_points);

        for x in 0..lattice_dimensions.cols {
            for y in 0..rows {
                points.push(x as f32 * lattice_dimensions.size);
                points.push(y as f32 * lattice_dimensions.size);
                points.push(0.0);
            }
        }

        let mut connectivity = Vec::with_capacity(n_cells);
        let mut offsets = Vec::with_capacity(n_cells);
        let mut cell_types = Vec::with_capacity(n_cells);
        let mut offset = 4;
        for x in 0..lattice_dimensions.cols - 1 {
            for y in 0..rows - 1 {
                connectivity.push(coord_to_linear(x, y, rows));
                connectivity.push(coord_to_linear(x + 1, y, rows));
                connectivity.push(coord_to_linear(x + 1, y + 1, rows));
                connectivity.push(coord_to_linear(x, y + 1, rows));
                offsets.push(offset);
                cell_types.push(CellType::Quad);
                offset += 4;
            }
        }

        let point_attributes = Vec::new();

        VTKBuilder2D {
            points,
            connectivity,
            offsets,
            cell_types,
            point_attributes,
        }
    }

    pub fn add_densities(&mut self, driver: &wgpu_util::Driver, densities: &kernel::Densities) {
        let densities_data = densities.get_data(driver);
        for q in 0..densities_data.densities.len() {
            self.point_attributes
                .push(Attribute::DataArray(DataArrayBase {
                    name: format!("q{}", q),
                    elem: ElementType::Scalars {
                        num_comp: 1,
                        lookup_table: None,
                    },
                    data: IOBuffer::F32(densities_data.densities[q].clone()),
                }));
        }
    }

    pub fn export(self, path: &str) {
        Vtk {
            version: Version { major: 1, minor: 0 },
            title: String::new(),
            byte_order: ByteOrder::LittleEndian,
            file_path: None,
            data: DataSet::inline(UnstructuredGridPiece {
                points: IOBuffer::F32(self.points),
                cells: Cells {
                    cell_verts: VertexNumbers::XML {
                        connectivity: self.connectivity,
                        offsets: self.offsets,
                    },
                    types: self.cell_types,
                },
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
