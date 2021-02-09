// The font data here was modified from
// http://paulbourke.net/dataformats/hershey/
// Compiled by Paul Bourke in 1997.


#[allow(unused)]
pub(crate) struct Key {
    name: char,
    ascii: u8,
    width: u8,
    stroke_data: Box<[i8]>,
}

lazy_static! {
pub(crate) static ref HERSHEY_FONT_SIMPLEX_COMPACT: [Key; 95] = [
    Key { name: ' ', ascii: 32, width: 16, stroke_data: Box::new([]) },
    Key { name: '!', ascii: 33, width: 10, stroke_data: Box::new([5, 21, 5, 7, -1, -1, 5, 2, 4, 1, 5, 0, 6, 1, 5, 2, ]) },
    Key { name: '"', ascii: 34, width: 16, stroke_data: Box::new([4, 21, 4, 14, -1, -1, 12, 21, 12, 14, ]) },
    Key { name: '#', ascii: 35, width: 21, stroke_data: Box::new([11, 25, 4, -7, -1, -1, 17, 25, 10, -7, -1, -1, 4, 12, 18, 12, -1, -1, 3, 6, 17, 6, ]) },
    Key { name: '$', ascii: 36, width: 20, stroke_data: Box::new([8, 25, 8, -4, -1, -1, 12, 25, 12, -4, -1, -1, 17, 18, 15, 20, 12, 21, 8, 21, 5, 20, 3, 18, 3, 16, 4, 14, 5, 13, 7, 12, 13, 10, 15, 9, 16, 8, 17, 6, 17, 3, 15, 1, 12, 0, 8, 0, 5, 1, 3, 3, ]) },
    Key { name: '%', ascii: 37, width: 24, stroke_data: Box::new([21, 21, 3, 0, -1, -1, 8, 21, 10, 19, 10, 17, 9, 15, 7, 14, 5, 14, 3, 16, 3, 18, 4, 20, 6, 21, 8, 21, 10, 20, 13, 19, 16, 19, 19, 20, 21, 21, -1, -1, 17, 7, 15, 6, 14, 4, 14, 2, 16, 0, 18, 0, 20, 1, 21, 3, 21, 5, 19, 7, 17, 7, ]) },
    Key { name: '&', ascii: 38, width: 26, stroke_data: Box::new([23, 12, 23, 13, 22, 14, 21, 14, 20, 13, 19, 11, 17, 6, 15, 3, 13, 1, 11, 0, 7, 0, 5, 1, 4, 2, 3, 4, 3, 6, 4, 8, 5, 9, 12, 13, 13, 14, 14, 16, 14, 18, 13, 20, 11, 21, 9, 20, 8, 18, 8, 16, 9, 13, 11, 10, 16, 3, 18, 1, 20, 0, 22, 0, 23, 1, 23, 2, ]) },
    Key { name: '\'', ascii: 39, width: 10, stroke_data: Box::new([5, 19, 4, 20, 5, 21, 6, 20, 6, 18, 5, 16, 4, 15, ]) },
    Key { name: '(', ascii: 40, width: 14, stroke_data: Box::new([11, 25, 9, 23, 7, 20, 5, 16, 4, 11, 4, 7, 5, 2, 7, -2, 9, -5, 11, -7, ]) },
    Key { name: ')', ascii: 41, width: 14, stroke_data: Box::new([3, 25, 5, 23, 7, 20, 9, 16, 10, 11, 10, 7, 9, 2, 7, -2, 5, -5, 3, -7, ]) },
    Key { name: '*', ascii: 42, width: 16, stroke_data: Box::new([8, 21, 8, 9, -1, -1, 3, 18, 13, 12, -1, -1, 13, 18, 3, 12, ]) },
    Key { name: '+', ascii: 43, width: 26, stroke_data: Box::new([13, 18, 13, 0, -1, -1, 4, 9, 22, 9, ]) },
    Key { name: ',', ascii: 44, width: 10, stroke_data: Box::new([6, 1, 5, 0, 4, 1, 5, 2, 6, 1, 6, -1, 5, -3, 4, -4, ]) },
    Key { name: '-', ascii: 45, width: 26, stroke_data: Box::new([4, 9, 22, 9, ]) },
    Key { name: '.', ascii: 46, width: 10, stroke_data: Box::new([5, 2, 4, 1, 5, 0, 6, 1, 5, 2, ]) },
    Key { name: '/', ascii: 47, width: 22, stroke_data: Box::new([20, 25, 2, -7, ]) },
    Key { name: '0', ascii: 48, width: 20, stroke_data: Box::new([9, 21, 6, 20, 4, 17, 3, 12, 3, 9, 4, 4, 6, 1, 9, 0, 11, 0, 14, 1, 16, 4, 17, 9, 17, 12, 16, 17, 14, 20, 11, 21, 9, 21, ]) },
    Key { name: '1', ascii: 49, width: 20, stroke_data: Box::new([8, 19, 10, 20, 11, 21, 11, 0, ]) },
    Key { name: '2', ascii: 50, width: 20, stroke_data: Box::new([4, 16, 4, 17, 5, 19, 6, 20, 8, 21, 12, 21, 14, 20, 15, 19, 16, 17, 16, 15, 15, 13, 13, 10, 3, 0, 17, 0, ]) },
    Key { name: '3', ascii: 51, width: 20, stroke_data: Box::new([5, 21, 16, 21, 10, 13, 13, 13, 15, 12, 16, 11, 17, 8, 17, 6, 16, 3, 14, 1, 11, 0, 8, 0, 5, 1, 4, 2, 3, 4, ]) },
    Key { name: '4', ascii: 52, width: 20, stroke_data: Box::new([13, 21, 3, 7, 18, 7, -1, -1, 13, 21, 13, 0, ]) },
    Key { name: '5', ascii: 53, width: 20, stroke_data: Box::new([15, 21, 5, 21, 4, 12, 5, 13, 8, 14, 11, 14, 14, 13, 16, 11, 17, 8, 17, 6, 16, 3, 14, 1, 11, 0, 8, 0, 5, 1, 4, 2, 3, 4, ]) },
    Key { name: '6', ascii: 54, width: 20, stroke_data: Box::new([16, 18, 15, 20, 12, 21, 10, 21, 7, 20, 5, 17, 4, 12, 4, 7, 5, 3, 7, 1, 10, 0, 11, 0, 14, 1, 16, 3, 17, 6, 17, 7, 16, 10, 14, 12, 11, 13, 10, 13, 7, 12, 5, 10, 4, 7, ]) },
    Key { name: '7', ascii: 55, width: 20, stroke_data: Box::new([17, 21, 7, 0, -1, -1, 3, 21, 17, 21, ]) },
    Key { name: '8', ascii: 56, width: 20, stroke_data: Box::new([8, 21, 5, 20, 4, 18, 4, 16, 5, 14, 7, 13, 11, 12, 14, 11, 16, 9, 17, 7, 17, 4, 16, 2, 15, 1, 12, 0, 8, 0, 5, 1, 4, 2, 3, 4, 3, 7, 4, 9, 6, 11, 9, 12, 13, 13, 15, 14, 16, 16, 16, 18, 15, 20, 12, 21, 8, 21, ]) },
    Key { name: '9', ascii: 57, width: 20, stroke_data: Box::new([16, 14, 15, 11, 13, 9, 10, 8, 9, 8, 6, 9, 4, 11, 3, 14, 3, 15, 4, 18, 6, 20, 9, 21, 10, 21, 13, 20, 15, 18, 16, 14, 16, 9, 15, 4, 13, 1, 10, 0, 8, 0, 5, 1, 4, 3, ]) },
    Key { name: ':', ascii: 58, width: 10, stroke_data: Box::new([5, 14, 4, 13, 5, 12, 6, 13, 5, 14, -1, -1, 5, 2, 4, 1, 5, 0, 6, 1, 5, 2, ]) },
    Key { name: ';', ascii: 59, width: 10, stroke_data: Box::new([5, 14, 4, 13, 5, 12, 6, 13, 5, 14, -1, -1, 6, 1, 5, 0, 4, 1, 5, 2, 6, 1, 6, -1, 5, -3, 4, -4, ]) },
    Key { name: '<', ascii: 60, width: 24, stroke_data: Box::new([20, 18, 4, 9, 20, 0, ]) },
    Key { name: '=', ascii: 61, width: 26, stroke_data: Box::new([4, 12, 22, 12, -1, -1, 4, 6, 22, 6, ]) },
    Key { name: '>', ascii: 62, width: 24, stroke_data: Box::new([4, 18, 20, 9, 4, 0, ]) },
    Key { name: '?', ascii: 63, width: 18, stroke_data: Box::new([3, 16, 3, 17, 4, 19, 5, 20, 7, 21, 11, 21, 13, 20, 14, 19, 15, 17, 15, 15, 14, 13, 13, 12, 9, 10, 9, 7, -1, -1, 9, 2, 8, 1, 9, 0, 10, 1, 9, 2, ]) },
    Key { name: '@', ascii: 64, width: 27, stroke_data: Box::new([18, 13, 17, 15, 15, 16, 12, 16, 10, 15, 9, 14, 8, 11, 8, 8, 9, 6, 11, 5, 14, 5, 16, 6, 17, 8, -1, -1, 12, 16, 10, 14, 9, 11, 9, 8, 10, 6, 11, 5, -1, -1, 18, 16, 17, 8, 17, 6, 19, 5, 21, 5, 23, 7, 24, 10, 24, 12, 23, 15, 22, 17, 20, 19, 18, 20, 15, 21, 12, 21, 9, 20, 7, 19, 5, 17, 4, 15, 3, 12, 3, 9, 4, 6, 5, 4, 7, 2, 9, 1, 12, 0, 15, 0, 18, 1, 20, 2, 21, 3, -1, -1, 19, 16, 18, 8, 18, 6, 19, 5, ]) },
    Key { name: 'A', ascii: 65, width: 18, stroke_data: Box::new([9, 21, 1, 0, -1, -1, 9, 21, 17, 0, -1, -1, 4, 7, 14, 7, ]) },
    Key { name: 'B', ascii: 66, width: 21, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 4, 21, 13, 21, 16, 20, 17, 19, 18, 17, 18, 15, 17, 13, 16, 12, 13, 11, -1, -1, 4, 11, 13, 11, 16, 10, 17, 9, 18, 7, 18, 4, 17, 2, 16, 1, 13, 0, 4, 0, ]) },
    Key { name: 'C', ascii: 67, width: 21, stroke_data: Box::new([18, 16, 17, 18, 15, 20, 13, 21, 9, 21, 7, 20, 5, 18, 4, 16, 3, 13, 3, 8, 4, 5, 5, 3, 7, 1, 9, 0, 13, 0, 15, 1, 17, 3, 18, 5, ]) },
    Key { name: 'D', ascii: 68, width: 21, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 4, 21, 11, 21, 14, 20, 16, 18, 17, 16, 18, 13, 18, 8, 17, 5, 16, 3, 14, 1, 11, 0, 4, 0, ]) },
    Key { name: 'E', ascii: 69, width: 19, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 4, 21, 17, 21, -1, -1, 4, 11, 12, 11, -1, -1, 4, 0, 17, 0, ]) },
    Key { name: 'F', ascii: 70, width: 18, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 4, 21, 17, 21, -1, -1, 4, 11, 12, 11, ]) },
    Key { name: 'G', ascii: 71, width: 21, stroke_data: Box::new([18, 16, 17, 18, 15, 20, 13, 21, 9, 21, 7, 20, 5, 18, 4, 16, 3, 13, 3, 8, 4, 5, 5, 3, 7, 1, 9, 0, 13, 0, 15, 1, 17, 3, 18, 5, 18, 8, -1, -1, 13, 8, 18, 8, ]) },
    Key { name: 'H', ascii: 72, width: 22, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 4, 11, 18, 11, -1, -1, 18, 21, 18, 0, ]) },
    Key { name: 'I', ascii: 73, width: 8, stroke_data: Box::new([4, 21, 4, 0, ]) },
    Key { name: 'J', ascii: 74, width: 16, stroke_data: Box::new([12, 21, 12, 5, 11, 2, 10, 1, 8, 0, 6, 0, 4, 1, 3, 2, 2, 5, 2, 7, ]) },
    Key { name: 'K', ascii: 75, width: 21, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 18, 21, 4, 7, -1, -1, 9, 12, 18, 0, ]) },
    Key { name: 'L', ascii: 76, width: 17, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 4, 0, 16, 0, ]) },
    Key { name: 'M', ascii: 77, width: 24, stroke_data: Box::new([4, 0, 4, 21, 4, 21, 12, 0, 12, 0, 20, 21, 20, 21, 20, 0, ]) },
    Key { name: 'N', ascii: 78, width: 22, stroke_data: Box::new([4, 0, 4, 21, 4, 21, 18, 0, 18, 0, 18, 21, ]) },
    Key { name: 'O', ascii: 79, width: 22, stroke_data: Box::new([9, 21, 7, 20, 5, 18, 4, 16, 3, 13, 3, 8, 4, 5, 5, 3, 7, 1, 9, 0, 13, 0, 15, 1, 17, 3, 18, 5, 19, 8, 19, 13, 18, 16, 17, 18, 15, 20, 13, 21, 9, 21, ]) },
    Key { name: 'P', ascii: 80, width: 21, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 4, 21, 13, 21, 16, 20, 17, 19, 18, 17, 18, 14, 17, 12, 16, 11, 13, 10, 4, 10, ]) },
    Key { name: 'Q', ascii: 81, width: 22, stroke_data: Box::new([9, 21, 7, 20, 5, 18, 4, 16, 3, 13, 3, 8, 4, 5, 5, 3, 7, 1, 9, 0, 13, 0, 15, 1, 17, 3, 18, 5, 19, 8, 19, 13, 18, 16, 17, 18, 15, 20, 13, 21, 9, 21, -1, -1, 12, 4, 18, -2, ]) },
    Key { name: 'R', ascii: 82, width: 21, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 4, 21, 13, 21, 16, 20, 17, 19, 18, 17, 18, 15, 17, 13, 16, 12, 13, 11, 4, 11, -1, -1, 11, 11, 18, 0, ]) },
    Key { name: 'S', ascii: 83, width: 20, stroke_data: Box::new([17, 18, 15, 20, 12, 21, 8, 21, 5, 20, 3, 18, 3, 16, 4, 14, 5, 13, 7, 12, 13, 10, 15, 9, 16, 8, 17, 6, 17, 3, 15, 1, 12, 0, 8, 0, 5, 1, 3, 3, ]) },
    Key { name: 'T', ascii: 84, width: 16, stroke_data: Box::new([8, 21, 8, 0, -1, -1, 1, 21, 15, 21, ]) },
    Key { name: 'U', ascii: 85, width: 22, stroke_data: Box::new([4, 21, 4, 6, 5, 3, 7, 1, 10, 0, 12, 0, 15, 1, 17, 3, 18, 6, 18, 21, ]) },
    Key { name: 'V', ascii: 86, width: 18, stroke_data: Box::new([1, 21, 9, 0, -1, -1, 17, 21, 9, 0, ]) },
    Key { name: 'W', ascii: 87, width: 24, stroke_data: Box::new([2, 21, 7, 0, -1, -1, 12, 21, 7, 0, -1, -1, 12, 21, 17, 0, -1, -1, 22, 21, 17, 0, ]) },
    Key { name: 'X', ascii: 88, width: 20, stroke_data: Box::new([3, 21, 17, 0, -1, -1, 17, 21, 3, 0, ]) },
    Key { name: 'Y', ascii: 89, width: 18, stroke_data: Box::new([1, 21, 9, 11, 17, 21, -1, -1, 9, 11, 9, 0]) },
    Key { name: 'Z', ascii: 90, width: 20, stroke_data: Box::new([3, 21, 17, 21, 17, 21, 3, 0, 3, 0, 17, 0, ]) },
    Key { name: '[', ascii: 91, width: 14, stroke_data: Box::new([11, 25, 4, 25, 4, -7, 11, -7, ]) },
    Key { name: '\\', ascii: 92, width: 14, stroke_data: Box::new([0, 21, 14, -3, ]) },
    Key { name: ']', ascii: 93, width: 14, stroke_data: Box::new([3, 25, 10, 25, 10, -7, 3, -7]) },
    Key { name: '^', ascii: 94, width: 16, stroke_data: Box::new([6, 15, 8, 18, 10, 15, -1, -1, 3, 12, 8, 17, 13, 12, -1, -1, 8, 17, 8, 0, ]) },
    Key { name: '_', ascii: 95, width: 16, stroke_data: Box::new([0, -2, 16, -2, ]) },
    Key { name: '`', ascii: 96, width: 10, stroke_data: Box::new([6, 21, 5, 20, 4, 18, 4, 16, 5, 15, 6, 16, 5, 17, ]) },
    Key { name: 'a', ascii: 97, width: 19, stroke_data: Box::new([15, 11, 13, 13, 11, 14, 8, 14, 6, 13, 4, 11, 3, 8, 3, 6, 4, 3, 6, 1, 8, 0, 11, 0, 13, 1, 15, 3, -1, -1, 15, 14, 15, 0, ]) },
    Key { name: 'b', ascii: 98, width: 19, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 4, 11, 6, 13, 8, 14, 11, 14, 13, 13, 15, 11, 16, 8, 16, 6, 15, 3, 13, 1, 11, 0, 8, 0, 6, 1, 4, 3, ]) },
    Key { name: 'c', ascii: 99, width: 18, stroke_data: Box::new([15, 11, 13, 13, 11, 14, 8, 14, 6, 13, 4, 11, 3, 8, 3, 6, 4, 3, 6, 1, 8, 0, 11, 0, 13, 1, 15, 3, ]) },
    Key { name: 'd', ascii: 100, width: 19, stroke_data: Box::new([15, 21, 15, 0, -1, -1, 15, 11, 13, 13, 11, 14, 8, 14, 6, 13, 4, 11, 3, 8, 3, 6, 4, 3, 6, 1, 8, 0, 11, 0, 13, 1, 15, 3, ]) },
    Key { name: 'e', ascii: 101, width: 18, stroke_data: Box::new([3, 8, 15, 8, 15, 10, 14, 12, 13, 13, 11, 14, 8, 14, 6, 13, 4, 11, 3, 8, 3, 6, 4, 3, 6, 1, 8, 0, 11, 0, 13, 1, 15, 3, ]) },
    Key { name: 'f', ascii: 102, width: 12, stroke_data: Box::new([10, 21, 8, 21, 6, 20, 5, 17, 5, 0, -1, -1, 2, 14, 9, 14, ]) },
    Key { name: 'g', ascii: 103, width: 19, stroke_data: Box::new([15, 11, 13, 13, 11, 14, 8, 14, 6, 13, 4, 11, 3, 8, 3, 6, 4, 3, 6, 1, 8, 0, 11, 0, 13, 1, 15, 3, -1, -1, 15, 14, 15, -2, 14, -5, 13, -6, 11, -7, 8, -7, 6, -6, ]) },
    Key { name: 'h', ascii: 104, width: 19, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 4, 10, 7, 13, 9, 14, 12, 14, 14, 13, 15, 10, 15, 0, ]) },
    Key { name: 'i', ascii: 105, width: 8, stroke_data: Box::new([3, 21, 4, 20, 5, 21, 4, 22, 3, 21, -1, -1, 4, 14, 4, 0, ]) },
    Key { name: 'j', ascii: 106, width: 10, stroke_data: Box::new([5, 21, 6, 20, 7, 21, 6, 22, 5, 21, -1, -1, 6, 14, 6, -3, 5, -6, 3, -7, 1, -7, ]) },
    Key { name: 'k', ascii: 107, width: 17, stroke_data: Box::new([4, 21, 4, 0, -1, -1, 14, 14, 4, 4, -1, -1, 8, 8, 15, 0, ]) },
    Key { name: 'l', ascii: 108, width: 8, stroke_data: Box::new([4, 21, 4, 0, ]) },
    Key { name: 'm', ascii: 109, width: 30, stroke_data: Box::new([4, 14, 4, 0, -1, -1, 4, 10, 7, 13, 9, 14, 12, 14, 14, 13, 15, 10, 15, 0, -1, -1, 15, 10, 18, 13, 20, 14, 23, 14, 25, 13, 26, 10, 26, 0, ]) },
    Key { name: 'n', ascii: 110, width: 19, stroke_data: Box::new([4, 14, 4, 0, -1, -1, 4, 10, 7, 13, 9, 14, 12, 14, 14, 13, 15, 10, 15, 0, ]) },
    Key { name: 'o', ascii: 111, width: 19, stroke_data: Box::new([8, 14, 6, 13, 4, 11, 3, 8, 3, 6, 4, 3, 6, 1, 8, 0, 11, 0, 13, 1, 15, 3, 16, 6, 16, 8, 15, 11, 13, 13, 11, 14, 8, 14, ]) },
    Key { name: 'p', ascii: 112, width: 19, stroke_data: Box::new([4, 14, 4, -7, -1, -1, 4, 11, 6, 13, 8, 14, 11, 14, 13, 13, 15, 11, 16, 8, 16, 6, 15, 3, 13, 1, 11, 0, 8, 0, 6, 1, 4, 3, ]) },
    Key { name: 'q', ascii: 113, width: 19, stroke_data: Box::new([15, 11, 13, 13, 11, 14, 8, 14, 6, 13, 4, 11, 3, 8, 3, 6, 4, 3, 6, 1, 8, 0, 11, 0, 13, 1, 15, 3, -1, -1, 15, 14, 15, -7, ]) },
    Key { name: 'r', ascii: 114, width: 13, stroke_data: Box::new([4, 14, 4, 0, -1, -1, 4, 8, 5, 11, 7, 13, 9, 14, 12, 14, ]) },
    Key { name: 's', ascii: 115, width: 17, stroke_data: Box::new([14, 11, 13, 13, 10, 14, 7, 14, 4, 13, 3, 11, 4, 9, 6, 8, 11, 7, 13, 6, 14, 4, 14, 3, 13, 1, 10, 0, 7, 0, 4, 1, 3, 3, ]) },
    Key { name: 't', ascii: 116, width: 12, stroke_data: Box::new([5, 21, 5, 4, 6, 1, 8, 0, 10, 0, -1, -1, 2, 14, 9, 14, ]) },
    Key { name: 'u', ascii: 117, width: 19, stroke_data: Box::new([4, 14, 4, 4, 5, 1, 7, 0, 10, 0, 12, 1, 15, 4, -1, -1, 15, 14, 15, 0, ]) },
    Key { name: 'v', ascii: 118, width: 16, stroke_data: Box::new([2, 14, 8, 0, -1, -1, 14, 14, 8, 0, ]) },
    Key { name: 'w', ascii: 119, width: 22, stroke_data: Box::new([3, 14, 7, 0, 11, 14, 15, 0, 19, 14, ]) },
    Key { name: 'x', ascii: 120, width: 17, stroke_data: Box::new([3, 14, 14, 0, -1, -1, 14, 14, 3, 0, ]) },
    Key { name: 'y', ascii: 121, width: 16, stroke_data: Box::new([2, 14, 8, 0, -1, -1, 14, 14, 8, 0, 6, -4, 4, -6, 2, -7, 1, -7, ]) },
    Key { name: 'z', ascii: 122, width: 17, stroke_data: Box::new([3, 14, 14, 14, 14, 14, 3, 0, 3, 0, 14, 0, ]) },
    Key { name: '{', ascii: 123, width: 14, stroke_data: Box::new([9, 25, 7, 24, 6, 23, 5, 21, 5, 19, 6, 17, 7, 16, 8, 14, 8, 12, 6, 10, -1, -1, 7, 24, 6, 22, 6, 20, 7, 18, 8, 17, 9, 15, 9, 13, 8, 11, 4, 9, 8, 7, 9, 5, 9, 3, 8, 1, 7, 0, 6, -2, 6, -4, 7, -6, -1, -1, 6, 8, 8, 6, 8, 4, 7, 2, 6, 1, 5, -1, 5, -3, 6, -5, 7, -6, 9, -7, ]) },
    Key { name: '|', ascii: 124, width: 8, stroke_data: Box::new([4, 25, 4, -7, ]) },
    Key { name: '}', ascii: 125, width: 14, stroke_data: Box::new([5, 25, 7, 24, 8, 23, 9, 21, 9, 19, 8, 17, 7, 16, 6, 14, 6, 12, 8, 10, -1, -1, 7, 24, 8, 22, 8, 20, 7, 18, 6, 17, 5, 15, 5, 13, 6, 11, 10, 9, 6, 7, 5, 5, 5, 3, 6, 1, 7, 0, 8, -2, 8, -4, 7, -6, -1, -1, 8, 8, 6, 6, 6, 4, 7, 2, 8, 1, 9, -1, 9, -3, 8, -5, 7, -6, 5, -7, ]) },
    Key { name: '~', ascii: 126, width: 24, stroke_data: Box::new([3, 6, 3, 8, 4, 11, 6, 12, 8, 12, 10, 11, 14, 8, 16, 7, 18, 7, 20, 8, 21, 10, -1, -1, 3, 8, 4, 10, 6, 11, 8, 11, 10, 10, 14, 7, 16, 6, 18, 6, 20, 7, 21, 10, 21, 12, ]) },
];
}
// fn get_font_char(font_name, char ascii_value, int& out_num_verts, int& out_horiz_dist)
// {
// if (1 | | ! strcmp(font_name, "hershey"))
// {
// if (ascii_value > = 32 & & ascii_value <= 126)
// {
// const int8_t * char_data = hershey_font_simplex_compact[ascii_value - 32];
// out_num_verts = char_data[0];
// out_horiz_dist = char_data[1];
// return char_data + 2;
// }
// }
// return NULL;
// }

