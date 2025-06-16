extern crate kiss3d;
extern crate rand;

pub mod arc_ball;
pub mod pc;

use crate::pc::PhysicalConstants;
use crate::pc::PhysicalConstantsVec;

use crate::arc_ball::ArcBall;
use crate::kiss3d::camera::Camera;

use kiss3d::light::Light;
use kiss3d::nalgebra as na;
use kiss3d::window::Window;
use na::Point3;
use na::{Matrix4, Translation3, Vector4};

use kiss3d::conrod;
use kiss3d::conrod::position::Positionable;
use kiss3d::conrod::widget_ids;
use kiss3d::conrod::Labelable;

use rand::random;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::Split;

// use binary_stream::Endian;
// use serde::{Deserialize, Serialize};

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
    let mut reader = BufReader::with_capacity(20 * 1024 * 1024, f);
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

    let mut window = Window::new("GravSim Viz");

    window.conrod_ui_mut().theme = theme();
    let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());
    let physical_constants = PhysicalConstants::from_file_r_in_km("../spice/simbodiespc")?;
    let pc_earth = physical_constants.by_name("earth").unwrap();
    let sim_e_radius = 5.0 as f32;
    let scale_factor = sim_e_radius / pc_earth.radius;
    let mut app = DemoApp::new(physical_constants);

    let mut bodies = vec![];

    window.add_cone(2.0, 2.0 * sim_e_radius);
    let mut i = 0;
    let t = states[i].t;
    while t == states[i].t {
        let s = &states[i];
        if let Some(pc) = app.physical_constants.by_spkid(s.spkid) {
            bodies.push(window.add_sphere(pc.radius * scale_factor));
        } else {
            bodies.push(window.add_sphere(1.0));
        }
        bodies[i].set_color(random(), random(), random());
        i += 1;
    }
    println!("n_bodies={}", bodies.len());

    let eye = Point3::new(0.0, 0.0, 1200.0);
    let at = Point3::origin();
    let mut arc_ball =
        ArcBall::new_with_frustrum(std::f32::consts::PI / 40.0, 0.001, f32::MAX, eye, at);
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
    #[allow(unused_labels)]
    'renderloop: while window.render_with_camera(&mut arc_ball) {
        let mut ui = window.conrod_ui_mut().set_widgets();
        gui(&mut ui, &ids, &mut app);

        // let vt = arc_ball.transformation();
        // let vt = vt / vt[15];
        // let vt = vp * vt;
        for _j in 0..app.speed {
            t = states[i].t;
            while t >= states[i].t {
                let s = &states[i];
                let pos = Point3::new(s.x, s.y, s.z);
                let adjpos = (pos - local_center) * (scale_factor as f32);
                let mut adjpos = Point3::new(adjpos.x, adjpos.y, adjpos.z);
                if s.spkid == app.center_body_spkid {
                    local_center = pos;
                    adjpos = Point3::origin();
                }
                // let old = bodies[s.bodyid].data().local_translation().vector;
                // let v = Vector4::new(adjpos.x, adjpos.y, adjpos.z, 1.0);
                // let v = vt * v;
                let trans = Translation3::new(adjpos.x, adjpos.y, adjpos.z);
                bodies[s.bodyid].set_local_translation(trans);
                i += 1;
                if i >= states.len() {
                    i = 0;
                    break;
                    // break 'renderloop;
                }
            }
        }
    }
    Ok(())
}

pub fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

pub struct DemoApp {
    center_body_input: String,
    center_body_name: String,
    center_body_spkid: usize,
    physical_constants: PhysicalConstantsVec,
    speed: usize,
}

impl DemoApp {
    pub fn new(physical_constants: PhysicalConstantsVec) -> Self {
        DemoApp {
            center_body_name: "Earth".to_string(),
            center_body_input: "Earth".to_string(),
            center_body_spkid: 399,
            physical_constants: physical_constants,
            speed: 100,
        }
    }
}

widget_ids! {
    pub struct Ids {
        // The scrollable canvas.
        canvas,
        // The title and introduction widgets.
        title,
        introduction,
        // Shapes.
        shapes_canvas,
        rounded_rectangle,
        shapes_left_col,
        shapes_right_col,
        shapes_title,
        line,
        point_path,
        rectangle_fill,
        rectangle_outline,
        trapezoid,
        oval_fill,
        oval_outline,
        circle,
        // Image.
        image_title,
        // Button, XyPad, Toggle.
        button_title,
        button,
        xy_pad,
        toggle,
        ball,
        // NumberDialer, PlotPath
        dialer_title,
        number_dialer,
        plot_path,
        // TextBox and TextEdit
        text_box,
        text_edit,
        // Scrollbar
        canvas_scrollbar,
    }
}

pub fn gui(ui: &mut conrod::UiCell, ids: &Ids, app: &mut DemoApp) {
    use conrod::{widget, Sizeable, Widget};

    const MARGIN: conrod::Scalar = 30.0;
    const TITLE_SIZE: conrod::FontSize = 42;

    // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
    // By default, its size is the size of the window. We'll use this as a background for the
    // following widgets, as well as a scrollable container for the children widgets.
    const TITLE: &'static str = "All Widgets";
    widget::Canvas::new()
        .pad(MARGIN)
        .align_right()
        .w(240.0)
        .scroll_kids_vertically()
        .set(ids.canvas, ui);

    ////////////////////////////////
    ///// TextBox and TextEdit /////
    ////////////////////////////////
    widget::Text::new(TITLE)
        .font_size(TITLE_SIZE)
        .mid_top_of(ids.canvas)
        .set(ids.title, ui);

    for event in widget::TextBox::new(&app.center_body_input)
        .down_from(ids.title, 60.0)
        .align_middle_x_of(ids.canvas)
        .padded_w_of(ids.canvas, 10.0)
        .h(40.0)
        .set(ids.text_box, ui)
    {
        use conrod::widget::text_box::Event;
        match event {
            Event::Enter => {}
            Event::Update(s) => {
                if let Some(pc) = app.physical_constants.by_name(&s) {
                    app.center_body_name = pc.name;
                    app.center_body_spkid = pc.spkid;
                }
                app.center_body_input = s;
            }
        }
    }

    widget::Text::new(format!("{} ({})", app.center_body_name, app.center_body_spkid).as_str())
        .font_size(24)
        .mid_top_of(ids.canvas)
        .set(ids.title, ui);

    widget::Text::new(format!("Speed: {}", app.speed).as_str())
        .down_from(ids.text_box, 20.0)
        .font_size(12)
        .set(ids.button_title, ui);

    for _press in widget::Button::new()
        .label("+ Speed")
        .mid_left_with_margin_on(ids.canvas, MARGIN)
        .down_from(ids.text_box, 60.0)
        .w_h(20.0, 20.0)
        .set(ids.button, ui)
    {
        app.speed += 1;
    }
    for _press in widget::Button::new()
        .label("- Speed")
        .mid_right_with_margin_on(ids.canvas, MARGIN)
        .down_from(ids.text_box, 60.0)
        .w_h(20.0, 20.0)
        .set(ids.toggle, ui)
    {
        app.speed -= 1;
    }

    /////////////////////
    ///// Scrollbar /////
    /////////////////////

    widget::Scrollbar::y_axis(ids.canvas)
        .auto_hide(false)
        .set(ids.canvas_scrollbar, ui);
}
