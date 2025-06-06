extern crate kiss3d;
extern crate rand;

pub mod arc_ball;

use crate::arc_ball::ArcBall;
use crate::kiss3d::camera::Camera;

use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::light::Light;
use kiss3d::nalgebra as na;
use kiss3d::text::Font;
use kiss3d::window::Window;
use na::Point2;
use na::Point3;
use na::{Matrix4, Translation3, Vector3, Vector4};

use rand::random;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::Split;

struct State {
    spkid: usize,
    bodyid: usize,
    t: f32,
    x: f32,
    y: f32,
    z: f32,
}

impl State {
    fn new(parts: &mut Split<'_, &str>) -> State {
        State {
            x: parts.next().unwrap().trim().parse::<f32>().unwrap(),
            y: parts.next().unwrap().trim().parse::<f32>().unwrap(),
            z: parts.next().unwrap().trim().parse::<f32>().unwrap(),
            bodyid: parts.next().unwrap().trim().parse::<usize>().unwrap(),
            t: parts.next().unwrap().trim().parse::<u64>().unwrap() as f32,
            spkid: parts.next().unwrap().trim().parse::<usize>().unwrap(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut states = vec![];
    let f = File::open("../gravsim/output.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut max_id = 0;
    while 0 != reader.read_line(&mut line)? {
        let mut parts = line.split(" ");
        let s = State::new(&mut parts);
        if max_id < s.bodyid {
            max_id = s.bodyid;
        }
        states.push(s);
        line.clear();
    }

    let mut window = Window::new("Kiss3d: primitives");

    let mut bodies = vec![];
    let font = Font::default();

    window.add_cone(2.0, 10.0);
    bodies.push(window.add_sphere(4.0));
    bodies[0].set_color(random(), random(), random());
    for i in 1..=max_id {
        bodies.push(window.add_sphere(4.0));
        bodies[i].set_color(random(), random(), random());
    }
    println!("n_bodies={}", bodies.len());

    let eye = Point3::new(0.0, 0.0, 120.0);
    let at = Point3::origin();
    let mut arc_ball =
        ArcBall::new_with_frustrum(std::f32::consts::PI / 4.0, 0.001, f32::MAX, eye, at);
    let hw = (window.width() / 2) as f32;
    let hh = (window.height() / 2) as f32;
    let vp = Matrix4::new(
        2.0 * hw,
        0.0,
        0.0,
        2.0 * hw,
        0.0,
        -2.0 * hh,
        0.0,
        2.0 * hh,
        0.0,
        0.0,
        0.5,
        0.5,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    //return Ok(());

    window.set_light(Light::StickToCamera);
    window.set_framerate_limit(Some(60));
    let mut t;
    let mut i = 0;
    let mut local_center = Point3::new(0.0, 0.0, 0.0);
    let center_body = 399;
    'renderloop: while window.render_with_camera(&mut arc_ball) {
        // arc_ball.set_at(local_center);
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    match button {
                        Key::Q => {
                            event.inhibited = true; // override the default keyboard handler
                            break 'renderloop;
                        }
                        Key::R => {
                            i = 0;
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        let vt = arc_ball.transformation();
        let vt = vt / vt[15];
        let vt = vp * vt;
        for _j in 0..100 {
            t = states[i].t;
            while t >= states[i].t {
                let s = &states[i];
                let pos = Point3::new(s.x, s.y, s.z);
                // let r = Vector3::new(local_center.x, local_center.y, local_center.z).norm();
                let r = 637100.0;
                let adjpos = (pos - local_center) / r;
                let mut adjpos = Point3::new(adjpos.x, adjpos.y, adjpos.z);
                if s.spkid == center_body {
                    local_center = pos;
                    adjpos = Point3::origin();
                    // window.draw_text(
                    //     format!("{}", s.bodyid).as_str(),
                    //     &Point2::origin(),
                    //     120.0,
                    //     &font,
                    //     &Point3::new(0.0, 1.0, 1.0),
                    // );
                }
                let v = Vector4::new(adjpos.x, adjpos.y, adjpos.z, 1.0);
                let v = vt * v;
                if 0.0 <= v[0] && v[0] < 2.0 * hw {
                    if 0.0 <= v[1] && v[1] < 2.0 * hh {
                        let p = Point2::new(v[0], v[1]);
                        window.draw_text(
                            format!("{}", s.spkid).as_str(),
                            &p,
                            72.0,
                            &font,
                            &Point3::new(0.0, 1.0, 1.0),
                        );
                    }
                }
                //}
                let trans = Translation3::new(adjpos.x, adjpos.y, adjpos.z);
                bodies[s.bodyid].set_local_translation(trans);
                i += 1;
                if i >= states.len() {
                    // i = 0;
                    // t = 0.0;
                    break 'renderloop;
                }
            }
        }
    }
    //             _ => (),
    //         }
    //     }
    // }
    Ok(())
}
