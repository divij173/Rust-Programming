#[allow(dead_code)]
#[allow(unused_variables)]
use ron::from_str;
use termgame::{SimpleEvent, Controller, Game, GameEvent, GameSettings, StyledCharacter, run_game, KeyCode, ViewportLocation, SCREEN_WIDTH, SCREEN_HEIGHT, KeyEventKind, GameStyle, GameColor};
use std::error;
use termgame::Message;
// use std::fmt::Error;
use std::{error::Error, path};
use std::time::Duration;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
// use ron::de::from_str;


#[derive(Debug, Deserialize, Serialize)]

enum RonType {
    blocky(String),
}

#[derive(Debug, Deserialize, Serialize)]
struct MyStruct {
    numb_coor: (i32,i32),
    block_type: RonType,
}

#[derive(Debug, Clone)]
struct MyGame {
    /// The x coordinate
    pub x: i32,
    /// The y coordinate.
    pub y: i32,
    /// Vector for storing blocks and their locations
    pub vec1: Vec<(i32,i32,String,String)>,
    /// variable to count the number of steps taken in water
    pub water_count: i32,
}

impl Controller for MyGame {
    /// As soon as the game loads this function gets called
    /// Loads the whole map and starts the game at initial stage
    fn on_start(&mut self, game: &mut Game){
        
        /// This bellow comment is for implementing the blocks that i tried using different approach

        // for i in 0..SCREEN_HEIGHT
        // for i in 0..SCREEN_HEIGHT

        // {
        //     for j in 0.. SCREEN_WIDTH
        //     {
                
        //         game.set_screen_char(i as i32, j as i32,Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Black)))));
        //     }
        // }
        // let path = Path::new("../maps/testing_game.ron");
        // barricade
        // for i in 0..SCREEN_HEIGHT{
        //     game.set_screen_char(5, i as i32 , Some(StyledCharacter::new('♟')));
        // }

        /// get the command line arguments
        /// and store them in a vector
        let args: Vec<String> = std::env::args().collect();
        let pattern = &args[1];
        // println!("{:?}",args[1]);
        let content = std::fs::read_to_string(pattern).unwrap();
        let len1= content.len()-3;
        let mut insta = &content[1..len1];
        let mut splity = insta.split("\n");
        let mut v = Vec::new();
        let mut v2 = Vec::new();
        let mut v3 = Vec::new();
        let mut v4 = Vec::new();
        let mut v_temp = Vec::new();

        /// Geting the ron file into string vector
        while true
        {   let temp=splity.next();
            
            match temp{
                Some(x) => {v.push(x);}
                None => {break;}            
            };
        }
        // println!("{:?}",v);
        v.remove(0);
        v_temp=v.clone();
        // println!("{:?}",v_temp);
        let mut track_numb=0;
        
        /// Iterating each string ele in vec form of ron file
        /// storing each block type and location of block type in vector
        for ele in v
        {   
            //for the first line
            if track_numb == 0
            {
                let len1= ele.len()-1;
                let numb1 = &ele[4..len1];
                track_numb+=1;
                let mut splity = numb1.split(": ");
                
                let temp=splity.next();
                
                /// Spliting and storing co-ordinates and type in different vectors 
                match temp{
                    Some(x) => {
                        let len2 = x.len()-1;
                        let dil = &x[1..len2];
                        let mut splity1 = dil.split(", ");

                        /// Splitting and storing the x and y coordinates in different vectors
                        let mut temp1=splity1.next();
                        match temp1{
                        Some(x) => {
                            let my_int = x.parse::<i32>().unwrap();
                            v2.push(my_int);
                        }   
                        None => {break;}
                        }
                        
                        let mut temp1=splity1.next();
                        match temp1{
                        Some(x) => {
                            let my_int = x.parse::<i32>().unwrap();
                            v3.push(my_int);
                        }   
                        None => {break;}
                        }
                        
                    }
                    None => {break;}            
                };

                /// Storing the block type in vector 4
                let temp=splity.next();
                
                match temp{
                    Some(x) => {v4.push(x);}
                    None => {break;}            
                };
                
                // println!("{:?} {:?} {:?}", v2,v3,v4);
            }
            else {
                /// for the rest of the lines
                let len1= ele.len()-2;
                let numb1 = &ele[4..len1];
                // println!("{:?}",numb1);
                let mut splity = numb1.split(": ");
                let temp=splity.next();
                
                /// Spliting and storing co-ordinates and type in different vectors 
                match temp{
                    Some(x) => {
                        let len2 = x.len()-1;
                        let dil = &x[1..len2];
                        let mut splity1 = dil.split(", ");

                        let mut temp1=splity1.next();

                        /// Splitting and storing the x and y coordinates in different vectors
                        match temp1{
                        Some(x) => {
                            let my_int = x.parse::<i32>().unwrap();
                            v2.push(my_int);
                        }   
                        None => {break;}
                        }
                        
                        let mut temp1=splity1.next();
                        match temp1{
                        Some(x) => {
                            let my_int = x.parse::<i32>().unwrap();
                            v3.push(my_int);
                        }   
                        None => {break;}
                        }
                        
                    }
                    None => {break;}                    
                };

                let temp=splity.next();
                /// Storing the type in vector 4
                match temp{
                    Some(x) => {v4.push(x);}
                    None => {break;}            
                };
                // println!("{:?} {:?} {:?}", v2,v3,v4);
            }
            // println!("{ele}");
        }

        // println!("{:?} {:?} {:?}", v2,v3,v4);
        
        /// Placing the block types at the specified position on the given map 
        let mut index_numb = 0;
        for ele in v4{
            if ele.contains("Sign")
            {
                let leny2 = ele.len()-1;
                let mut ahm = &ele[6..leny2];
                // game.set_message(Some(ahm));
                // println!("{}",ahm);
                game.set_screen_char(v2[index_numb], v3[index_numb], Some(StyledCharacter::new('📨').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                self.vec1.push((v2[index_numb], v3[index_numb], ele.to_string(), ahm.to_string()));
            }
            else if ele.contains("Obj")
            {
                let leny2 = ele.len()-1;
                let ahm = &ele[8..leny2];
                let ahm2: Vec<char> = ahm.chars().collect();
                game.set_screen_char(v2[index_numb], v3[index_numb], Some(StyledCharacter::new(ahm2[0])));
                // println!("{:?}",ahm2);
                self.vec1.push((v2[index_numb], v3[index_numb], ele.to_string(), ahm2[0].to_string()));
            }
            else if ele.contains("Grass")
            {
                game.set_screen_char(v2[index_numb], v3[index_numb], Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Green)))));   
                self.vec1.push((v2[index_numb], v3[index_numb], ele.to_string(), "Green".to_string()));
            }
            else if ele.contains("Sand")
            {
                game.set_screen_char(v2[index_numb], v3[index_numb], Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Yellow)))));   
                self.vec1.push((v2[index_numb], v3[index_numb], ele.to_string(), "Yellow".to_string()));
            }
            else if ele.contains("Rocks")
            {
                game.set_screen_char(v2[index_numb], v3[index_numb], Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Gray)))));   
                self.vec1.push((v2[index_numb], v3[index_numb], ele.to_string(), "Gray".to_string()));
            }
            else if ele.contains("Cinderblock")
            {
                game.set_screen_char(v2[index_numb], v3[index_numb], Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightRed)))));   
                self.vec1.push((v2[index_numb], v3[index_numb], ele.to_string(), "LightRed".to_string()));
            }
            else if ele.contains("Flower")
            {
                game.set_screen_char(v2[index_numb], v3[index_numb], Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Magenta)))));   
                self.vec1.push((v2[index_numb], v3[index_numb], ele.to_string(), "Magenta".to_string()));
            }
            else if ele.contains("Water")
            {
                game.set_screen_char(v2[index_numb], v3[index_numb], Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightBlue)))));   
                self.vec1.push((v2[index_numb], v3[index_numb], ele.to_string(), "LightBlue".to_string()));
            }
            else if ele.contains("Barrier")
            {
                game.set_screen_char(v2[index_numb], v3[index_numb], Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::White)))));   
                self.vec1.push((v2[index_numb], v3[index_numb], ele.to_string(), "White".to_string()));
            }
            index_numb+=1;

        }
        
        // game.set_screen_char(v2[0], v3[0], Some(StyledCharacter::new('♟')));
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            /// When key 'e' is pressed 
            /// The game will end immediately
            SimpleEvent::Just(KeyCode::Char('e')) => {
            game.end_game();
            }

            /// When key 'r' is pressed 
            /// The game will be reset immediately
            /// the player will be set to initial position and the viewport will be set accordingly and the water count will be set to 0
            SimpleEvent::Just(KeyCode::Char('r')) => {
                let vec1_clone = self.vec1.clone();
                /// setting the default color of background blocks as soon the reset key is hit
                for ele in vec1_clone
                {
                    if ele.0 == self.x && ele.1 == self.y
                    {
                        if ele.2.contains("Obj")
                        {
                            let ahm2: Vec<char> = ele.3.chars().collect();
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(ahm2[0]).style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            // game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ')));
                            break;
                        }
                        else if ele.2.contains("Sign")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('📨').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            break;
                        }
                        else if ele.2.contains("Grass")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Green)))));
                            break;
                        }
                        else if ele.2.contains("Sand")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Yellow)))));
                            break;
                        }
                        else if ele.2.contains("Rocks")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Gray)))));
                            break;
                        }
                        else if ele.2.contains("Cinderblock")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightRed)))));
                            break;
                        }
                        else if ele.2.contains("Flower")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Magenta)))));
                            break;
                        }
                        else if ele.2.contains("Water")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightBlue)))));
                            break;
                        }
                        else if ele.2.contains("Barrier")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::White)))));
                            break;
                        }
                        else {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ')));
                            break;
                        }
                    }
                } 

                /// setting the players position to initial position
                /// And setting the viewport to default position
                game.set_screen_char(3, 3, Some(StyledCharacter::new('♟')));
                game.set_viewport(ViewportLocation { x: (0), y: (0) });
                self.x=3;
                self.y=3;
                self.water_count=0;
            }

            /// show the player to begin the game
            /// sets the player to initial positon 
            SimpleEvent::Just(KeyCode::Char('t')) => {
                game.set_screen_char(3, 3, Some(StyledCharacter::new('♟')));
                // game.set_viewport(ViewportLocation { x: (-3), y: (-3) });
                // let temp = game.get_viewport();
                // println!("{:?}",temp);
            },

            /// when the 'up' arrow is pressed, the player moves in the up direction
            /// all the postion hurdles will be checked at each iteration
            SimpleEvent::Just(KeyCode::Up) => {
                /// check if player has travelled 10 blocks of water 
                /// if yes the game will end instantly
                if self.water_count == 10
                {
                    game.end_game();
                }
                let mut checky=0;
                let vec1_clone = self.vec1.clone();
                
                /// set the default color background as soon as the player changes or moves from its postion
                for ele in vec1_clone
                {
                    if ele.0 == self.x && ele.1 == self.y
                    {
                        if ele.2.contains("Obj")
                        {
                            // let ahm2: Vec<char> = ele.3.chars().collect();
                            // game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(ahm2[0]).style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sign")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('📨').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            game.set_message(None);
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Grass")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Green)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sand")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Yellow)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Rocks")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Gray)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Cinderblock")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightRed)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Flower")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Magenta)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Water")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightBlue)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Barrier")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::White)))));
                            checky+=1;
                            break;
                        }
                        else {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ')));
                            checky+=1;
                            break;
                        }
                    }
                } 
                
                if checky==0
                {game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ')));}

                
                self.y=self.y-1;
                let coor=game.get_viewport();
                if coor.y>self.y
                {
                    // println!("{}, {}he", coor.x, coor.y);
                    game.set_viewport(ViewportLocation { x: (coor.x), y: (coor.y-1) });
                }

                let mut checky=0;
                let vec1_clone = self.vec1.clone();

                /// setting the default color and player as soon as the player goes to the postion according to the key
                for ele in vec1_clone
                {
                    if ele.0 == self.x && ele.1 == self.y
                    {
                        if ele.2.contains("Obj")
                        {
                            let ahm2: Vec<char> = ele.3.chars().collect();
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sign")
                        {
                            game.set_screen_char(self.x, self.y , Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            let z: Option<Message> = Some(Message::new(String::from("You can breathe here don't worry :)")).title(String::from("Message")));
                            game.set_message(z);
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Grass")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Green)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sand")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Yellow)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Rocks")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Gray)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Cinderblock")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::LightRed)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Flower")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Magenta)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Water")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::LightBlue)))));
                            self.water_count+=1;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Barrier")
                        {
                            self.y=self.y+1;
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::White)))));
                            checky+=1;
                            break;
                        }
                        checky+=1;
                        break;
                    }

                }
                /// if no match then set the screen as empty
                if checky==0
                {game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟')));}
                
                if self.water_count==10
                {
                    let z: Option<Message> = Some(Message::new(String::from("You Drowned :(")).title(String::from("Message")));
                    game.set_message(z);
                }
                // println!("{}", char_loc1.x);
            },

            /// when the 'down' arrow is pressed, the player moves in the up direction
            /// all the postion hurdles will be checked at each iteration
            SimpleEvent::Just(KeyCode::Down) => {
                /// check if player has travelled 10 blocks of water 
                /// if yes the game will end instatnly
                if self.water_count == 10
                {
                    game.end_game();
                }
                let mut checky=0;
                let vec1_clone = self.vec1.clone();
                /// set the default color background as soon as the player changes or moves from its postion
                for ele in vec1_clone
                {
                    if ele.0 == self.x && ele.1 == self.y
                    {
                        if ele.2.contains("Obj")
                        {
                            // let ahm2: Vec<char> = ele.3.chars().collect();
                            // game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(ahm2[0]).style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sign")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('📨').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            game.set_message(None);
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Grass")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Green)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sand")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Yellow)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Rocks")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Gray)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Cinderblock")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightRed)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Flower")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Magenta)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Water")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightBlue)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Barrier")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::White)))));
                            checky+=1;
                            break;
                        }
                        else {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ')));
                            checky+=1;
                            break;
                        }
                    }   
                }

                if checky==0
                {game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ')));}

                
                self.y=self.y+1;
                let coor=game.get_viewport();
                if self.y-coor.y>20
                {
                    // println!("{}, {}he", coor.x, coor.y);
                    game.set_viewport(ViewportLocation { x: (coor.x), y: (coor.y+1) });
                }

                let mut checky=0;
                let vec1_clone = self.vec1.clone();
                /// setting the default color and player as soon as the player goes to the postion according to the key
                for ele in vec1_clone
                {
                    if ele.0 == self.x && ele.1 == self.y
                    {
                        if ele.2.contains("Obj")
                        {
                            let ahm2: Vec<char> = ele.3.chars().collect();
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sign")
                        {
                            game.set_screen_char(self.x, self.y , Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            let z: Option<Message> = Some(Message::new(String::from("You can breathe here don't worry :)")).title(String::from("Message")));
                            game.set_message(z);
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Grass")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Green)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sand")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Yellow)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Rocks")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Gray)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Cinderblock")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::LightRed)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Flower")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Magenta)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Water")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::LightBlue)))));
                            self.water_count+=1;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Barrier")
                        {
                            self.y=self.y-1;
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::White)))));
                            checky+=1;
                            break;
                        }
                        checky+=1;
                        break;
                    }

                }
                /// if no match then set the screen as empty
                if checky==0
                {game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟')));}

                if self.water_count==10
                {
                    let z: Option<Message> = Some(Message::new(String::from("You Drowned :(")).title(String::from("Message")));
                    game.set_message(z);
                }
            },

            /// when the 'up' arrow is pressed, the player moves in the up direction
            /// all the postion hurdles will be checked at each iteration
            SimpleEvent::Just(KeyCode::Left) => {
                /// check if player has travelled 10 blocks of water 
                /// if yes the game will end instatnly
                if self.water_count == 10
                {
                    game.end_game();
                }
                let mut checky=0;
                let vec1_clone = self.vec1.clone();
                /// set the default color background as soon as the player changes or moves from its postion
                for ele in vec1_clone
                {
                    if ele.0 == self.x && ele.1 == self.y
                    {
                        if ele.2.contains("Obj")
                        {
                            // let ahm2: Vec<char> = ele.3.chars().collect();
                            // game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(ahm2[0]).style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sign")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('📨').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            game.set_message(None);
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Grass")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Green)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sand")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Yellow)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Rocks")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Gray)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Cinderblock")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightRed)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Flower")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Magenta)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Water")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightBlue)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Barrier")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::White)))));
                            checky+=1;
                            break;
                        }
                        else {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ')));
                            checky+=1;
                            break;
                        }
                    }   
                }

                if checky==0
                {game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ')));}

                
                self.x=self.x-1;
                let coor=game.get_viewport();
                if coor.x>self.x
                {
                    
                    // println!("{}, {}he", coor.x, coor.y);
                    game.set_viewport(ViewportLocation { x: (coor.x-1), y: (coor.y) });
                }

                let mut checky=0;
                let vec1_clone = self.vec1.clone();
                /// setting the default color and player as soon as the player goes to the postion according to the key

                for ele in vec1_clone
                {
                    if ele.0 == self.x && ele.1 == self.y
                    {
                        if ele.2.contains("Obj")
                        {
                            let ahm2: Vec<char> = ele.3.chars().collect();
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sign")
                        {
                            game.set_screen_char(self.x, self.y , Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            let z: Option<Message> = Some(Message::new(String::from("You can breathe here don't worry :)")).title(String::from("Message")));
                            game.set_message(z);
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Grass")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Green)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sand")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Yellow)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Rocks")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Gray)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Cinderblock")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::LightRed)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Flower")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Magenta)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Water")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::LightBlue)))));
                            self.water_count+=1;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Barrier")
                        {
                            self.x=self.x+1;
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::White)))));
                            checky+=1;
                            break;
                        }
                        checky+=1;
                        break;
                    }

                }
                /// if no match then set the screen as empty
                if checky==0
                {game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟')));}
                // game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟')));
                // println!("{}", char_loc1.x);
                if self.water_count==10
                {
                    let z: Option<Message> = Some(Message::new(String::from("You Drowned :(")).title(String::from("Message")));
                    game.set_message(z);
                }
            },

            /// when the 'up' arrow is pressed, the player moves in the up direction
            /// all the postion hurdles will be checked at each iteration
            SimpleEvent::Just(KeyCode::Right) => {
                /// check if player has travelled 10 blocks of water 
                /// if yes the game will end instatnly

                if self.water_count == 10
                {
                    game.end_game();
                }
                let mut checky=0;
                let vec1_clone = self.vec1.clone();
                /// set the default color background as soon as the player changes or moves from its postion
                for ele in vec1_clone
                {
                    if ele.0 == self.x && ele.1 == self.y
                    {
                        if ele.2.contains("Obj")
                        {
                            // let ahm2: Vec<char> = ele.3.chars().collect();
                            // game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(ahm2[0]).style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sign")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('📨').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            game.set_message(None);
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Grass")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Green)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sand")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Yellow)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Rocks")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Gray)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Cinderblock")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightRed)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Flower")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Magenta)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Water")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::LightBlue)))));
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Barrier")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::White)))));
                            checky+=1;
                            break;
                        }
                        else {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ')));
                            checky+=1;
                            break;
                        }
                    }   
                }

                if checky==0
                {game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ')));}

                
                self.x=self.x+1;
                let coor=game.get_viewport();
                if self.x-coor.x>76
                {
                    
                    // println!("{}, {}he", coor.x, coor.y);
                    game.set_viewport(ViewportLocation { x: (coor.x+1), y: (coor.y) });
                }

                let mut checky=0;
                let vec1_clone = self.vec1.clone();
                /// setting the default color and player as soon as the player goes to the postion according to the key
                for ele in vec1_clone
                {
                    if ele.0 == self.x && ele.1 == self.y
                    {
                        if ele.2.contains("Obj")
                        {
                            let ahm2: Vec<char> = ele.3.chars().collect();
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sign")
                        {
                            game.set_screen_char(self.x, self.y , Some(StyledCharacter::new(' ').style(GameStyle::new().background_color(Some(GameColor::Black)))));
                            let z: Option<Message> = Some(Message::new(String::from("You can breathe here don't worry :)")).title(String::from("Message")));
                            game.set_message(z);
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Grass")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Green)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Sand")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Yellow)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Rocks")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Gray)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Cinderblock")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::LightRed)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Flower")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::Magenta)))));
                            self.water_count=0;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Water")
                        {
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::LightBlue)))));
                            self.water_count+=1;
                            checky+=1;
                            break;
                        }
                        else if ele.2.contains("Barrier")
                        {
                            self.x=self.x-1;
                            game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟').style(GameStyle::new().background_color(Some(GameColor::White)))));
                            checky+=1;
                            break;
                        }
                        checky+=1;
                        break;
                    }

                }
                /// if no match then set the screen as empty
                if checky==0
                {game.set_screen_char(self.x, self.y, Some(StyledCharacter::new('♟')));}
                
                if self.water_count==10
                {
                    let z: Option<Message> = Some(Message::new(String::from("You Drowned :(")).title(String::from("Message")));
                    game.set_message(z);
                }
            },
            _ => {}
        }

    }
    fn on_tick(&mut self, _game: &mut Game) {
        
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    /// setting the struct to default values
    /// x coordinate to 3
    /// y coordinate to 3
    /// water count to 0 
    let mut controller = MyGame {
        x: 3,
        y: 3,
        vec1: Vec::new(),
        water_count: 0,
    };

    /// run the game with the help of a controller
    /// exit the game with ctrl + key 'c' which basically triggers a simple event
    /// with a duration of 50 milliseconds
    run_game(
        &mut controller,
        GameSettings::new()
            // The below are the defaults, but shown so you can edit them.
            .tick_duration(Duration::from_millis(50))
            .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into()))

    )?;

    /// As soon as the game ends the message gets displayed in the terminal
    /// inidcating that the game has ended
    println!("Game Ended!");

    Ok(())
}
