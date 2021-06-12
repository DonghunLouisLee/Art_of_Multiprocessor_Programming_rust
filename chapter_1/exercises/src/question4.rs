use rand::Rng;

fn main() {
    //core of this program is to designate a prisoner("leader") that will count how many prisoners have visited the cell
    //for simplicity, we will designate prisoner 1 as the leader
    let mut count = 0;
    let mut switch = true;
    let number_of_prisoners = 100;
    let mut rng = rand::thread_rng();
    while count < number_of_prisoners {
        //first prisoner is chosen
        let ran = rng.gen_range(1..number_of_prisoners + 1);
        println!("prisoner_{} is selected", ran);
        //if chosen prisoner is the leader,
        if ran == 1 {
            if switch == true {
                count += 1;
            }
            switch = false;
        } else {
            if switch == false {
                switch = true;
            }
        }
    }

    println!("this is the final count: {}", count);
}
