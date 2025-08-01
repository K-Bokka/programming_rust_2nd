use std::sync::{Arc, Mutex};

fn main() {
    println!("Fern empire");

    #[allow(unused_variables)]
    let app = Arc::new(FernEmpireApp {
        waiting_list: Mutex::new(vec![]),
    });
}

type PlayerId = u32;

const GAME_SIZE: usize = 8;

type WaitingList = Vec<PlayerId>;

struct FernEmpireApp {
    waiting_list: Mutex<WaitingList>,
}

#[allow(dead_code)]
impl FernEmpireApp {
    fn join_waiting_list(&self, player: PlayerId) {
        let mut guard = self.waiting_list.lock().unwrap();

        guard.push(player);
        if guard.len() == GAME_SIZE {
            #[allow(unused_variables)]
            let players = guard.split_off(0);
            // self.start_game(player);
        }
    }
}
