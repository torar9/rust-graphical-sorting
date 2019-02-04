use object::*;

pub struct BubbleSort
{
    i: usize,
    j: usize,
}

impl BubbleSort
{
    pub fn new() -> BubbleSort
    {
        BubbleSort{i: 0, j: 0}
    }

    pub fn do_cycle(&mut self, list: &mut Vec<Object<u32>>) -> Option<()>
    {
        for i in self.i..list.len() -1
        {
            for j in self.j..list.len() - i - 1
            {
                if list[self.j + 1].value < list[j].value
                {
                    list.swap(self.j + 1, self.j);
                }
                list[self.j + 1].is_active = true;

                self.j += 1;
                if self.j >= list.len() - i - 1
                {
                    self.j = 0;
                    self.i += 1;
                }
                return None;
            }
            self.i += 1;
        }
        return Some(());
    }
}
