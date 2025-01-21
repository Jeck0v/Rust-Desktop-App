use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("Que souhaitez-vous faire ?");
        println!("1. Ajouter une tâche");
        println!("2. Afficher les tâches");
        println!("3. Quitter");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Erreur de lecture");

        match choice.trim() {
            "1" => {
                println!("Entrez la tâche à ajouter : ");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Erreur de lecture");
                tasks.push(task.trim().to_string());
                println!("Tâche ajoutée !");
            }
            "2" => {
                println!("Vos tâches :");
                for (index, task) in tasks.iter().enumerate() {
                    println!("{}: {}", index + 1, task);
                }
            }
            "3" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide, réessayez."),
        }
    }
}
