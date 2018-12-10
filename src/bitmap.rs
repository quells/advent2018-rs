#[derive(Clone)]
pub struct Bitmap<A> {
    pub w: usize,
    pub h: usize,
    pub field: Vec<A>,
}

#[allow(dead_code)]
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

    pub fn rows(&self) -> Vec<Vec<A>> {
        let mut rows = Vec::new();
        for y in 0 .. self.h {
            let idx = y * self.w;
            let row = self.field[idx .. idx+self.w].to_vec();
            rows.push(row);
        }
        rows
    }
}