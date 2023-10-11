// // The Computer Language Benchmarks Game
// // http://benchmarksgame.alioth.debian.org/
// // contributed by Mike Pall
// // modified by Geoff Leyland
// // modified by Mario Pernici

const double PI = 3.141592653589793;

const double SOLAR_MASS = 4.0 * PI * PI;

const double DAYS_PER_YEAR = 365.24;

const int N = 1000;

DateTime start = DateTime.Now;

Planet sun = default;
Planet jupiter = default;
Planet saturn = default;
Planet uranus = default;
Planet neptune = default;

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

Planet[] bodies = new[] { sun, jupiter, saturn, uranus, neptune };

int nbody = bodies.Length;

offsetMomentum(bodies, nbody);
Console.WriteLine($"{energy(bodies, nbody)}");
for (int i = 0; i < N; i++)
{
    advance(bodies, nbody, 0.01);
}
Console.WriteLine($"{energy(bodies, nbody)}");

Console.WriteLine($"Took {DateTime.Now.Subtract(start).Microseconds}");

double invsqrt(double n)
{
    return Math.Pow(n, -0.5);
}

void advance(Planet[] bodies, int nbody, double dt)
{
    for (int i = 0; i < nbody; i++)
    {
        ref Planet bi = ref bodies[i];
        var (bix, biy, biz, bimass) = (bi.x, bi.y, bi.z, bi.mass);
        var (bivx, bivy, bivz) = (bi.vx, bi.vy, bi.vz);

        for (int j = i + 1; j < nbody; j++)
        {
            ref Planet bj = ref bodies[j];
            var (dx, dy, dz) = (bix - bj.x, biy - bj.y, biz - bj.z);
            var dist2 = (dx * dx) + (dy * dy) + (dz * dz);
            var mag = invsqrt(dist2);
            mag = dt * (mag / dist2);
            var bm = bj.mass * mag;
            bivx = bivx - (dx * bm);
            bivy = bivy - (dy * bm);
            bivz = bivz - (dz * bm);
            bm = bimass * mag;
            bj.vx = bj.vx + (dx * bm);
            bj.vy = bj.vy + (dy * bm);
            bj.vz = bj.vz + (dz * bm);
        }

        bi.vx = bivx;
        bi.vy = bivy;
        bi.vz = bivz;
        bi.x = bix + dt * bivx;
        bi.y = biy + dt * bivy;
        bi.z = biz + dt * bivz;
    }
}

double energy(Planet[] bodies, int nbody)
{
    var e = 0.0;
    for (int i = 0; i < nbody; i++)
    {
        ref Planet bi = ref bodies[i];
        var (vx, vy, vz, bim) = (bi.vx, bi.vy, bi.vz, bi.mass);
        e = e + (0.5 * bim * (vx * vx + vy * vy + vz * vz));
        for (int j = i + 1; j < nbody; j++)
        {
            ref Planet bj = ref bodies[j];
            var (dx, dy, dz) = (bi.x - bj.x, bi.y - bj.y, bi.z - bj.z);
            var distance = invsqrt(dx * dx + dy * dy + dz * dz);

            e = e - (bim * bj.mass * distance);
        }
    }

    return e;
}

void offsetMomentum(Planet[] b, int nbody)
{
    var (px, py, pz) = (0.0, 0.0, 0.0);
    for (int i = 0; i < nbody; i++)
    {
        ref Planet bi = ref b[i];
        var bim = bi.mass;
        px = px + (bi.vx * bim);

        py = py + (bi.vy * bim);
        pz = pz + (bi.vz * bim);
    }

    b[0].vx = -px / SOLAR_MASS;
    b[0].vy = -py / SOLAR_MASS;
    b[0].vz = -pz / SOLAR_MASS;
}

struct Planet
{
    public double x;
    public double y;
    public double z;
    public double vx;
    public double vy;
    public double vz;
    public double mass;
}