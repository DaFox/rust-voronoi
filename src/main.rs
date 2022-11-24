
use std::fs::File;
use std::io::Write;
use std::io::BufWriter;

type Color = u32;
type Coord = (usize, usize);

#[derive(Copy, Clone, Debug)]
struct Point {
    color: Color,
    pos: Coord
}

struct Canvas {
   pixel: [[u32; WIDTH]; HEIGHT],
   points: Vec<Point>
}

// we store each pixel as 0xRRGGBBAA. we will find out if this was
// wise when we try to animate the stuff ...

impl Canvas {
   fn new() -> Self {
       Self {
           pixel: [[0u32; WIDTH]; HEIGHT],
           points: vec![]
       }
   }

   fn pixels(&self) -> Vec<u32> {
       self.pixel
	 .iter()
	 .flat_map(|row| row.iter())
         .cloned()
	 .collect()
   }

   fn point(&mut self, pos: Coord, width: u8, color: Color) {
       // TODO: make sure pos is in bounds
       let offset: usize = (width/2).into();

       for i in pos.0-offset..pos.0+offset {
           for j in pos.1-offset..pos.1+offset {
              self.pixel[i][j] = COLOR_BLACK;
           }
       }

       self.points.push(Point { color, pos });
   }

   fn fill(&mut self, color: Color) {
       for i in 0..HEIGHT {
          for j in 0..WIDTH {
              self.pixel[i][j] = color;
//println!("W: {}, H: {}, {:?}", i, j, self.pixel[i][j]);
          }
       }
   }

   fn sqr_dst(a: Coord, b: Coord) -> f32 {
       // sqrt(a*a + b*b) = c
       let ax:i32 = (b.0 as i32 - a.0 as i32);
       let ay:i32 = (b.1 as i32 - a.1 as i32);

       ((ax * ax + ay * ay) as f32).sqrt()
   }
  
   fn fill_voronoi_cell(&mut self, p: &Point) {
       for i in 0..HEIGHT {
           for j in 0..WIDTH {
              for o in self.points.iter() {
                  if Self::sqr_dst((i, j), p.pos) < Self::sqr_dst((i, j), o.pos) {
                      self.pixel[i][j] = p.color;
                  }
               }
           }
       }
   }


   fn fill_voronoi_areas(&mut self) {
//       self.fill(points[0].color);

      for i in 0..HEIGHT {
        for j in 0..WIDTH {
          let mut points = self.points.iter();
          let mut last = points.next().unwrap();
          
          self.pixel[i][j] = last.color;

          for p in points {
              if Self::sqr_dst((i, j), p.pos) < Self::sqr_dst((i, j), last.pos) {
                  self.pixel[i][j] = p.color;
                  last = p;
              }
          }

        }
      }


//            self.fill_voronoi_cell(&p);
/*           for i in 0..HEIGHT {
               for j in 0..WIDTH {
                   if Self::sqr_dst((i, j), p.pos) < Self::sqr_dst((i, j), last.pos) {
                       self.pixel[i][j] = p.color;
                   }
               }
           }

           last = p;*/
//       }
   }

//   fn as_u6(byte: u32) -> [u8; 3] {

//   }

   fn save_as_netpbm(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
       let mut file = BufWriter::new(File::create(path)?);
       let head = format!("P6\n{} {} 255\n", WIDTH, HEIGHT);

       file.write_all(&head.as_bytes());

           for byte in self.pixels() {
               file.write_all(&      [
         (((byte & 0xFF000000) >> 24) as u8),  
         ((byte & 0x00FF0000) >> 16) as u8,
         ((byte & 0x0000FF00) >> 8)  as u8,
      ]);

          }

       file.flush()?;

       Ok(())
   }
}

const WIDTH: usize = 400;
const HEIGHT: usize = 600;
const POINTS: usize = 24;

const COLOR_WHITE: Color = 0xFFFFFFFF;
const COLOR_RED: Color   = 0xFF000000;
const COLOR_BLUE: Color  = 0x0000FF00;
const COLOR_GREEN: Color = 0x00FF0000;
const COLOR_BLACK: Color = 0x00000000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let palette = vec![
        COLOR_RED,
        COLOR_BLUE,
        COLOR_GREEN,
        0xDDDDDD00,
        0xFFFF0000,
        0xEFEFEF00,
        0xFF00FF00,
        0xA0A0A000,
        0xBBAAFF00
    ];

    let mut canvas = Canvas::new();
    canvas.fill(COLOR_WHITE);
    
    for i in 0..=POINTS {
        let x = (rand::random::<f32>() * WIDTH as f32) as usize;
        let y = (rand::random::<f32>() * HEIGHT as f32) as usize;

        canvas.point((y, x), 2, palette[i % palette.len()]);
    }

//    canvas.point((320, 300), 2, COLOR_RED);
//    canvas.point((120, 290), 2, COLOR_GREEN);
    canvas.fill_voronoi_areas();
    canvas.save_as_netpbm("foobar.ppm")?;

    Ok(())
}
 
