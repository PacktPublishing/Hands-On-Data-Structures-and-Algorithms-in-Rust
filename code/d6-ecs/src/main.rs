mod gen;
mod store;
mod systems;
mod data;


use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use store::EcsStore;
use std::io::Write;

fn main() {
    println!("Hello, world!");

    let (ch_s,ch_r) = std::sync::mpsc::channel();
    std::thread::spawn(move ||{
        let stdin = std::io::stdin();
        for k in stdin.keys() {
            ch_s.send(k) ;
        }
    });
    let (w,h) = termion::terminal_size().unwrap();
    let mut screen = std::io::stdout().into_raw_mode().unwrap();

    let mut gen = gen::GenManager::new();
    let mut strengths = store::VecStore::new();
    let mut dirs = store::VecStore::new();
    let mut poss = store::VecStore::new();
    let mut pass = 0;

    //eventy loop
    loop {
        //create random
        let g = gen.next();
        strengths.add(g,data::Strength{s:1,h:5});
        dirs.add(g,data::Dir{vx:0,vy:0});
        poss.add(g,data::Pos{x:(rand::random::<u16>()%w)as i32,y:(rand::random::<u16>()%h)as i32});

        systems::dir_sys(&mut dirs,&poss);         
        systems::move_sys(&dirs,&mut poss);
        systems::collision_sys(&poss,&mut strengths);
        systems::death_sys(&mut gen,&mut strengths,&mut poss,&mut dirs);
        systems::render_sys(&mut screen,&poss , &strengths);

        write!(&mut screen,"{}PASS={}",termion::cursor::Goto(1,1),pass);
        pass +=1;

        screen.flush();
               


        std::thread::sleep(std::time::Duration::from_millis(300));

        while let Ok(Ok(k)) = ch_r.try_recv(){
            match k {
                Key::Char('q')=>return,
                _=>{},
            }
        }
    }

}
