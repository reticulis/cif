// use image::ImageBuffer;
//
// pub struct Image {
//     buffer: T,
//     output_path: String
// }
//
// impl Image {
//     pub fn rgb(buf: Vec<[u8; 3]>, output_path: String) {
//         let mut imgbuf = ImageBuffer::new(cif.width, cif.height);
//         for (pixel, buf) in imgbuf.pixels_mut().zip(cif.buf_rgb) {
//             *pixel = image::Rgb(buf)
//         }
//         imgbuf.save(output_path).except("Error saving image to file!");
//     }
//     pub fn rgba(buf: Vec<[u8; 4]>, output_path: String) {
//         let mut imgbuf = ImageBuffer::new(cif.width, cif.height);
//         for (pixel, buf) in imgbuf.pixels_mut().zip(cif.buf_rgb) {
//             *pixel = image::Rgba(buf)
//         }
//         imgbuf.save(output_path).except("Error saving image to file!");
//     }
// }
