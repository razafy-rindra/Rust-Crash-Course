use std::{error::Error, io, time::{Duration, Instant}, sync::mpsc, thread::{self}};

use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Hide, Show}, ExecutableCommand, event::{self, KeyCode, Event}};
use invaders::{frame::{self, new_frame, Drawable}, render::{render, self}, player::{Player, self}, invaders::Invaders};
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>>{
    let mut audio = Audio ::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");


    // Initialise the terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?; //So we can accept keyboard input, ? -> crash if we have error
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?; // Hide the cursor
    
    //Render loop in a seperate thread
    let (render_tx, render_rx) = mpsc::channel(); // In real project should use crossbeam channels instead.
    let render_handle = thread::spawn(move ||{
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop{
            let curr_frame = match render_rx.recv(){
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop{ 
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();
        // Input 
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event::read()?{
                match key_event.code{
                    KeyCode::Left => {player.move_left();}
                    KeyCode::Right => {player.move_right();}
                    KeyCode::Char(' ') | KeyCode::Enter =>{
                        if player.shoot(){
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') =>{
                        audio.play("lose");
                        break 'gameloop;
                    } 

                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta){
            audio.play("move");
        }
        if player.detect_hits(&mut invaders){
            audio.play("explode");
        }

        // Draw and render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables{
            drawable.draw(&mut curr_frame); 
        }
        let _ = render_tx.send(curr_frame); //Ignore Error, we expect this will fail the first few times.
        thread::sleep(Duration::from_millis(1)); // Gameloop is faster then render, so do this to not render too many frames/sec
        
        // Win or lose?
        if invaders.all_killed(){
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom(){
            audio.play("lose");
            break 'gameloop;
        }
    }

    //Cleanuo
    drop(render_tx);
    audio.wait();
    stdout.execute(Show)?; // Show the cursor
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

// More features that can be added:

// Score
// Change playing field dimension
// Change logic, to make multi character ship and aliens.    