module 3d;

import math;

struct Vec3 {
  f64 x,y,z;
}

Vec3 Vec3.zero() => Vec3{x:0, y:0, z:0};

f64 length(Vec3 a) {
  return math.sqrt(a.x*a.x + a.y*a.y + a.z*a.z);
}

f64 dot(Vec3 a, Vec3 b) => a.x*b.x + a.y * b.y + a.z * b.z;


Vec3 +(Vec3 a, Vec3 b) => Vec3{x: a.x+b.x, y: a.y+b.y, z: a.z+b.z};
Vec3 -(Vec3 a, Vec3 b) => Vec3{x: a.x-b.x, y: a.y-b.y, z: a.z-b.z};

Vec3 scale(Vec3 a, f64 s) {
  return Vec3{
    x: a.x * s,
    y: a.y * s,
    z: a.z * s,
  };
}

Vec3 cross(Vec3 a, Vec3 b) {
  return Vec3{
    x: a.y*b.z - a.z*b.y;
    y: a.x*b.z - a.z*b.x;
    z: a.x*b.y - a.y*b.x;
  };
}