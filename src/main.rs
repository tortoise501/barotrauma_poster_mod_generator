//TODO: args to set mod name



use std::{fs, io::{Error, ErrorKind, Write}, path::PathBuf};
use image::GenericImageView;

fn main() {


    println!("enter your mod's name or press Enter to use default name \"my poster mod\"");
    let mod_name = match  get_correct_input() {
        Some(name) => name,
        None => "my poster mod".to_string(),
    };

    println!("Name : {mod_name}");

    println!("enter your texture folder name or press Enter to use default name \"posters\"");
    let folder_name = match get_correct_input() {
        Some(name) => name,
        None => "posters".to_string(),
    };

    println!("Folder : {folder_name}");

    println!("enter price for every poster or press Enter to use default price \"35\"");
    let mut poster_price;
    loop {
        let price = get_correct_number_input();
        if let Ok(price) = price {
            poster_price = match price {
                Some(s) => s,
                None => 35,
            };
            break;
        }
        println!("incorrect input, please enter positive number");
        
    }

    println!("Price : {poster_price}");

    println!("enter Y if you want to put file name as item name, puts numbers if not chosen");
    let put_filename = get_correct_bool_input();

    println!("Filename : {put_filename}");


    let img_list = ImgList::create_from_directory(folder_name.as_str());
    create_file_list(&mod_name);
    create_mod_file(img_list,&mod_name,poster_price,put_filename);
}


fn get_input() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn get_correct_number_input() -> std::io::Result<Option<u32>> {
    let input = get_correct_input();
    if let None = input {
        return Ok(None)
    }
    let input = input.unwrap();
    let input = input.parse::<u32>();
    match input {
        Ok(s) => Ok(Some(s)),
        Err(e) => Err(std::io::Error::new(ErrorKind::InvalidInput, e)),
    }
}

fn get_correct_input() -> Option<String> {
    let input = get_input();
    if input.is_empty(){
        return None;
    }
    Some(input)
}

fn get_correct_bool_input() -> bool{
    let input = get_correct_input();
    if let None = input {
        return false
    }
    let input = input.unwrap();
    if input.to_lowercase().trim() == "y"{
        return true
    }
    else {
        return false
    }

}



fn create_file_list(mod_name:&String) {
    let mut contents = String::new();
    contents.push_str(
        format!(
r#"<?xml version="1.0" encoding="utf-8"?>
<contentpackage name="{}" corepackage="False">
  <Item file="%ModDir%/{}items.xml" />
</contentpackage>"#,mod_name,mod_name.to_lowercase().replace(' ', "_")
        ).as_str()
);
    let mut file = fs::File::create("filelist.xml").unwrap();
    _ = file.write(contents.as_bytes());
}

fn create_mod_file(img_list:ImgList,mod_name:&String, poster_price:u32, put_filename:bool) {
    let mut contents = String::new();
    contents.push_str(
        r#"<?xml version="1.0" encoding="utf-8"?>
<Items>"#
    );


    let mut poster_count = 0;
    for img in img_list.imgs {
        poster_count += 1;

        let poster_name = if put_filename {
            let filename = img.file_name().unwrap().to_str().unwrap().to_string().split(".").take(1).collect::<String>();
            filename

        }else{
            format!("{mod_name} poster no. {poster_count}")
        };

        let poster_description = if put_filename {
            format!("{poster_name} poster from {mod_name} mod")

        }else{
            format!("Poster from {mod_name} mod number {poster_count}")
        };
        



        contents.push_str("\n");
        let path = format!("{}",img.display()).replace('\\', "/");
        let img = image::open(img).expect("KYS"); //TODO: Kys
        let (width, height) = img.dimensions();
        contents.push_str(
            format!(
                r#"  <Item name="{poster_name}" identifier="{mod_name} - {poster_count}" description="{poster_description}" category="Decorative" scale="0.5" maxstacksize="4" pickdistance="200" tags="mediumitem" isshootable="True" Indestructible="True" AllowRotatingInEditor="True" CanFlipX="False" CanFlipY="False" >
    <Body width="{width}" height="{height}" density="10"/>
    <Sprite texture="%ModDir%/{path}" depth="0.845" sourcerect="00,00,{width},{height}" origin="0.5,0.5"/>
    <Holdable selectkey="Select" pickkey="Use" slots="Any,RightHand,LeftHand" msg="ItemMsgDetach" PickingTime="1" aimpos="0,0" handle1="0,0" Attachable="True" AttachedByDefault="True" Aimable="True"/>
    <Price baseprice="{poster_price}" soldeverywhere="false">
      <Price storeidentifier="merchantoutpost" maxavailable="1" />
      <Price storeidentifier="merchantcity" minavailable="1" maxavailable="2" multiplier="0.9"/>
    </Price>
    <Fabricate suitablefabricators="medicalfabricator" requiredtime="5" amount="1">
      <RequiredItem identifier="organicfiber" />
    </Fabricate>
    <Deconstruct time="1">
      <Item identifier="organicfiber" />
    </Deconstruct>
    <Fabricate suitablefabricators="vendingmachine" requiredtime="1" requiredmoney="65" fabricationlimitmin="1" fabricationlimitmax="1" />
    <PreferredContainer primary="crewcab" />
    <PreferredContainer secondary="wreckcrewcab,abandonedcrewcab,abandonedstoragecab,steelcabinetwrecked,mediumsteelcabinetwrecked" maxamount="1" spawnprobability="0.003" allowtransfershere="False" />
  </Item>"#,
            ).as_str()
        )
    }
    contents.push_str("\n</Items>");

    let mut file = fs::File::create(format!("{mod_name}items.xml")).unwrap();
    _ = file.write(contents.as_bytes());
}



struct ImgList {
    imgs: Vec<PathBuf>
}
impl ImgList {
    fn create_from_directory(path_to_dir:&str) -> ImgList {
        // let files = fs::read_dir(path_to_dir).unwrap();  
        let files = match fs::read_dir(path_to_dir) {
            Ok(read) => {
                read
            },
            Err(_) => {
                let _ = fs::create_dir(path_to_dir);
                fs::read_dir(path_to_dir).unwrap()
            },
        };
        let mut imgs: Vec<PathBuf> = Vec::new();
        for path in files {
            // println!("Name: {}", path.unwrap().path().display());
            imgs.push(path.unwrap().path());
        }
        ImgList {
            imgs
        }
    }
}

// const modname: &str = "testmod12344321"; 