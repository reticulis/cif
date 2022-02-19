// use image::ImageBuffer;
//
// pub struct Image {
//     buffer: T,
//     output_path: String
// }
//
// impl Image {
//     pub fn new(buffer: T, output_path: String) {
//         let mut imgbuf = ImageBuffer::new(cif.width, cif.height);
//         for (pixel, buf) in imgbuf.pixels_mut().zip(cif.buf_rgb) {
//             *pixel = image::Rgb(buf)
//         }
//         imgbuf.save(output_path).except("Error saving image to file!");
//     }
// }
