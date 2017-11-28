extern crate rand;

use self::rand::Rng;


pub fn solve(matrix: &mut Vec<Vec<i32>>, iterations: i32, start: i32) {

    // Ilość istotnych zdarzeń
    let mut critical_events: i32 = 0;
    // Aktualna ściezka
    let mut current_path: Vec<i32> = Vec::new();
    // Koszt aktualnej ściezki
    let mut current_path_value: i32 = 0;
    // Lista tabu
    let mut tabu_list: Vec<Vec<i32>> = Vec::new();

    // Wypełnij wektor okreslający ściezkę kolejnymi wierzchokami
    for i in 0..(matrix.len() as i32) {
        current_path.push(i);
    }

    // Losowa zmiana kolejności w wektorze
    rand::thread_rng().shuffle(&mut current_path);

    // Sprawdzenie kosztu aktualnej drogi
    current_path_value = get_current_path_value(&matrix, &mut current_path);

    // Wygeneorwanie pustej listy tabu
    for i in 0..(matrix.len() as i32) {
        let mut tabu_list_row: Vec<i32> =
            current_path.push(i);
    }
}

fn get_current_path_value(matrix: &Vec<Vec<i32>>, path: &mut Vec<i32>) -> i32 {

    // Początkowy koszt ścieżki to 0
    let mut value: i32 = 0;
    // Pierwszy wierzchołek
    let mut previous_node: usize = 0;

    // Iteracja po wszystkich kolejnych wierzchołkach
    for i in 0..(path.len() as i32) {
        // Zwiększenie kosztu
        value = value + matrix[previous_node][(path[(i as usize)] as usize)];
        // Przypisanie aktualnego wierzchołka jako poprzedniego
        previous_node = path[(i as usize)] as usize;
    }

    // Zwiększenie kosztu trasy o koszt powrotu do wierzchołka początkowego
    value = value + matrix[previous_node][0];

    return value;
}