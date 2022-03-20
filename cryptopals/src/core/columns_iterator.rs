pub struct Columns<T> {
    chunks: Vec<T>,
}

impl<T> Iterator for Columns<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Self::Item = self
            .chunks
            .iter_mut()
            .filter_map(|iter| iter.next())
            .collect();

        if result.len() == 0 {
            None
        } else {
            Some(result)
        }
    }
}

impl<T: Iterator> Columns<T> {
    pub fn from(chunks: Vec<T>) -> Self {
        Self { chunks }
    }
}
