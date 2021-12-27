use euler::vec3;
use euler::Vec3;
use std::cmp::max;
use std::cmp::min;

struct Camera {
    pos: Vec3,
    dir: Vec3,
}

impl Camera {
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        let U: Vec3 = vec3!(1.0, 0.0, 0.0);
        let V: Vec3 = vec3!(0.0, 1.0, 0.0);
        let R: Ray = Ray {
            pos: self.pos,
            dir: (self.dir + u * U + v * V).normalize(),
        };
        R
    }
}

struct Ray {
    pos: Vec3,
    dir: Vec3,
}

struct Light {
    pos: Vec3,
    int: f32,
}

pub enum Object3d {
    Sphere(Sphere),
    Trig(Trig),
}

impl Object3d {
    pub fn is_ray_intersect(&self, R: &Ray) -> bool {
        match self {
            &Object3d::Sphere(ref s) => s.is_ray_intersect(R),
            &Object3d::Trig(ref s)   => s.is_ray_intersect(R),
        }
    }
    pub fn give_t(&self, R: &Ray) -> f32 {
        match self {
            &Object3d::Sphere(ref s) => s.give_t(R),
            &Object3d::Trig(ref s) => s.give_t(R),
        }
    }
    pub fn get_ray_brightness(&self, R: &Ray, L: &Vec<&Light>, O: &Vec<&Object3d>) -> usize {
        match self {
            &Object3d::Sphere(ref s) => s.get_ray_brightness(R, L, O),
            &Object3d::Trig(ref s) => s.get_ray_brightness(R, L, O),
        }
    }
}

struct Sphere {
    pos: Vec3,
    rad: f32,
}

impl Sphere {
    fn is_ray_intersect(&self, R: &Ray) -> bool {
        let v: Vec3 = R.pos - self.pos;
        let b: f32 = 2.0 * v.dot(R.dir);
        let c: f32 = v.dot(v) - self.rad * self.rad;
        let d: f32 = b * b - 4.0 * c;
        if d < 0.0 {
            return false;
        }
        let t0: f32 = (-b - d.sqrt()) / 2.0;
        let t1: f32 = (-b + d.sqrt()) / 2.0;
        if t0 > 0.0 || t1 > 0.0 {
            return true;
        }
        false
    }

    fn give_t(&self, R: &Ray) -> f32 {
        let v: Vec3 = R.pos - self.pos;
        let b: f32 = 2.0 * v.dot(R.dir);
        let c: f32 = v.dot(v) - self.rad * self.rad;
        let d: f32 = b * b - 4.0 * c;
        if d < 0.0 {
            return -1.0;
        }
        let t0: f32 = (-b - d.sqrt()) / 2.0;
        let t1: f32 = (-b + d.sqrt()) / 2.0;
        t0.min(t1)
    }

    fn get_ray_brightness(&self, R: &Ray, L: &Vec<&Light>, O: &Vec<&Object3d>) -> usize {
        if self.is_ray_intersect(R) == false {
            return 0;
        }
        let mut br: f32 = 0.;
        for l in L {
            let light_ray = Ray {
                pos: R.pos + R.dir * (self.give_t(R) - 0.001),
                dir: vec3!() - (R.pos + R.dir * self.give_t(R) - l.pos).normalize(),
            };
            let mut is_light_ray_intersect: bool = false;
            for o in O {
                is_light_ray_intersect = is_light_ray_intersect || o.is_ray_intersect(&light_ray);
            }
            if is_light_ray_intersect == false {
                br += l.int * (vec3!() - R.dir).dot(l.pos - self.pos);
            }
        }
        let br: i32 = br as i32;
        let br: usize = br as usize;
        br
    }
}

struct Trig {
    v0 : Vec3,
    v1 : Vec3,
    v2 : Vec3,
}

