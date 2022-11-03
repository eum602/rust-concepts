use error_example::Dolphin;
use anyhow::Result;

 fn play_time(dolphin: &Dolphin) -> Result<Vec<String>> {
     let mut responses = vec![];
     responses.push(dolphin.say_your_name()?);
     responses.push(dolphin.flip()?);
     responses.push(dolphin.shake_hands()?);

     Ok(responses)
    }

fn main() -> Result<()> {
    let dolphins = vec![
        Dolphin {
            name: "Augustinius".into(),
            age: 7,
            hungry: false,
        },
        Dolphin {
            name: "Bitty".into(),
            age: 2,
            hungry: true,
        },
        Dolphin {
            name: "Carson".into(),
            age: 5,
            hungry: true,
        },
        Dolphin {
            name: "Devin".into(),
            age: 6,
            hungry: false,
        },
    ];
    for dolphin in &dolphins {
        println!("{} did a FABULOUS PERFORMANCE!", dolphin.name);
        let responses = play_time(dolphin)?;
        for response in responses {
            println!("{}", response);
        }
        /*match play_time(dolphin) {
            Ok(responses) => {
                println!("{} did a FABULOUS PERFORMANCE!", dolphin.name);
                for response in responses {
                    println!("  {}", response);
                }
            }
            Err(e) => println!("{} can't perform today: {}", dolphin.name, e.to_string()),
        }*/
    }
    Ok(())
}