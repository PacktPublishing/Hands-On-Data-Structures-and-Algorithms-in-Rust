use crate::data::*;
use crate::store::EcsStore;
use termion::raw::RawTerminal;
use termion::{color, cursor};

pub fn move_sys<D: EcsStore<Dir>, P: EcsStore<Pos>>(dd: &D, pp: &mut P) {
    pp.for_each_mut(|g, p| {
        if let Some(d) = dd.get(g) {
            p.x += d.vx;
            p.y += d.vy;
        }
    });
}

pub fn dir_sys<D: EcsStore<Dir>, P: EcsStore<Pos>>(dd: &mut D, pp: &P) {
    let (w, h) = termion::terminal_size().unwrap();
    let w = w as i32;
    let h = h as i32;
    dd.for_each_mut(|g, dr| {
        match rand::random::<u8>() % 5 {
            0 => dr.vx += 1,
            1 => dr.vx -= 1,
            2 => dr.vy += 1,
            3 => dr.vy -= 1,
            _ => {}
        }
        dr.vx = std::cmp::min(3, dr.vx);
        dr.vx = std::cmp::max(-3, dr.vx);
        dr.vy = std::cmp::min(3, dr.vy);
        dr.vy = std::cmp::max(-3, dr.vy);
        if let Some(p) = pp.get(g) {
            if p.x < 4 {
                dr.vx = 1
            }
            if p.x + 4 >= w {
                dr.vx = -1
            }
            if p.y < 4 {
                dr.vy = 1
            }
            if p.y + 4 >= h {
                dr.vy = -1
            }
        }
    });
}

pub fn collision_sys<P: EcsStore<Pos>, S: EcsStore<Strength>>(pp: &P, ss: &mut S) {
    let mut collisions = Vec::new();
    pp.for_each(|og, op| {
        let c = &mut collisions;
        pp.for_each(|ig, ip| {
            if (ip == op) && (ig != og) {
                &c.push((og, ig));
            }
        });
    });

    for (og, ig) in collisions {
        let dam = match ss.get(og) {
            Some(b) => b.s,
            None => continue,
        };
        let h_up = if let Some(bumpee) = ss.get_mut(ig) {
            let n = bumpee.s + 1;
            bumpee.h -= dam;
            if bumpee.h <= 0 {
                n
            } else {
                0
            }
        } else {
            0
        };
        if h_up > 0 {
            if let Some(bumper) = ss.get_mut(og) {
                bumper.h += h_up;
                bumper.s += 1;
            }
        }
    }
}

pub fn render_sys<T: std::io::Write, P: EcsStore<Pos>, S: EcsStore<Strength>>(
    t: &mut RawTerminal<T>,
    pp: &P,
    ss: &S,
) {
    write!(t, "{}", termion::clear::All);
    let (w, h) = termion::terminal_size().unwrap();
    pp.for_each(|g, p| {
        if let Some(st) = ss.get(g) {
            let col = match st.h {
                0 => format!("{}", color::Fg(color::Black)),
                1 => format!("{}", color::Fg(color::Red)),
                2 => format!("{}", color::Fg(color::Yellow)),
                3 => format!("{}", color::Fg(color::Green)),
                _ => format!("{}", color::Fg(color::Blue)),
            };
            let x = (p.x % (w as i32)) + 1;
            let y = (p.y % (h as i32)) + 1;
            write!(t, "{}{}{}", cursor::Goto(x as u16, y as u16), col, st.s);
        }
    });
}

pub fn death_sys<S: EcsStore<Strength>, P: EcsStore<Pos>, D: EcsStore<Dir>>(
    g: &mut crate::gen::GenManager,
    ss: &mut S,
    pp: &mut P,
    dd: &mut D,
) {
    let mut to_kill = Vec::new();
    let (w, h) = termion::terminal_size().unwrap();
    let w = w as i32;
    let h = h as i32;
    pp.for_each(|g, p| {
        if p.x > w || p.x < 0 || p.y > h || p.y < 0 {
            to_kill.push(g);
        }
    });

    ss.for_each(|g, s| {
        if s.h <= 0 {
            to_kill.push(g);
        }
    });

    for tk in to_kill {
        g.drop(tk);
        pp.drop(tk);
        ss.drop(tk);
        dd.drop(tk);
    }
}
