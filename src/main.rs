use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};


const MIME_FILES: [&str; 2] = ["mime.types", "mime-support/mime.types"];


fn main()-> Result<(), Box<dyn Error>> {

    let mut all_types = HashMap::new();
    for filename in MIME_FILES {
        let map = parse_mime_file(filename)?;
        all_types.extend(map);
    }

    find_duplicates(&all_types)?;

    let mut types: Vec<_> = all_types.keys().cloned().collect();
    types.sort_unstable();

    
    for ty in types {
        // let exts = all_types.get(&ty).unwrap_or(&HashSet::new());
        if let Some(exts) = all_types.get(&ty) {
            println!("Type: {}, Extensions: {:?}", ty, exts);            
        }else {
            println!("Type: {}, Extensions: None", ty);
        }
    }

    Ok(())
}

fn find_duplicates(map: &HashMap<String, HashSet<String>>)-> Result<(), Box<dyn Error>> {
    let mut seen = HashMap::new();
    for (ty, exts) in map {
        for ext in exts {
            if let Some(prev) = seen.insert(ext, ty) { // ext.clone & ty.clone
                println!("Duplicate extension {} for types {} and {}", ext, prev, ty);
            }
        }
    }
     Ok(())
}

fn parse_mime_file(filename: &str) -> Result<HashMap<String, HashSet<String>>, Box<dyn Error>> {
 let file = File::open(filename)?;
 let reader = BufReader::new(file);
 
 let map = reader.lines()
     .filter_map(|ln| ln.ok())
     .filter(|ln| !ln.is_empty() && !ln.trim_start().starts_with('#'))
     .map(|ln| {
         let mut parts = ln.split_whitespace();
         let ty = parts.next().unwrap().to_string();
         let exts = parts.map(String::from).collect::<HashSet<_>>();
         (ty, exts)
     })
     .collect::<HashMap<_, _>>();

Ok(map)
}
