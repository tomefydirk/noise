pub struct  Matrixcube{
    def:Vec<Vec<f32>>,
    taille:usize
}
impl Matrixcube  {
    fn init(taille:usize) ->Self{
        let mut def=Vec::with_capacity(taille);
        for i in 0..taille{
            def.push(Vec::<f32>::with_capacity(taille));
        }
        Self { def, taille }
    }
    fn get_position(&self,x:usize,y:usize)->Option<f32>{
        self.def.get(x).and_then(|ligne|{
            ligne.get(y).copied()
        })
    }
    
}
fn main() {
    println!("Hello, world!");
}
