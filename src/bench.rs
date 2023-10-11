// // The Computer Language Benchmarks Game
// // http://benchmarksgame.alioth.debian.org/
// // contributed by Mike Pall
// // modified by Geoff Leyland
// // modified by Mario Pernici

const PI: f64 = std::f64::consts::PI;

const SOLAR_MASS: f64 = 4.0 * PI * PI;

const DAYS_PER_YEAR: f64 = 365.24;

fn invsqrt(n: f64) -> f64 {
    n.powf(-0.5)
}

fn advance(bodies: &mut [Planet], nbody: usize, dt: f64) {
    for i in 0..nbody {
        let bi = &mut bodies[i];
        let (bix, biy, biz, bimass) = (bi.x, bi.y, bi.z, bi.mass);
        let (mut bivx, mut bivy, mut bivz) = (bi.vx, bi.vy, bi.vz);

        for j in (i + 1)..nbody {
            let bj = &mut bodies[j];
            let (dx, dy, dz) = (bix - bj.x, biy - bj.y, biz - bj.z);
            let dist2 = (dx * dx) + (dy * dy) + (dz * dz);
            let mut mag = invsqrt(dist2);
            mag = dt * (mag / dist2);
            let mut bm = bj.mass * mag;
            bivx = bivx - (dx * bm);
            bivy = bivy - (dy * bm);
            bivz = bivz - (dz * bm);
            bm = bimass * mag;
            bj.vx = bj.vx + (dx * bm);
            bj.vy = bj.vy + (dy * bm);
            bj.vz = bj.vz + (dz * bm);
        }

        let bi = &mut bodies[i];
        bi.vx = bivx;
        bi.vy = bivy;
        bi.vz = bivz;
        bi.x = bix + dt * bivx;
        bi.y = biy + dt * bivy;
        bi.z = biz + dt * bivz;
    }
}

fn energy(bodies: &[Planet], nbody: usize) -> f64 {
    let mut e = 0.0;
    for i in 0..nbody {
        let bi = &bodies[i];
        let (vx, vy, vz, bim) = (bi.vx, bi.vy, bi.vz, bi.mass);
        e = e + (0.5 * bim * (vx * vx + vy * vy + vz * vz));
        for j in (i + 1)..nbody {
            let bj = &bodies[j];
            let (dx, dy, dz) = (bi.x - bj.x, bi.y - bj.y, bi.z - bj.z);
            let distance = invsqrt(dx * dx + dy * dy + dz * dz);

            e = e - ((bim * bj.mass) * distance);
        }
    }

    return e;
}

fn offsetMomentum(b: &mut [Planet], nbody: usize) {
    let (mut px, mut py, mut pz) = (0.0, 0.0, 0.0);
    for i in 0..nbody {
        let bi = &b[i];
        let bim = bi.mass;
        px = px + (bi.vx * bim);

        py = py + (bi.vy * bim);
        pz = pz + (bi.vz * bim);
    }

    b[0].vx = -px / SOLAR_MASS;
    b[0].vy = -py / SOLAR_MASS;
    b[0].vz = -pz / SOLAR_MASS;
}

const N: usize = 1000;

#[derive(Default)]
struct Planet {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    mass: f64,
}

pub fn main() {
    let mut sun = Planet::default();
    let mut jupiter = Planet::default();
    let mut saturn = Planet::default();
    let mut uranus = Planet::default();
    let mut neptune = Planet::default();

    sun.x = 0.0;
    sun.y = 0.0;
    sun.z = 0.0;
    sun.vx = 0.0;
    sun.vy = 0.0;
    sun.vz = 0.0;
    sun.mass = SOLAR_MASS;
    jupiter.x = 4.84143144246472090;
    jupiter.y = -1.16032004402742839;
    jupiter.z = -0.103622044471123109;
    jupiter.vx = 0.00166007664274403694 * DAYS_PER_YEAR;
    jupiter.vy = 0.00769901118419740425 * DAYS_PER_YEAR;
    jupiter.vz = -0.0000690460016972063023 * DAYS_PER_YEAR;
    jupiter.mass = 0.000954791938424326609 * SOLAR_MASS;
    saturn.x = 8.34336671824457987;
    saturn.y = 4.12479856412430479;
    saturn.z = -0.403523417114321381;
    saturn.vx = -0.00276742510726862411 * DAYS_PER_YEAR;
    saturn.vy = 0.00499852801234917238 * DAYS_PER_YEAR;
    saturn.vz = 0.0000230417297573763929 * DAYS_PER_YEAR;
    saturn.mass = 0.000285885980666130812 * SOLAR_MASS;
    uranus.x = 12.8943695621391310;
    uranus.y = -15.1111514016986312;
    uranus.z = -0.223307578892655734;
    uranus.vx = 0.00296460137564761618 * DAYS_PER_YEAR;
    uranus.vy = 0.00237847173959480950 * DAYS_PER_YEAR;
    uranus.vz = -0.0000296589568540237556 * DAYS_PER_YEAR;
    uranus.mass = 0.0000436624404335156298 * SOLAR_MASS;
    neptune.x = 15.3796971148509165;
    neptune.y = -25.9193146099879641;
    neptune.z = 0.179258772950371181;
    neptune.vx = 0.00268067772490389322 * DAYS_PER_YEAR;
    neptune.vy = 0.00162824170038242295 * DAYS_PER_YEAR;
    neptune.vz = -0.0000951592254519715870 * DAYS_PER_YEAR;
    neptune.mass = 0.0000515138902046611451 * SOLAR_MASS;

    let bodies = &mut [sun, jupiter, saturn, uranus, neptune];

    let nbody = bodies.len();

    offsetMomentum(bodies, nbody);
    println!("{}", energy(bodies, nbody));
    for _ in 0..N {
        advance(bodies, nbody, 0.01);
    }
    println!("{}", energy(bodies, nbody));
}
