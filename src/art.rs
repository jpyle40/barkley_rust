use crate::game::Game;
use barkely_highlands::{
    mission::{Friend, Tile},
    simulation::{Direction, Pos},
};
use macroquad::prelude::*;

pub const W: f32 = 1280.;
pub const H: f32 = 900.;
pub const INK: Color = color_u8!(37, 62, 53, 255);
pub const MUTED: Color = color_u8!(103, 119, 101, 255);
pub const PAPER: Color = color_u8!(250, 247, 235, 255);
pub const CREAM: Color = color_u8!(239, 235, 218, 255);
pub const GREEN: Color = color_u8!(44, 103, 77, 255);
pub const GOLD: Color = color_u8!(228, 172, 66, 255);
pub const BLUE: Color = color_u8!(72, 134, 159, 255);
pub const BX: f32 = 64.;
pub const BY: f32 = 224.;
pub const CELL: f32 = 75.;

pub fn round(r: Rect, radius: f32, color: Color) {
    let radius = radius.min(r.w / 2.).min(r.h / 2.);
    draw_rectangle(r.x + radius, r.y, r.w - 2. * radius, r.h, color);
    draw_rectangle(r.x, r.y + radius, r.w, r.h - 2. * radius, color);
    for (x, y) in [
        (r.x + radius, r.y + radius),
        (r.x + r.w - radius, r.y + radius),
        (r.x + radius, r.y + r.h - radius),
        (r.x + r.w - radius, r.y + r.h - radius),
    ] {
        draw_circle(x, y, radius, color);
    }
}
pub fn label(text: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(text, x, y, size, color);
}
pub fn centered(text: &str, r: Rect, size: f32, color: Color) {
    let d = measure_text(text, None, size as u16, 1.);
    label(
        text,
        r.x + (r.w - d.width) / 2.,
        r.y + (r.h + d.height) / 2. - 2.,
        size,
        color,
    );
}
pub fn wrap(text: &str, x: f32, mut y: f32, width: f32, size: f32, color: Color) -> f32 {
    let mut line = String::new();
    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_string()
        } else {
            format!("{line} {word}")
        };
        if measure_text(&candidate, None, size as u16, 1.).width > width && !line.is_empty() {
            label(&line, x, y, size, color);
            y += size * 1.25;
            line = word.into();
        } else {
            line = candidate;
        }
    }
    if !line.is_empty() {
        label(&line, x, y, size, color);
        y += size * 1.25;
    }
    y
}
pub fn point(p: Pos) -> Vec2 {
    vec2(
        BX + (p.x as f32 + 0.5) * CELL,
        BY + (p.y as f32 + 0.5) * CELL,
    )
}
pub fn tile_at(p: Vec2) -> Option<Pos> {
    let x = ((p.x - BX) / CELL).floor() as i32;
    let y = ((p.y - BY) / CELL).floor() as i32;
    if (0..9).contains(&x) && (0..7).contains(&y) {
        Some(Pos::new(x, y))
    } else {
        None
    }
}
pub fn paw(p: Vec2, s: f32, color: Color) {
    draw_ellipse(p.x, p.y + 4. * s, 7. * s, 6. * s, 0., color);
    for (x, y) in [(-7., -3.), (-2.5, -7.), (3., -7.), (8., -2.)] {
        draw_ellipse(p.x + x * s, p.y + y * s, 2.8 * s, 3.6 * s, x * 2., color);
    }
}
pub fn tree(p: Vec2, s: f32, seed: usize) {
    draw_ellipse(
        p.x + 6. * s,
        p.y + 21. * s,
        27. * s,
        11. * s,
        0.,
        Color::new(0.15, 0.28, 0.17, 0.18),
    );
    round(
        Rect::new(p.x - 5. * s, p.y - 1. * s, 10. * s, 24. * s),
        3. * s,
        color_u8!(127, 91, 53, 255),
    );
    let dark = if seed.is_multiple_of(3) {
        color_u8!(45, 100, 70, 255)
    } else {
        color_u8!(58, 117, 72, 255)
    };
    draw_circle(p.x, p.y - 13. * s, 28. * s, dark);
    draw_circle(p.x - 15. * s, p.y - 13. * s, 18. * s, dark);
    draw_circle(p.x + 15. * s, p.y - 16. * s, 19. * s, dark);
    draw_circle(
        p.x - 4. * s,
        p.y - 31. * s,
        21. * s,
        color_u8!(86, 140, 77, 255),
    );
    draw_circle(
        p.x - 12. * s,
        p.y - 28. * s,
        12. * s,
        color_u8!(111, 159, 88, 255),
    );
    draw_circle(
        p.x + 12. * s,
        p.y - 15. * s,
        13. * s,
        color_u8!(77, 134, 75, 255),
    );
    for i in 0..4 {
        let x = p.x + ((seed * 7 + i * 11) % 35) as f32 * s - 17. * s;
        let y = p.y - ((seed + i * 13) % 36) as f32 * s;
        draw_ellipse(x, y, 3. * s, 1.5 * s, -30., color_u8!(143, 174, 95, 180));
    }
}
pub fn flower(p: Vec2, seed: usize) {
    draw_line(
        p.x,
        p.y,
        p.x + 1.,
        p.y - 5.,
        1.,
        color_u8!(79, 131, 68, 255),
    );
    let c = [
        color_u8!(255, 234, 157, 255),
        color_u8!(235, 174, 157, 255),
        color_u8!(246, 246, 222, 255),
    ][seed % 3];
    for i in 0..5 {
        let a = i as f32 * std::f32::consts::TAU / 5.;
        draw_circle(p.x + a.cos() * 2.5, p.y - 6. + a.sin() * 2.5, 2.4, c);
    }
    draw_circle(p.x, p.y - 6., 1.8, GOLD);
}
pub fn dog(
    p: Vec2,
    direction: Direction,
    s: f32,
    mudge: bool,
    time: f32,
    moving: bool,
    carrying: bool,
) {
    let facing = match direction {
        Direction::East => vec2(1., 0.),
        Direction::South => vec2(0., 1.),
        Direction::West => vec2(-1., 0.),
        Direction::North => vec2(0., -1.),
    };
    let side = vec2(-facing.y, facing.x);
    let at = |x: f32, y: f32| p + (facing * x + side * y) * s;
    let fur = if mudge {
        color_u8!(48, 43, 39, 255)
    } else {
        color_u8!(252, 248, 229, 255)
    };
    let brown = if mudge {
        color_u8!(164, 102, 54, 255)
    } else {
        color_u8!(151, 89, 47, 255)
    };
    let angle = facing.y.atan2(facing.x).to_degrees();
    draw_ellipse(
        p.x,
        p.y + 12. * s,
        25. * s,
        13. * s,
        0.,
        Color::new(0.12, 0.22, 0.12, 0.22),
    );
    let stride = if moving { (time * 19.).sin() * 4. } else { 0. };
    for (x, y, k) in [
        (-11., -10., 1.),
        (10., -10., -1.),
        (-11., 10., -1.),
        (10., 10., 1.),
    ] {
        let foot = at(x + stride * k, y);
        draw_circle(foot.x, foot.y, 5. * s, if mudge { brown } else { fur });
    }
    let tail = at(-29., (time * 9.).sin() * 5.);
    let rump = at(-15., 0.);
    draw_line(rump.x, rump.y, tail.x, tail.y, 5. * s, fur);
    draw_ellipse(p.x, p.y, 23. * s, 13. * s, angle, fur);
    if !mudge {
        let spot = at(-9., -4.);
        draw_ellipse(spot.x, spot.y, 9. * s, 7. * s, angle, brown);
    }
    let neck = at(13., 0.);
    draw_line(
        at(11., -11.).x,
        at(11., -11.).y,
        at(11., 11.).x,
        at(11., 11.).y,
        4. * s,
        if mudge {
            GOLD
        } else {
            color_u8!(198, 81, 58, 255)
        },
    );
    draw_circle(neck.x, neck.y, 11. * s, fur);
    let head = at(22., 0.);
    draw_circle(head.x, head.y, 12. * s, fur);
    for y in [-10., 10.] {
        let ear = at(17., y);
        let tip = at(9., y * 1.4);
        let base = at(25., y * 0.65);
        draw_triangle(ear, tip, base, brown);
    }
    let patch = at(24., -4.);
    draw_circle(patch.x, patch.y, 7. * s, brown);
    let muzzle = at(31., 1.);
    draw_ellipse(
        muzzle.x,
        muzzle.y,
        8. * s,
        6. * s,
        angle,
        if mudge { brown } else { fur },
    );
    let nose = at(36., 0.);
    draw_circle(nose.x, nose.y, 3. * s, INK);
    for y in [-5., 5.] {
        let eye = at(26., y);
        draw_circle(eye.x, eye.y, 2.2 * s, INK);
        draw_circle(eye.x + 0.6 * s, eye.y - 0.6 * s, 0.6 * s, WHITE);
    }
    if carrying {
        let ball = at(42., 0.);
        draw_circle(ball.x, ball.y, 7. * s, GOLD);
        draw_circle_lines(
            ball.x,
            ball.y,
            7. * s,
            1.5 * s,
            color_u8!(164, 106, 36, 255),
        );
    }
}
pub fn person(p: Vec2, friend: Friend, s: f32, time: f32, walking: bool) {
    let adult = friend != Friend::Oliver;
    let height = if adult { 1.2 } else { 0.9 };
    let s = s * height;
    let hair = match friend {
        Friend::Oliver => color_u8!(238, 112, 34, 255),
        Friend::Henry => color_u8!(239, 198, 76, 255),
        Friend::Betsey => color_u8!(100, 61, 37, 255),
    };
    let shirt = match friend {
        Friend::Oliver => color_u8!(80, 143, 166, 255),
        Friend::Henry => color_u8!(64, 104, 130, 255),
        Friend::Betsey => color_u8!(186, 111, 89, 255),
    };
    let skin = color_u8!(239, 189, 144, 255);
    let stride = if walking {
        (time * 12.).sin() * 3. * s
    } else {
        0.
    };
    draw_ellipse(
        p.x,
        p.y + 11. * s,
        15. * s,
        6. * s,
        0.,
        Color::new(0.12, 0.22, 0.12, 0.18),
    );
    if friend == Friend::Betsey {
        round(
            Rect::new(p.x - 13. * s, p.y - 43. * s, 26. * s, 32. * s),
            9. * s,
            hair,
        );
    }
    for sign in [-1., 1.] {
        draw_line(
            p.x + sign * 5. * s,
            p.y - 3. * s,
            p.x + sign * 6. * s + stride * sign,
            p.y + 10. * s,
            6. * s,
            color_u8!(56, 73, 76, 255),
        );
        draw_ellipse(
            p.x + sign * 6. * s + stride * sign,
            p.y + 11. * s,
            5. * s,
            3. * s,
            0.,
            INK,
        );
        draw_line(
            p.x + sign * 11. * s,
            p.y - 23. * s,
            p.x + sign * 15. * s,
            p.y - 9. * s - stride * sign,
            5. * s,
            skin,
        );
    }
    round(
        Rect::new(p.x - 10. * s, p.y - 28. * s, 20. * s, 27. * s),
        5. * s,
        shirt,
    );
    draw_circle(p.x, p.y - 37. * s, 11. * s, skin);
    draw_ellipse(p.x, p.y - 45. * s, 12. * s, 7. * s, 0., hair);
    draw_circle(p.x - 8. * s, p.y - 41. * s, 5. * s, hair);
    if friend == Friend::Oliver {
        for i in 0..4 {
            draw_triangle(
                vec2(p.x - 9. * s + i as f32 * 5. * s, p.y - 45. * s),
                vec2(p.x - 8. * s + i as f32 * 5. * s, p.y - 54. * s),
                vec2(p.x - 2. * s + i as f32 * 5. * s, p.y - 46. * s),
                hair,
            );
        }
    }
    for x in [-4., 4.] {
        draw_circle(p.x + x * s, p.y - 36. * s, 1.3 * s, INK);
    }
    draw_line(
        p.x - 3. * s,
        p.y - 30. * s,
        p.x + 3. * s,
        p.y - 30. * s,
        1.2 * s,
        color_u8!(151, 94, 69, 255),
    );
}
pub fn background(time: f32) {
    clear_background(PAPER);
    for i in 0..90 {
        let x = ((i * 167 + 37) % 1280) as f32;
        let y = ((i * 131 + 83) % 900) as f32;
        draw_circle(x, y, 0.8, color_u8!(144, 152, 119, 25));
    }
    draw_ellipse(1200., -80., 320., 270., 0., color_u8!(216, 225, 195, 100));
    for i in 0..3 {
        let x = 720. + i as f32 * 31. + (time * 0.4).sin() * 4.;
        let y = 47. + (i % 2) as f32 * 7.;
        draw_line(x - 5., y - 3., x, y, 1.5, MUTED);
        draw_line(x, y, x + 5., y - 3., 1.5, MUTED);
    }
}
pub fn board(g: &Game, time: f32) {
    let m = g.mission();
    round(
        Rect::new(BX - 10., BY - 10. + 5., CELL * 9. + 20., CELL * 7. + 20.),
        20.,
        color_u8!(61, 90, 62, 35),
    );
    round(
        Rect::new(BX - 10., BY - 10., CELL * 9. + 20., CELL * 7. + 20.),
        20.,
        color_u8!(203, 215, 172, 255),
    );
    draw_rectangle(BX, BY, CELL * 9., CELL * 7., color_u8!(161, 187, 120, 255));
    for y in 0..7 {
        for x in 0..9 {
            let pos = Pos::new(x, y);
            let p = point(pos);
            let tile = m.tile(pos);
            let seed = (x * 71 + y * 133 + g.index as i32 * 19) as usize;
            let r = Rect::new(p.x - CELL / 2., p.y - CELL / 2., CELL, CELL);
            if matches!(tile, Tile::Water | Tile::Bridge) {
                draw_rectangle(r.x, r.y, r.w, r.h, color_u8!(101, 169, 179, 255));
                for j in 0..5 {
                    let yy = r.y + 9. + j as f32 * 14.;
                    let xx = r.x + 6. + (time * 1.3 + j as f32).sin() * 4.;
                    draw_line(xx, yy, xx + 27., yy, 2., color_u8!(175, 211, 207, 210));
                }
            } else {
                let tint = ((seed % 9) as f32 - 4.) / 255.;
                draw_rectangle(
                    r.x,
                    r.y,
                    r.w,
                    r.h,
                    Color::new(0.635 + tint, 0.737 + tint, 0.475 + tint, 1.),
                );
                for j in 0..7 {
                    let xx = r.x + ((seed + j * 23) % 70 + 4) as f32;
                    let yy = r.y + ((seed / 3 + j * 37) % 68 + 5) as f32;
                    draw_line(xx, yy, xx - 2., yy - 4., 1., color_u8!(111, 150, 85, 100));
                    draw_line(xx, yy, xx + 2., yy - 3., 1., color_u8!(111, 150, 85, 100));
                    if j < 2 && tile == Tile::Grass {
                        flower(vec2(xx, yy), seed + j);
                    }
                }
            }
            if matches!(tile, Tile::Path | Tile::Switch | Tile::Gate) {
                round(
                    Rect::new(r.x + 1., r.y + 5., CELL - 2., CELL - 10.),
                    17.,
                    color_u8!(208, 198, 149, 255),
                );
                draw_ellipse(
                    p.x - 10.,
                    p.y + 13.,
                    4.,
                    2.,
                    -10.,
                    color_u8!(180, 173, 124, 170),
                );
                draw_ellipse(
                    p.x + 18.,
                    p.y - 16.,
                    3.,
                    2.,
                    20.,
                    color_u8!(230, 218, 176, 200),
                );
            }
            if tile == Tile::Bridge {
                draw_rectangle(r.x, r.y + 7., CELL, CELL - 14., color_u8!(117, 91, 55, 255));
                for j in 0..6 {
                    round(
                        Rect::new(r.x + 3. + j as f32 * 12., r.y + 9., 10., CELL - 18.),
                        2.,
                        color_u8!(194, 155, 97, 255),
                    );
                }
                for yy in [r.y + 5., r.y + CELL - 6.] {
                    draw_line(
                        r.x - 2.,
                        yy,
                        r.x + CELL + 2.,
                        yy,
                        5.,
                        color_u8!(121, 91, 54, 255),
                    );
                }
            }
            if tile == Tile::Switch {
                draw_circle(p.x, p.y, 18., if g.sim.gate_open { GREEN } else { GOLD });
                paw(p, 0.85, PAPER);
            }
            if g.progress.show_grid {
                draw_rectangle_lines(r.x, r.y, r.w, r.h, 1., Color::new(1., 1., 1., 0.23));
            }
        }
    }
    // Route markers are visible before execution so timing can be reasoned about.
    for actor in &m.patrols {
        route(&actor.route, color_u8!(125, 85, 61, 180));
    }
    if !m.friend_route.is_empty() {
        route(&m.friend_route, color_u8!(67, 113, 149, 180));
    }
    for p in g.sim.trail.iter().skip(1) {
        let p = point(*p);
        draw_circle(p.x, p.y + 21., 3.5, color_u8!(60, 115, 75, 160));
    }
    let home = point(m.destination);
    draw_ellipse(
        home.x,
        home.y + 15.,
        29.,
        12.,
        0.,
        color_u8!(250, 226, 143, 220),
    );
    draw_ellipse_lines(
        home.x,
        home.y + 15.,
        29.,
        12.,
        0.,
        2.,
        color_u8!(184, 137, 59, 255),
    );
    if let Some(p) = g.prediction {
        flag(point(p), GOLD);
    }
    if let Some(item) = g.sim.item
        && g.sim.revealed
    {
        let p = point(item);
        draw_ellipse(p.x, p.y + 11., 12., 5., 0., color_u8!(83, 118, 61, 130));
        draw_circle(p.x, p.y + 2. + (time * 3.).sin() * 1.5, 10., GOLD);
        draw_circle_lines(p.x, p.y + 2., 10., 2., PAPER);
    }
    // Draw obstacles and characters in row order for a clear overhead storybook view.
    for y in 0..7 {
        for x in 0..9 {
            let pos = Pos::new(x, y);
            let p = point(pos);
            match m.tile(pos) {
                Tile::Tree => tree(p, 0.92, (x * 7 + y * 13) as usize),
                Tile::Rock => {
                    draw_ellipse(
                        p.x + 4.,
                        p.y + 12.,
                        24.,
                        10.,
                        0.,
                        color_u8!(96, 118, 87, 120),
                    );
                    draw_poly(p.x, p.y, 7, 24., 20., color_u8!(145, 152, 129, 255));
                    draw_poly(
                        p.x - 4.,
                        p.y - 6.,
                        5,
                        16.,
                        0.,
                        color_u8!(177, 181, 153, 255),
                    );
                }
                Tile::Bench => {
                    for xx in [-21., 21.] {
                        draw_line(p.x + xx, p.y - 16., p.x + xx, p.y + 19., 5., INK);
                    }
                    for yy in [-17., -6., 8.] {
                        round(
                            Rect::new(p.x - 30., p.y + yy, 60., 8.),
                            2.,
                            color_u8!(167, 115, 63, 255),
                        );
                    }
                }
                Tile::Fence | Tile::Gate => {
                    if m.tile(pos) != Tile::Gate || !g.sim.gate_open {
                        for xx in [-25., -8., 9., 26.] {
                            round(
                                Rect::new(p.x + xx - 4., p.y - 27., 8., 54.),
                                3.,
                                color_u8!(160, 127, 76, 255),
                            );
                        }
                        for yy in [-16., 13.] {
                            draw_line(
                                p.x - 30.,
                                p.y + yy,
                                p.x + 30.,
                                p.y + yy,
                                5.,
                                color_u8!(201, 170, 114, 255),
                            );
                        }
                        if m.tile(pos) == Tile::Gate {
                            draw_circle(p.x + 17., p.y, 4., GOLD);
                        }
                    } else {
                        draw_line(
                            p.x - 31.,
                            p.y - 30.,
                            p.x - 31.,
                            p.y + 28.,
                            6.,
                            color_u8!(160, 127, 76, 255),
                        );
                    }
                }
                _ => {}
            }
        }
    }
    let t = g.animation * g.animation * (3. - 2. * g.animation);
    let friend_before = point(m.friend_at(g.previous.tick));
    let friend_now = point(m.friend_at(g.sim.tick));
    let fp = friend_before.lerp(friend_now, t);
    person(
        fp,
        m.friend,
        0.95,
        time,
        friend_before != friend_now && g.animation < 1.,
    );
    badge(m.friend.name(), fp + vec2(-34., 27.), 68., BLUE);
    for actor in &m.patrols {
        let before = actor.at(g.previous.tick);
        let now = actor.at(g.sim.tick);
        let p = point(before).lerp(point(now), t);
        let direction = direction_between(before, now);
        dog(
            p,
            direction,
            1.0,
            true,
            time,
            before != now && g.animation < 1.,
            false,
        );
        badge(
            "Mudge",
            p + vec2(-30., 27.),
            60.,
            color_u8!(116, 84, 59, 255),
        );
        let next = point(actor.at(g.sim.tick + 1));
        draw_circle_lines(next.x, next.y, 26., 2., color_u8!(117, 76, 51, 180));
    }
    let bp = point(g.previous.pos).lerp(point(g.sim.pos), t);
    let moving = g.previous.pos != g.sim.pos && g.animation < 1.;
    let bounce = if g.celebration > 0. {
        (time * 8.).sin().abs() * 5.
    } else {
        0.
    };
    dog(
        bp - vec2(0., bounce),
        g.sim.direction,
        0.82,
        false,
        time,
        moving,
        g.sim.carrying,
    );
    let ahead = point(g.sim.direction.ahead(g.sim.pos)) - point(g.sim.pos);
    let v = ahead.normalize();
    let side = vec2(-v.y, v.x);
    let arrow = bp + v * 34.;
    draw_triangle(
        arrow + v * 8.,
        arrow - v * 4. + side * 5.,
        arrow - v * 4. - side * 5.,
        PAPER,
    );
    badge("Barkely", bp + vec2(-33., 27.), 66., GREEN);
    if g.predicting {
        round(Rect::new(BX + 50., BY + 10., 602., 48.), 14., PAPER);
        centered(
            "Where will Barkely finish? Click a spot to plant your flag.",
            Rect::new(BX + 50., BY + 10., 602., 48.),
            20.,
            INK,
        );
    }
    if g.celebration > 0. {
        for i in 0..36 {
            let x = BX + ((i * 79) % 700) as f32;
            let y = BY + ((time * 45. + i as f32 * 37.) % 540.);
            draw_rectangle(
                x,
                y,
                5.,
                8.,
                [GOLD, PAPER, color_u8!(213, 135, 106, 255)][i % 3],
            );
        }
    }
}
fn route(points: &[Pos], color: Color) {
    for i in 0..points.len() {
        let a = point(points[i]);
        let b = point(points[(i + 1) % points.len()]);
        draw_line(a.x, a.y, b.x, b.y, 2., color);
    }
    for (i, p) in points.iter().enumerate() {
        let p = point(*p);
        let duplicate = points[..i].iter().filter(|&&q| q == points[i]).count();
        draw_circle(p.x - 21. + duplicate as f32 * 14., p.y - 22., 8., PAPER);
        label(
            &(i + 1).to_string(),
            p.x - 25. + duplicate as f32 * 14.,
            p.y - 18.,
            13.,
            color,
        );
    }
}
pub fn direction_between(a: Pos, b: Pos) -> Direction {
    if b.x > a.x {
        Direction::East
    } else if b.x < a.x {
        Direction::West
    } else if b.y < a.y {
        Direction::North
    } else {
        Direction::South
    }
}
pub fn badge(text: &str, p: Vec2, width: f32, color: Color) {
    round(Rect::new(p.x, p.y, width, 20.), 8., color);
    centered(text, Rect::new(p.x, p.y, width, 20.), 14., PAPER);
}
pub fn flag(p: Vec2, color: Color) {
    draw_line(p.x - 17., p.y + 10., p.x - 17., p.y - 30., 3., INK);
    draw_triangle(
        vec2(p.x - 16., p.y - 30.),
        vec2(p.x + 9., p.y - 23.),
        vec2(p.x - 16., p.y - 15.),
        color,
    );
}
