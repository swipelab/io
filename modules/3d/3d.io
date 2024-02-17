module 3d;

import math { sqrt };

type Vec3 : struct{
  x,y,z: f64;
}

impl Vec3 {
  operator(+): (b: Vec3) -> Vec3 => Vec3{x: self.x+b.x, y: self.y+b.y, z: self.z+b.z}
  operator(-): (b: Vec3) -> Vec3 => Vec3{x: self.x-b.x, y: self.y-b.y, z: self.z-b.z}
}

Vec3.zero : () -> Vec3 => Vec3{x:0, y:0, z:0}

length: (a: Vec3) -> f64 => sqrt(a.x*a.x + a.y*a.y + a.z*a.z);
dot: (a, b: Vec3) -> f64 => a.x*b.x + a.y * b.y + a.z * b.z;

scale: (a: Vec3, s: f64) -> Vec3 =>
  Vec3{
    x: a.x * s,
    y: a.y * s,
    z: a.z * s,
  }

cross: (a, b: Vec3) -> Vec3 =>
  Vec3{
    x: a.y*b.z - a.z*b.y,
    y: a.x*b.z - a.z*b.x,
    z: a.x*b.y - a.y*b.x,
  };