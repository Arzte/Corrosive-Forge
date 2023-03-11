/// This is intended to phrase through the `project.json` of all the (pretty much) useless information.
/// Ideally that can be used to make a new CYOA viewer, but at the very least it'll give me some ideas
/// on what to do with the ``project.json`` reduced down without the extra noise.
mod project;

use colored::Colorize;
use project::Root;

use std::{fs::File, io::prelude::*};

use crate::project::Required;

fn main() -> std::io::Result<()> {
    // Open `project.json` into a buffer
    let mut file = File::open("project.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // Feed the buffer into serde
    let v: Root = serde_json::from_str(&contents)?;

    // Write a new file without the style information.
    let pretty = serde_json::to_string(&v)?;
    println!("Writing to project.json to project_new.json without styling");
    std::fs::write("project_new.json", pretty)?;
    println!("Successfully wrote to project_new.json");

    //let pretty_no_style: Root = serde_json::from_str(&pretty)?;
    //println!("{:?}", pretty_no_style.styling);

    // Print the title of every row in the CYOA
    //v.rows.iter().for_each(|row_item| {
    //    let required = {
    //        if row_item.objects.is_empty() || row_item.objects[0].addons.is_empty() {
    //            vec![Required::default()]
    //        } else {
    //            // TODO: don't clone the requireds vector
    //            row_item.objects[0].addons[0].requireds.clone()
    //        }
    //    };

    //    // Only print if there's actually items to be found
    //    if !required.is_empty() {
    //        println!("{}: \n\t{:?}", row_item.title.green(), required);
    //    }
    //});
    Ok(())
}
