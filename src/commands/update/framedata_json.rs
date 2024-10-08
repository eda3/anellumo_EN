use crate::{MoveInfo, CHARS};
use regex::Regex;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
extern crate ureq;

#[derive(Deserialize, Debug)]
struct Response {
    cargoquery: Vec<Data>,
}

#[derive(Deserialize, Debug)]
struct Data {
    title: Title,
}

#[derive(Deserialize, Debug)]
struct Title {
    input: Option<String>,
    name: Option<String>,
    damage: Option<String>,
    guard: Option<String>,
    invuln: Option<String>,
    startup: Option<String>,
    active: Option<String>,
    recovery: Option<String>,
    #[serde(rename = "onHit")]
    hit: Option<String>,
    #[serde(rename = "onBlock")]
    block: Option<String>,
    // level: Option<String>,
    // counter: Option<String>,
}

pub async fn frames_to_json(
    mut char_page_response_json: String,
    mut file: &File,
    char_count: usize,
) {
    char_page_response_json = char_page_response_json.replace(r#" &gt; "#, "");
    char_page_response_json = char_page_response_json.replace(r#"&lt;br&gt;"#, ", ");
    char_page_response_json = char_page_response_json.replace(r#"&lt;br/&gt;"#, ", ");
    // Ino low profile
    char_page_response_json = char_page_response_json.replace(r#" &lt;span class=&quot;tooltip&quot; &gt;Low Profile&lt;span class=&quot;tooltiptext&quot; style=&quot;&quot;&gt;When a character's hurtbox is entirely beneath an opponent's attack. This can be caused by crouching, certain moves, and being short.&lt;/span&gt;&lt;/span&gt;"#, "");
    char_page_response_json = char_page_response_json.replace(r#"&#039;&#039;&#039;"#, "");
    char_page_response_json =
        char_page_response_json.replace(r#"&#039;&#039;&#039;&lt;/span&gt;"#, "");

    char_page_response_json = char_page_response_json.replace(r#"Tomato"#, "");
    char_page_response_json = char_page_response_json.replace(r#"#00d7c"#, "");
    char_page_response_json = char_page_response_json.replace(r#"#b70c0b"#, "");
    
    
    char_page_response_json =
        char_page_response_json.replace(r#"&lt;span style=&quot;color: \n#3455ff&quot; &gt;"#, "");
    char_page_response_json =
        char_page_response_json.replace(r#"&lt;span style=&quot;color: \n#bcbd0c&quot; &gt"#, "");
    char_page_response_json = char_page_response_json.replace(r#"&lt;/span&gt;"#, "");
    char_page_response_json = char_page_response_json.replace(r#"&lt;span style=&quot;"#, "");
    char_page_response_json = char_page_response_json.replace(r#"span style=&quot;"#, "");
    char_page_response_json = char_page_response_json.replace(r#"color: "#, "");
    char_page_response_json = char_page_response_json.replace(r#"0&quot; &gt;"#, "");
    char_page_response_json = char_page_response_json.replace(r#"&quot; &gt;"#, "");

    char_page_response_json = char_page_response_json.replace(r#"j."#, "j");
    char_page_response_json = char_page_response_json.replace(r#""c."#, r#""c"#);
    char_page_response_json = char_page_response_json.replace(r#""f."#, r#""f"#);
    char_page_response_json = char_page_response_json.replace(r#"Ultimate"#, "U");
    char_page_response_json = char_page_response_json.replace(r#"Ulitmate"#, "U");
    char_page_response_json = char_page_response_json.replace(r#"L+U"#, r#"LU"#);
    let mut moves_info: Response = serde_json::from_str(&char_page_response_json).unwrap();

    for x in 0..moves_info.cargoquery.len() {
        // Replacing None values with a generic '-'
        if moves_info.cargoquery[x].title.input.is_none() {
            moves_info.cargoquery[x].title.input = Some("-".to_string());
        } else {
            // Skips finish blow for sol
            if *moves_info.cargoquery[x].title.input.as_ref().unwrap() == "j.XX during Homing Jump"
            {
                continue;
            }
        }
        if moves_info.cargoquery[x].title.name.is_none() {
            moves_info.cargoquery[x].title.name = Some(
                moves_info.cargoquery[x]
                    .title
                    .input
                    .as_ref()
                    .unwrap()
                    .to_string(),
            );
        } else {
            // Skips dash cancel entry, ino hoverdash chipp escape zato flight and finish blow
            if *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Dash Cancel"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Hoverdash"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Finish Blow"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Flight"
                || *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Escape"
            {
                continue;
            }
        }
        if moves_info.cargoquery[x].title.damage.is_none() {
            moves_info.cargoquery[x].title.damage = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.guard.is_none() {
            moves_info.cargoquery[x].title.guard = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.invuln.is_none() {
            moves_info.cargoquery[x].title.invuln = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.startup.is_none() {
            moves_info.cargoquery[x].title.startup = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.active.is_none() {
            moves_info.cargoquery[x].title.active = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.recovery.is_none() {
            moves_info.cargoquery[x].title.recovery = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.hit.is_none() {
            moves_info.cargoquery[x].title.hit = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.hit.is_none() {
            moves_info.cargoquery[x].title.hit = Some("-".to_string());
        }
        if let Some(input) = &mut moves_info.cargoquery[x].title.input {
            *input = input.replace("~", "");
            // *input = input.replace(" ", "");
        }
        if let Some(hit) = &mut moves_info.cargoquery[x].title.hit {
            *hit = hit.replace("&lt;span style=&quot;color: \n#b70c0b&quot; &gt;", "");
            *hit = hit.replace("&lt;span style=&quot;color: \n#00d7c0&quot; &gt;'''", "");
            *hit = hit.replace("'''&lt;/span&gt;", "");
            *hit = hit.replace("'''", "");
            *hit = hit.replace("&lt;span style=&quot;color: Tomato&quot; &gt;", "");
        }
        if moves_info.cargoquery[x].title.block.is_none() {
            moves_info.cargoquery[x].title.block = Some("-".to_string());
        }
        if let Some(block) = &mut moves_info.cargoquery[x].title.block {
            *block = block.replace("&lt;span style=&quot;color: \n#b70c0b&quot; &gt;", "");
            *block = block.replace("&lt;span style=&quot;color: \n#00d7c0&quot; &gt;'''", "");
            *block = block.replace("'''&lt;/span&gt;", "");
            *block = block.replace("'''", "");
            *block = block.replace("&lt;span style=&quot;color: Tomato&quot; &gt;", "");
        }
        // if moves_info.cargoquery[x].title.level.is_none(){
        //   moves_info.cargoquery[x].title.level = Some("-".to_string());
        //}
        //        if moves_info.cargoquery[x].title.riscgain.is_none(){
        //            moves_info.cargoquery[x].title.riscgain = Some("-".to_string());
        //        }
        //        if moves_info.cargoquery[x].title.prorate.is_none(){
        //            moves_info.cargoquery[x].title.prorate = Some("-".to_string());
        //        }
        //        if moves_info.cargoquery[x].title.counter.is_none(){
        //            moves_info.cargoquery[x].title.counter = Some("-".to_string());
        //        }

        println!("");
        println!(
            "{}",
            moves_info.cargoquery[x]
                .title
                .input
                .as_ref()
                .unwrap()
                .to_string()
        );
        println!(
            "{}",
            moves_info.cargoquery[x]
                .title
                .hit
                .as_ref()
                .unwrap()
                .to_string()
        );
        // Serializing frame data
        let processed_moves_info = serde_json::to_string(&MoveInfo {
            input: moves_info.cargoquery[x]
                .title
                .input
                .as_ref()
                .unwrap()
                .to_string(),
            name: moves_info.cargoquery[x]
                .title
                .name
                .as_ref()
                .unwrap()
                .to_string(),
            damage: moves_info.cargoquery[x]
                .title
                .damage
                .as_ref()
                .unwrap()
                .to_string(),
            guard: moves_info.cargoquery[x]
                .title
                .guard
                .as_ref()
                .unwrap()
                .to_string(),
            invincibility: moves_info.cargoquery[x]
                .title
                .invuln
                .as_ref()
                .unwrap()
                .to_string(),
            startup: moves_info.cargoquery[x]
                .title
                .startup
                .as_ref()
                .unwrap()
                .to_string(),
            active: moves_info.cargoquery[x]
                .title
                .active
                .as_ref()
                .unwrap()
                .to_string(),
            recovery: moves_info.cargoquery[x]
                .title
                .recovery
                .as_ref()
                .unwrap()
                .to_string(),
            hit: moves_info.cargoquery[x]
                .title
                .hit
                .as_ref()
                .unwrap()
                .to_string(),
            block: moves_info.cargoquery[x]
                .title
                .block
                .as_ref()
                .unwrap()
                .to_string(),
            // level: moves_info.cargoquery[x].title.level.as_ref().unwrap().to_string(),
            // riscgain: moves_info.cargoquery[x].title.riscgain.as_ref().unwrap().to_string(),
            // scaling: moves_info.cargoquery[x].title.prorate.as_ref().unwrap().to_string(),
            // counter: moves_info.cargoquery[x].title.counter.as_ref().unwrap().to_string(),
        })
        .unwrap();

        write!(file, "{}", processed_moves_info)
            .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count] + ".json'."));

        // Skip writting comma/tab if next and last iteration
        // contains 'finish blow' in last the input field
        if x == moves_info.cargoquery.len() - 2
            && *moves_info.cargoquery[x + 1].title.input.as_ref().unwrap()
                == "j.XX during Homing Jump"
        {
            continue;
        } else if x != moves_info.cargoquery.len() - 1 {
            // Adding comma/tab
            // file.write(b",\n\t")
            //     .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS[char_count]+ ".json'."));
            (&mut file).write_all(b",\n\t").expect(
                &("\nFailed to write ',\\n\\t' while serializing '".to_owned()
                    + CHARS[char_count]
                    + ".json'."),
            );
        }
    }
}
