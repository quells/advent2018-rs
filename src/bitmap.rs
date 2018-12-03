pub struct Bitmap<A> {
    pub w: usize,
    pub h: usize,
    pub field: Vec<A>,
}

impl <A> Bitmap<A>
where
    A: Clone,
{
    pub fn new(w: usize, h: usize, zero: A) -> Bitmap<A> {
        let field = vec![zero; w*h];
        Bitmap{w, h, field}
    }

    pub fn draw_rectangle<F>(&mut self, x: usize, y: usize, w: usize, h: usize, increment: F)
    where
        F: Fn(&A) -> A,
    {
        for y in y .. y+h {
            let row = y * self.w;
            for x in x .. x+w {
                let idx = row + x;
                let new_value = increment(&self.field[idx]);
                self.field[idx] = new_value;
            }
        }
    }
}