use cm_lbm::lbm_2d::*;


#[tokio::main]
async fn main() {
    println!("Instantiating LBM 2D");
    let driver  = setup_wgpu().await;
    let _lbm_2d = LBM2D::new(&driver, 100, 100);
}
