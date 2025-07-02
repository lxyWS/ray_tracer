use crate::{
    rtweekend::random_int,
    vec3::{Point3, Vec3, dot, unit_vector},
};

#[derive(Debug)]
pub struct Perlin {
    // rand_float: Vec<f64>,
    randvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

const POINT_COUNT: usize = 256;

impl Perlin {
    pub fn new() -> Self {
        // let mut rand_float = Vec::with_capacity(POINT_COUNT);
        let mut randvec = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            // rand_float.push(random_double());
            randvec.push(unit_vector(Vec3::random_range(-1.0, 1.0)));
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Perlin {
            // rand_float,
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = Vec::with_capacity(POINT_COUNT);
        for i in 0..POINT_COUNT {
            p.push(i);
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut [usize], n: usize) {
        for i in (1..n).rev() {
            let target = random_int(0, i as i32) as usize;
            p.swap(i, target);
        }
    }

    #[inline]
    fn fade(t: f64) -> f64 {
        t * t * (3.0 - 2.0 * t)
    }

    // fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    //     let mut accum = 0.0;
    //     for i in 0..2 {
    //         for j in 0..2 {
    //             for k in 0..2 {
    //                 let i_factor = i as f64 * u + (1.0 - i as f64) * (1.0 - u);
    //                 let j_factor = j as f64 * v + (1.0 - j as f64) * (1.0 - v);
    //                 let k_factor = k as f64 * w + (1.0 - k as f64) * (1.0 - w);
    //                 accum += i_factor * j_factor * k_factor * c[i][j][k];
    //             }
    //         }
    //     }
    //     accum
    // }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = Self::fade(u);
        let vv = Self::fade(v);
        let ww = Self::fade(w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    let i_factor = i as f64 * uu + (1.0 - i as f64) * (1.0 - uu);
                    let j_factor = j as f64 * vv + (1.0 - j as f64) * (1.0 - vv);
                    let k_factor = k as f64 * ww + (1.0 - k as f64) * (1.0 - ww);
                    accum += i_factor * j_factor * k_factor * dot(&c[i][j][k], &weight_v);
                }
            }
        }
        accum
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        // let uu = Self::fade(u);
        // let vv = Self::fade(v);
        // let ww = Self::fade(w);

        // let i = ((4.0 * p.x()) as i32 & 255) as usize;
        // let j = ((4.0 * p.y()) as i32 & 255) as usize;
        // let k = ((4.0 * p.z()) as i32 & 255) as usize;

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x_index = ((i + di) & 255) as usize;
                    let y_index = ((j + dj) & 255) as usize;
                    let z_index = ((k + dk) & 255) as usize;

                    let index = self.perm_x[x_index] ^ self.perm_y[y_index] ^ self.perm_z[z_index];
                    c[di as usize][dj as usize][dk as usize] = self.randvec[index];
                }
            }
        }
        // let index = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
        // self.rand_float[index]
        Self::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }
}