const MOD_CAPS: u16 = 0x0001;
const MOD_SHIFT: u16 = 0x0002;
const MOD_CTRL: u16 = 0x0004;
const MOD_ALT: u16 = 0x0008;

/*pub(crate) fn keycode_to_ascii(keycode: u16, mods: u16) -> char {
    if mods & (MOD_CTRL | MOD_ALT) != 0 {
        // Do whatever we want here. Maybe launch VNC?
        return '\0';
    }
    return match keycode {
        KEY_SEMICOLON => if mods & MOD_SHIFT != 0 { ':' } else { ';' },
        KEY_OPEN_BRACKET => if mods & MOD_SHIFT != 0 { '{' } else { '[' },
        KEY_CLOSE_BRACKET => if mods & MOD_SHIFT != 0 { '}' } else { ']' },
        KEY_HYPHEN_MINUS => if mods & MOD_SHIFT != 0 { '_' } else { '-' },
        KEY_EQUALS => if mods & MOD_SHIFT != 0 { '+' } else { '=' },
        KEY_QUOTE => if mods & MOD_SHIFT != 0 { '\"' } else { '\'' },
        KEY_BACK_QUOTE => if mods & MOD_SHIFT != 0 { '~' } else { '`' },
        KEY_BACK_SLASH => if mods & MOD_SHIFT != 0 { '|' } else { '\\' },
        KEY_COMMA => if mods & MOD_SHIFT != 0 { '<' } else { ',' },
        KEY_PERIOD => if mods & MOD_SHIFT != 0 { '>' } else { '.' },
        KEY_SLASH => if mods & MOD_SHIFT != 0 { '?' } else { '/' },
        KEY_SPACE => ' ',
        KEY_0 => if mods & MOD_SHIFT != 0 { ')' } else { '0' },
        KEY_1 => if mods & MOD_SHIFT != 0 { '!' } else { '1' },
        KEY_2 => if mods & MOD_SHIFT != 0 { '@' } else { '2' },
        KEY_3 => if mods & MOD_SHIFT != 0 { '#' } else { '3' },
        KEY_4 => if mods & MOD_SHIFT != 0 { '$' } else { '4' },
        KEY_5 => if mods & MOD_SHIFT != 0 { '%' } else { '5' },
        KEY_6 => if mods & MOD_SHIFT != 0 { '^' } else { '6' },
        KEY_7 => if mods & MOD_SHIFT != 0 { '&' } else { '7' },
        KEY_8 => if mods & MOD_SHIFT != 0 { '*' } else { '8' },
        KEY_9 => if mods & MOD_SHIFT != 0 { '(' } else { '9' },
        KEY_A => if mods & MOD_CAPS != 0 { 'A' } else { 'a' },
        KEY_B => if mods & MOD_CAPS != 0 { 'B' } else { 'b' },
        KEY_C => if mods & MOD_CAPS != 0 { 'C' } else { 'c' },
        KEY_D => if mods & MOD_CAPS != 0 { 'D' } else { 'd' },
        KEY_E => if mods & MOD_CAPS != 0 { 'E' } else { 'e' },
        KEY_F => if mods & MOD_CAPS != 0 { 'F' } else { 'f' },
        KEY_G => if mods & MOD_CAPS != 0 { 'G' } else { 'g' },
        KEY_H => if mods & MOD_CAPS != 0 { 'H' } else { 'h' },
        KEY_I => if mods & MOD_CAPS != 0 { 'I' } else { 'i' },
        KEY_J => if mods & MOD_CAPS != 0 { 'J' } else { 'j' },
        KEY_K => if mods & MOD_CAPS != 0 { 'K' } else { 'k' },
        KEY_L => if mods & MOD_CAPS != 0 { 'L' } else { 'l' },
        KEY_M => if mods & MOD_CAPS != 0 { 'M' } else { 'm' },
        KEY_N => if mods & MOD_CAPS != 0 { 'N' } else { 'n' },
        KEY_O => if mods & MOD_CAPS != 0 { 'O' } else { 'o' },
        KEY_P => if mods & MOD_CAPS != 0 { 'P' } else { 'p' },
        KEY_Q => if mods & MOD_CAPS != 0 { 'Q' } else { 'q' },
        KEY_R => if mods & MOD_CAPS != 0 { 'R' } else { 'r' },
        KEY_S => if mods & MOD_CAPS != 0 { 'S' } else { 's' },
        KEY_T => if mods & MOD_CAPS != 0 { 'T' } else { 't' },
        KEY_U => if mods & MOD_CAPS != 0 { 'U' } else { 'u' },
        KEY_V => if mods & MOD_CAPS != 0 { 'V' } else { 'v' },
        KEY_W => if mods & MOD_CAPS != 0 { 'W' } else { 'w' },
        KEY_X => if mods & MOD_CAPS != 0 { 'X' } else { 'x' },
        KEY_Y => if mods & MOD_CAPS != 0 { 'Y' } else { 'y' },
        KEY_Z => if mods & MOD_CAPS != 0 { 'Z' } else { 'z' },
        _ => '�',
    };
}
*/

