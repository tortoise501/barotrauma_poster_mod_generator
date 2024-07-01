//TODO: args to set mod name



use std::{env,fs, io::Write, path::PathBuf};
use image::GenericImageView;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = if args.len() >=2{
        args[1].clone().to_lowercase().replace(' ', "_")
    }else{"my_poster_mod".to_string()};
    let poster_dir_name = if args.len() >= 3{
        args[2].clone()
    }else{"posters".to_string()};
    let img_list = ImgList::create_from_directory(poster_dir_name.as_str());
    create_file_list(&name);
    create_mod_file(img_list,&name);
}

fn create_file_list(modname:&String) {
    let mut contents = String::new();
    contents.push_str(
        format!(
r#"<?xml version="1.0" encoding="utf-8"?>
<contentpackage name="{}" corepackage="False">
  <Item file="%ModDir%/{}items.xml" />
</contentpackage>"#,modname,modname
        ).as_str()
);
    let mut file = fs::File::create("filelist.xml").unwrap();
    _ = file.write(contents.as_bytes());
}

fn create_mod_file(img_list:ImgList,modname:&String) {
    let mut contents = String::new();
    contents.push_str(
        r#"<?xml version="1.0" encoding="utf-8"?>
<Items>"#
    );
    let mut poster_count = 0;
    for img in img_list.imgs {
        poster_count += 1;
        contents.push_str("\n");
        let path = format!("{}",img.display()).replace('\\', "/");
        let img = image::open(img).expect("KYS"); //TODO: Kys
        let (width, height) = img.dimensions();
        contents.push_str(
            format!(
                r#"  <Item name="{modname} poster no. {poster_count}" identifier="{modname}{poster_count}" description="Poster from {modname} mod number {poster_count}" category="Decorative" scale="0.5" maxstacksize="4" pickdistance="200" tags="mediumitem" isshootable="True" Indestructible="True" AllowRotatingInEditor="True" CanFlipX="False" CanFlipY="False" >
    <Body width="{width}" height="{height}" density="10"/>
    <Sprite texture="%ModDir%/{path}" depth="0.845" sourcerect="00,00,{width},{height}" origin="0.5,0.5"/>
    <Holdable selectkey="Select" pickkey="Use" slots="Any,RightHand,LeftHand" msg="ItemMsgDetach" PickingTime="1" aimpos="0,0" handle1="0,0" Attachable="True" AttachedByDefault="True" Aimable="True"/>
    <Price baseprice="15" soldeverywhere="false">
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

    let mut file = fs::File::create(format!("{modname}items.xml")).unwrap();
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