impl Trig {
    fn is_ray_intersect(&self, R: &Ray) -> bool {
        let norm : Vec3 = (self.v1 - self.v0).cross(self.v2 - self.v0);
        let A = norm.x; let B = norm.y; let C = norm.z;
        let D = - (A * self.v0.x + B * self.v0.y + C * self.v0.z);
        if A * R.dir.x + B * R.dir.y  + C * R.dir.z == 0.0 {return false;}
        let t = - (D + A * R.pos.x + B * R.pos.y + C * R.pos.z) / (A * R.dir.x + B * R.dir.y  + C * R.dir.z);
        if t < 0.0 {return false;}
        let M = R.pos + t * R.dir;
        let a = self.v0 - M;
        let b = self.v1 - M;
        let c = self.v2 - M;
        let base = (self.v2 - self.v0).cross(self.v1 - self.v0).normalize();
        if (a.cross(b).normalize() + base).length() > 0.01 {return false;}
        if (b.cross(c).normalize() + base).length() > 0.01 {return false;}
        if (c.cross(a).normalize() + base).length() > 0.01 {return false;}
        true
    }
    fn give_t(&self, R: &Ray) -> f32 {
        let norm : Vec3 = (self.v1 - self.v0).cross(self.v2 - self.v0);
        let A = norm.x; let B = norm.y; let C = norm.z;
        let D = - (A * self.v0.x + B * self.v0.y + C * self.v0.z);
        if A * R.dir.x + B * R.dir.y  + C * R.dir.z == 0.0 {return -1.0;}
        let t = - (D + A * R.pos.x + B * R.pos.y + C * R.pos.z) / (A * R.dir.x + B * R.dir.y  + C * R.dir.z);
        t
    }
    fn get_ray_brightness(&self, R: &Ray, L: &Vec<&Light>, O: &Vec<&Object3d>) -> usize {
        if self.is_ray_intersect(R) == false {
            return 0;
        }
        let mut br: f32 = 0.;
        for l in L {
            let light_ray = Ray {
                pos : R.pos + R.dir * (self.give_t(R) - 0.001),
                dir : vec3!() - (R.pos + R.dir * self.give_t(R) - l.pos).normalize(),
            };
            let mut is_light_ray_intersect: bool = false;
            for o in O {
                is_light_ray_intersect = is_light_ray_intersect || o.is_ray_intersect(&light_ray);
            }
            if is_light_ray_intersect == false {
                br += l.int * (vec3!() - R.dir).dot(l.pos - R.dir * self.give_t(R));
            }
        }
        let br: i32 = br as i32;
        let br: usize = br as usize;
        br
    }
}

fn cast_ray(O: Vec<&Object3d>, L: Vec<&Light>, R: &Ray) -> usize {
    let mut V: Vec<&Object3d> = Vec::new();
    for o in &O {
        if o.is_ray_intersect(R) == true {
            V.push(o)
        }
    }
    let mut mem: &Object3d;
    let memt: f32;
    if V.len() == 0 {
        return (0 as usize);
    }
    mem = V[0];
    memt = V[0].give_t(R);
    for v in V {
        if v.give_t(R) < memt {
            mem = v;
        }
    }
    mem.get_ray_brightness(R, &L, &O)
}

pub struct Play {
    width: usize,
    height: usize,
    gradient: Vec<char>,
    brightness: Vec<Vec<usize>>,
}

impl Play {
    pub fn new(w: usize, h: usize) -> Self {
        Play {
            width: w,
            height: h,
            gradient: " .:!/r(l1Z4H9W8$@".chars().collect(),
            brightness: vec![vec![1; w]; h],
        }
    }

    pub fn run(&mut self) {
        let mut t: f32 = 0.0;
        loop {
            self.render(t);
            print!("{}[2J", 27 as char);
            self.display();
            t += 0.01;
        }
    }

    fn clamp(&self, brightness: usize) -> usize {
        min(max(0, brightness), self.gradient.len() - 1)
    }

