use csv::{ ReaderBuilder, StringRecord };
use std::{ fs };
use std::collections::{ HashMap };

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
struct DataHistory {
    data_type: String,
    tag: String,
    text: String,
    life: i32,
    options: Vec<DataHistory>
}

impl DataHistory {
    fn new(row: StringRecord)->DataHistory{

        let life = row.get(3).unwrap().trim();
        let life = life.parse::<i32>().unwrap_or(0);

        return DataHistory {
            data_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life,
            options: vec![],
        };
    }
}

fn main() {

    let mut life = 100;
    let mut current_tag = FIRST_TAG;
    let mut last_record: String = String::new();

    let mut data_history: HashMap<String, DataHistory> = HashMap::new();
    
    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        
        let data = DataHistory::new(result.unwrap());

        if data.data_type == "SITUACION" {
            
            last_record = data.tag.clone();

            data_history.insert(last_record.clone(), data);

        } else if data.data_type == "OPCION" {

            if let Some(data_memory) = data_history.get_mut(&last_record){
            
                (*data_memory).options.push(data);

            }
        }
    }
    
    
    loop {
        
        println!("You have {life} life");

        if let Some(data) = data_history.get(current_tag){

            println!("{}", data.text);

            for (index, option) in data.options.iter().enumerate() {
                
                println!("[{index}] {}", option.text)

            }

            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).unwrap();

            let selection = selection.trim().parse().unwrap_or(99);

            if let Some(_select) = &data.options.get(selection) {

                current_tag = &_select.tag;

            } else {

                println!("Command not found :(");

            }

            life += data.life;

        } else {

            break;

        }

        if life <= 0 {
            
            break println!("You're loser");
        
        }
    }

}
