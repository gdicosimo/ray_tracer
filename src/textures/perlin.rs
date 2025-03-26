use super::math::*;

const POINT_COUNT: usize = 256;

pub(crate) struct Perlin {
    rand_unitvecs: [UnitVec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let rand_unitvecs: [UnitVec3; POINT_COUNT] =
            std::array::from_fn(|_| UnitVec3::unchecked_random());

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();

        Perlin {
            rand_unitvecs,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn smooth_noise(&self, p: &Point3) -> f32 {
        let uvw = p.as_array().map(|coord| coord - coord.floor());

        let ijk = p.as_array().map(|coord| coord.floor() as i32);

        self.perlin_interp(uvw, ijk)
    }

    pub fn turbulence(&self, mut point: Point3, depth: usize) -> f32 {
        let mut accum = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.smooth_noise(&point).abs();
            weight *= 0.5;
            point *= 2.0;
        }

        accum.abs()
    }

    #[inline(always)]
    fn get_gradient(&self, i: usize, j: usize, k: usize) -> &UnitVec3 {
        let combined_index = self.perm_x[i & 255] ^ self.perm_y[j & 255] ^ self.perm_z[k & 255];

        &self.rand_unitvecs[combined_index as usize]
    }

    fn perlin_interp(&self, [u, v, w]: [f32; 3], [i, j, k]: [i32; 3]) -> f32 {
        let mut accum = 0.0;

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let grad =
                        self.get_gradient((i + di) as usize, (j + dj) as usize, (k + dk) as usize);

                    let d = Vec3::new(u - di as f32, v - dj as f32, w - dk as f32);

                    let weight = (di as f32 * uu + (1 - di) as f32 * (1.0 - uu))
                        * (dj as f32 * vv + (1 - dj) as f32 * (1.0 - vv))
                        * (dk as f32 * ww + (1 - dk) as f32 * (1.0 - ww));

                    accum += weight * grad.dot(&d);
                }
            }
        }
        accum
    }
}

fn perlin_generate_perm() -> [i32; POINT_COUNT] {
    let mut p: [i32; POINT_COUNT] = std::array::from_fn(|i| i as i32);
    permute(&mut p);
    p
}

#[inline]
#[allow(clippy::manual_swap)]
fn permute(p: &mut [i32]) {
    for i in 0..POINT_COUNT {
        let target = random_int_beetwen(0.0, 1.0);
        // p.swap(i, target);
        p[i] ^= p[target];
        p[target] ^= p[i];
        p[i] ^= p[target];
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
