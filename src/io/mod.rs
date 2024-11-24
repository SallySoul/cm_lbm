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

/*
        let mut points = Vec::with_capacity(self.nodes.len() * 3);
        let mut edge_list_lengths = Vec::with_capacity(self.nodes.len());
        let mut displacement = Vec::with_capacity(self.nodes.len() * 3);
        let mut rhs = Vec::with_capacity(self.nodes.len() * 3);
        let mut node_id = Vec::with_capacity(self.nodes.len());
        let mut node_dirty = Vec::with_capacity(self.nodes.len());

        for node_index in self.node_range() {
            let node = self.node(node_index);

            points.push(node.pos.x);
            points.push(node.pos.y);
            points.push(node.pos.z);

            edge_list_lengths.push(self.edge_list_len(node_index) as u32);

            displacement.push(node.sol.x);
            displacement.push(node.sol.y);
            displacement.push(node.sol.z);

            rhs.push(node.rhs.x);
            rhs.push(node.rhs.y);
            rhs.push(node.rhs.z);

            node_id.push(node_index as i32);
            node_dirty.push(node.dirty() as i32);
        }

        let mut connectivity = Vec::with_capacity(self.element_count() * 8);
        let mut cell_types = Vec::with_capacity(self.element_count());
        let mut offsets = Vec::with_capacity(self.element_count());
        let mut element_id = Vec::with_capacity(self.element_count());
        let mut element_dirty = Vec::with_capacity(self.element_count());
        let mut offset = 8;
        for element_index in self.element_range() {
            let element = self.element(element_index);
            for c in element.nodes.iter() {
                connectivity.push((c.clone() - NODE_PREFIX) as u64);
            }

            cell_types.push(CellType::LagrangeHexahedron);
            offsets.push(offset);
            element_id.push(element_index as i32);
            element_dirty.push(element.dirty() as i32);

            offset += 8;
        }

        Vtk {
            version: Version { major: 1, minor: 0 },
            title: String::new(),
            byte_order: ByteOrder::LittleEndian,
            file_path: None,
            data: DataSet::inline(UnstructuredGridPiece {
                points: IOBuffer::F64(points),
                cells: Cells {
                    cell_verts: VertexNumbers::XML {
                        connectivity: connectivity,
                        offsets: offsets,
                    },
                    types: cell_types,
                },
                data: Attributes {
                    point: vec![
                        Attribute::DataArray(DataArrayBase {
                            name: String::from("edge_list_len"),
                            elem: ElementType::Scalars {
                                num_comp: 1,
                                lookup_table: None,
                            },
                            data: IOBuffer::U32(edge_list_lengths),
                        }),
                        Attribute::DataArray(DataArrayBase {
                            name: String::from("rhs"),
                            elem: ElementType::Scalars {
                                num_comp: 3,
                                lookup_table: None,
                            },
                            data: IOBuffer::F64(rhs),
                        }),
                        Attribute::DataArray(DataArrayBase {
                            name: String::from("displacement"),
                            elem: ElementType::Scalars {
                                num_comp: 3,
                                lookup_table: None,
                            },
                            data: IOBuffer::F64(displacement),
                        }),
                        Attribute::DataArray(DataArrayBase {
                            name: String::from("node_id"),
                            elem: ElementType::Scalars {
                                num_comp: 1,
                                lookup_table: None,
                            },
                            data: IOBuffer::I32(node_id),
                        }),
                        Attribute::DataArray(DataArrayBase {
                            name: String::from("node_dirty"),
                            elem: ElementType::Scalars {
                                num_comp: 1,
                                lookup_table: None,
                            },
                            data: IOBuffer::I32(node_dirty),
                        }),
                    ],
                    cell: vec![
                        Attribute::DataArray(DataArrayBase {
                            name: String::from("element_id"),
                            elem: ElementType::Scalars {
                                num_comp: 1,
                                lookup_table: None,
                            },
                            data: IOBuffer::I32(element_id),
                        }),
                        Attribute::DataArray(DataArrayBase {
                            name: String::from("element_dirty"),
                            elem: ElementType::Scalars {
                                num_comp: 1,
                                lookup_table: None,
                            },
                            data: IOBuffer::I32(element_dirty),
                        }),
                    ],
                },
            }),
        }
    }
*/
