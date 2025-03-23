use std::f32::consts::PI;
use image::Rgba;
use image::ImageBuffer;
use bevy_math::ops::{sin,cos};
#[derive(Debug)]
pub struct Vec2(f32,f32);

impl Vec2 {
    fn init_rand(ix:u32,iy:u32,seed:&u32)->Self{
        let u = *seed + iy ;
        let v = *seed + ix  ;
        let mut a:f32=256.0 + (u + v) as f32;
         a /= PI + *seed as f32 + v  as f32;
        Vec2(sin(a),cos(a))
    }
}

fn interpolate(a0:f32, a1:f32, w:f32)->f32 {
    let t = (1.0 - cos(w * PI)) * 0.5; // Interpolation cosinus
    a0 * (1.0 - t) + a1 * t
}

fn normalize_perlin(value: f32) -> u8 {
    let normalized = ((value + 1.0) / 2.0 * 255.0).round();
    normalized as u8
}  
fn dotGridgradient(ix:u32,iy:u32,x:f32,y:f32,seed:&u32)->f32{
    let gradient=Vec2::init_rand(ix, iy,seed);
    let dx=x- (ix as f32);
    let dy=y- (iy as f32); 

    dx*gradient.0+ dy *gradient.1
}
fn perlin(x:f32,y:f32,seed:&u32)->f32{
    
    let x0=x as u32;
    let y0=y as u32;
    let x1=x0 + 1;
    let y1=y0 + 1;

    let xf=x-x0 as f32;
    let yf= y- y0 as f32;
     
    let mut n0=dotGridgradient(x0, y0, x, y,seed);
    let mut n1=dotGridgradient(x1, y0, x, y,seed);
    let ix0 =interpolate(n0, n1, xf);

     n0=dotGridgradient(x0, y1, x, y,seed);
     n1=dotGridgradient(x1, y1, x, y,seed);

    let ix1=interpolate(n0, n1, xf);

    interpolate(ix0, ix1, yf)
}
fn pixel_constrat(x:u32,y:u32,size:&u32,seed:&u32)->f32{
    let mut val=0.0;
    let mut freq=1.0;
    let mut amp=1.0 ;
    for _i in 0..12{
        val += perlin((x as f32 * freq) / (*size as f32), (y as f32 * freq) / (*size as f32),seed) * amp;
        freq *=2.0;
        amp  /=2.0;
    }
    val*=1.2;
    val = val.clamp(-1.00, 1.00);
    println!("constrat done on {x},{y}");
    val

}
fn generte_img(imgx:u32,imgy:u32,seed:u32){
    let crid_size=50;
    let mut imgbuf = ImageBuffer::<Rgba<u8>, _>::new(imgx, imgy); 
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let resultat_perlin=pixel_constrat(x, y,&crid_size,&seed);
        let r =normalize_perlin (resultat_perlin);
        let g =normalize_perlin(resultat_perlin);
        let b = normalize_perlin(resultat_perlin);
        *pixel = Rgba([r, g, b, 255]); 
    }


    imgbuf.save("image.png").unwrap();
} 
fn main() {
    
    let imgx=100;
    let imgy=100;
     
    generte_img(imgx, imgy,330211);
 
 
    
    println!("Hello, world!");
}
