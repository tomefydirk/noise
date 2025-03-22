use image::Rgba;
use image::ImageBuffer;
use rand::seq::SliceRandom; 
pub struct  MatrixcubeColor{
    def:Vec<Vec<f64>>,
    taille:usize
}
pub struct PermutationTable{
    def:Vec<u8>
}
impl PermutationTable{
    fn init()->Self{
        let mut rng=Vec::<u8>::new();
        for i in 0..256{
            rng[i]=i as u8;
        }
        let mut a = rand::rng();
        rng.shuffle(&mut a);
        rng.extend(rng.clone());
       
        Self { def: rng }
    }
}
impl MatrixcubeColor  {
    fn init(taille:usize) ->Self{
        let mut def=Vec::with_capacity(taille);
        for _i in 0..taille{
            def.push(Vec::<f64>::with_capacity(taille));
        }
        Self { def, taille }
    }
    fn get_position(&self,x:usize,y:usize)->Option<f64>{
        self.def.get(x).and_then(|ligne|{
            ligne.get(y).copied()
        })
    }
    
}

fn fade(t:f64)->f64{
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
fn normalize_perlin(value: f64) -> u8 {
    let normalized = ((value + 1.0) / 2.0 * 255.0).round();
    normalized as u8
}
fn perlin(x:f64,y:f64,perm:PermutationTable){
    let X=x.floor() as u32 & 255;
    let Y=Y.floor() as u32 & 255;
}
fn main() {
    let imgx=128;
    let imgy=128;
    //taille la map
    let scale=25.0;

    //niveau de détails
    let octave=2;

    //détails par octave
    let lacunarity=2.0;

    //contribution des octaves sur la forme
    let persistance=1.2;

    let mut imgbuf = ImageBuffer::<Rgba<u8>, _>::new(imgx, imgy);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r =normalize_perlin (0.07100564928000022);
        let g =normalize_perlin(0.07100564928000022);
        let b = normalize_perlin(0.07100564928000022);
        *pixel = Rgba([r, g, b, 255]); 
    }


    imgbuf.save("image.png").unwrap();
    println!("Hello, world!");
}