    fn get_brightness(cam: &Camera, u: f32, v: f32, t: f32) -> usize {
        let mut vertex = vec!(vec!(vec!(vec3!(); 2); 2); 2);
        vertex[0][0][0] = vec3!(-1.0,-1.0,-1.0);
        vertex[0][0][1] = vec3!(-1.0,-1.0, 1.0);
        vertex[0][1][0] = vec3!(-1.0, 1.0,-1.0);
        vertex[0][1][1] = vec3!(-1.0, 1.0, 1.0);
        vertex[1][0][0] = vec3!( 1.0,-1.0,-1.0);
        vertex[1][0][1] = vec3!( 1.0,-1.0, 1.0);
        vertex[1][1][0] = vec3!( 1.0, 1.0,-1.0);
        vertex[1][1][1] = vec3!( 1.0, 1.0, 1.0);
        
        let mut TT : Vec<Object3d> = vec!();

        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[0][0][0],
            v1: vertex[0][1][0],
            v2: vertex[0][0][1],
        });
        TT.push(T);
        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[0][1][1],
            v1: vertex[0][1][0],
            v2: vertex[0][0][1],
        });
        TT.push(T);
        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[1][0][0],
            v1: vertex[1][1][0],
            v2: vertex[1][0][1],
        });
        TT.push(T);
        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[1][1][1],
            v1: vertex[1][0][1],
            v2: vertex[1][1][0],
        });
        TT.push(T);


        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[0][0][0],
            v1: vertex[1][0][0],
            v2: vertex[0][0][1],
        });
        TT.push(T);
        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[1][0][1],
            v1: vertex[1][0][0],
            v2: vertex[0][0][1],
        });
        TT.push(T);
        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[0][1][0],
            v1: vertex[1][1][0],
            v2: vertex[0][1][1],
        });
        TT.push(T);
        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[1][1][1],
            v1: vertex[0][1][1],
            v2: vertex[1][1][0],
        });
        TT.push(T);


        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[0][0][0],
            v1: vertex[0][1][0],
            v2: vertex[1][0][0],
        });
        TT.push(T);
        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[1][1][0],
            v1: vertex[0][1][0],
            v2: vertex[1][0][0],
        });
        TT.push(T);
        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[0][0][1],
            v1: vertex[0][1][1],
            v2: vertex[1][0][1],
        });
        TT.push(T);
        let T : Object3d = Object3d::Trig(Trig {
            v0: vertex[1][1][1],
            v1: vertex[1][0][1],
            v2: vertex[0][1][1],
        });
        TT.push(T);


        let T : Object3d = Object3d::Trig(Trig {
            v0: vec3!(),
            v1: vec3!(),
            v2: vec3!(),
        });
        TT.push(T);


        let R: Ray = cam.get_ray(u, v);
        let L: Light = Light {
            pos: vec3!(2.0, 2.0,-10.0),
            int: 1.0,
        };
        let mut tt : Vec<&Object3d> = Vec::new();
        for i in 0..TT.len()-1 {
            tt.push(&TT[i]);
        }
        cast_ray(tt, vec![&L], &R)
    }

    fn render(&mut self, t: f32) {
        let cam: Camera = Camera {
            pos: vec3!( 2.0, 2.0,-3.0),  
            dir: vec3!(-1.0,-1.0, 1.0).normalize(),
        };
        let mut u: f32;
        let mut v: f32;
        for i in 0..self.height {
            for j in 0..self.width {
                u = (i as f32) / (self.height as f32) * 2.0 - 1.0;
                v = ((j as f32) / (self.width as f32) * 2.0 - 1.0)
                    * (self.width as f32 / self.height as f32)
                    * 11.0
                    / 24.0;
                self.brightness[i][j] = Play::get_brightness(&cam, u, v, t);
            }
        }
    }

    fn display(&self) {
        let mut s: String = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
                let ch: char = self.gradient[self.clamp(self.brightness[i][j])];
                s.push(ch);
            }
            s.push_str("\n");
        }
        print!("{}", s);
    }
}
