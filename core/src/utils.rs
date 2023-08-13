#[inline]
pub fn to_index(row: u32, column: u32, width: u32) -> usize {
    (row * width + column) as usize
}

#[inline]
pub fn to_position(index: usize, width: usize) -> (u32, u32) {
    let row = index / width;
    let column = index % width;
    (row as u32, column as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_index_to_position() {
        let width = 10;
        let height = 5;
        for row in 0..height {
            for column in 0..width {
                let index = to_index(row, column, width);
                let (_row, _column) = to_position(index, width as usize);
                assert_eq!(_column, column);
                assert_eq!(_row, row);
            }
        }
    }
}