mod bc_uniform;
mod bounce_back;
mod collision;
mod dimensions_uniform;
mod dirichlet;
mod distributions;
mod faces;
mod init_op;
mod moments;
mod shader_template;
mod slip_surfaces;
mod solver_state;
mod stream;

pub use bc_uniform::*;
pub use bounce_back::*;
pub use collision::*;
pub use dimensions_uniform::*;
pub use dirichlet::*;
pub use distributions::*;
pub use faces::*;
pub use init_op::*;
pub use moments::*;
pub use shader_template::*;
pub use slip_surfaces::*;
pub use solver_state::*;
pub use stream::*;

pub enum CollisionType {
    BGK(f32),
    MRT(f32),
    CMMRT(f32),
}
