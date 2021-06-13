#  RUST 로 공부하는 Art of Multiprocessor Programming

저는 심심할때마다 Crust of Rust 강의를 챙겨보는 편입니다. Rust 관련 강의가 많의 없는 현실에서 Rust의 기초를 알고 난 후 그 이상을 배우기 위해서는 이만한 강의가 없다고 생각합니다. 다만 가장 최근에 올라온 강의였던 Lock-Free to Wait-free Simulation in Rust 를 듣다가 이 강의를 제가 잘 따라가지 못한다는 느낌을 받았습니다. 곰곰히 생각해보니 그 이유는 지금까지 "A in Rust"라는 강의에서 A라는 개념에 대해서 제가 예전에 공부해본적이 있던 주제들이었는데 이번 주제인 Lock-free to Wait-free는 제가 깊게 공부해보지 않았던 분야여서 주제 자체에 대한 이해가 많이 부족해서 그랬던 것이었습니다. 그래서 마침 병렬 프로그래링도 제대로 다시 공부해볼겸 Art of Multiprocessor Programming 책을 공부해보기로 했습니다. 현재 계획은 한주에 최소 1 챕터를 공부하기여서 아마 최대 18주 정도 걸리지 않을까 생각이 듭니다. 물론 열심히 해서 그전에 끝낼수 있도록 해야죠 ㅎ

Art of Multiprocessor Programming 은 모든 code를 자바 혹은 C로 구성하고 있습니다. 그래서 챕터별 연습문제 및 중간 예시 코드를 Rust로 구성해보면서 Rust 공부도 같이 해보려고 합니다. 시리즈의 순서는 챕터 순이될것이고 챕터 내용 간단 정리 및 연습문제 풀어보기로 구성이 될것 같습니다.  시리즈에서 모든 코드를 올리기보다는 code snippet위주로 가는게 가독성에 도움이 될것 같아서 코드 자체는 https://github.com/DonghunLouisLee/Art_of_Multiprocessor_Programming_rust 에 올려 놓도록 하겠습니다~

현재까지의 시리즈는

1. Introduction


입니다. 이 중 오늘은 Introduction입니다. 재미있게 읽어주세요~

## Theory

Multiprocessor Programming 은 이름 자체가 말해주듯 single thread 가 아닌 multi thread에서 programming을 하는것을 의미합니다. 현대에 들어오면서 하드웨어의 발전으로 웬만한 컴퓨터에는 여러 cpu 가 탑재되어 있고 프로그래밍을 할때 이 multi cpu 를 어떻게 효율적으로 사용하냐는 성능에 큰 영향을 미치게 됩니다. 이 책은 multiprocessor 환경에서 어떻게 하면 안전하고 빠르게 프로그래밍을 할수 있을지를 이론과 실습 부분으로 나누어서 자세히 설명을 합니다. 챕터2 부터 6까지는 이론을 다루고 7부터 18까지는 앞에서 배운 이론들을 바탕으로 multiprocessor programming에 사용되는 algorithm 및 data type들을 실제로 구현하는 일종의 실습을 다루고 있습니다. 이번 챕터에서는 본격적으로 이론 부분에 들어가기 전에 간단이 앞으로 사용하게 될 용어에 대한 정리를 하고 넘어가겠습니다. 

1. Shared Object and Synchronization

   => 여러 스레드가 공통적으로 사용되는 데이터를 나타냅니다. 이 스레드들은 같은 작업(같은 함수를 처리)을 할수도 있고 완전히 다른 작업을 처리할수도 있습니다. 하지만 중요한 점은 여러 스레드들을 각자 작업을 처리하는 도중 공통된 하나의 데이터에 대해 접근을 해야 할때가 있습니다. 이 경우 공통된 하나의 데이터를 shared object 라고 부르고 여러 스레드들이 이 shared object를 어떤 방식으로 접근할지에 대해 원칙(순서)을 정하는 것을 Synchronization 라고 부릅니다. 결국 Multiprocessor programming은 이 shared object를 어떻게 synchronize 하는것이 핵심이라고 할수 있습니다. synchronize를 구현할때 지켜야 하는 원칙은 다음과 같습니다. 

   1. Mutual Exclusion: 두개 이상의 스레드는 동시에 해당 shared object를 변경시키는 작업을 할수 없습니다. 즉, 한 스레드가 shared object를 변경하려고 할때 다른 스레드들은 shared object 에 접근을 하지 못하여야 합니다. 
   2. Deadlock-freedom: 두 개이상의 스레드가 해당 shared object에 접근하려고 할때 언젠가는(시간이 걸리더라도) 결국 하나의 스레드는 접근을 할수 있어야 합니다. 만약 스레드들이 모두 영원히 shared object 에 접근을 하지 못하는 상황이 발생하면 이를 deadlock 이라고 부릅니다.
   3. Starvation-freedom: 어떤 스레드든 shared object 에 접근을 하려고 할때 모든 스레드는 결국(시간이 걸리더라도) shared object에 접근을 할수 있어야 합니다. 만약 두 개의 스레드가 shared object에 접근하려고 했을 때 만약 하나의 스레드만 계속 접근하고 다른 하나는 접근을 못한다면 이는 starvation freedom 원칙에 위반됩니다. 
   4. Waiting: 두 개 이상의 스레드가 shared object에 접근을 하려고 했을때 하나의 스레드가 접근 권한을 반환하지 않고 작동을 멈추게 되면 다른 스레드는 무한정 대기를 하는 상황이 발생합니다. 이 상황을 다른 스레드들이 Waiting하고 있다고 표현합니다. 

   이 책에서 앞으로 여러 synchronization protocol를 다루게 되는데 해당 protocol이 위 원칙 중 어떤것들을 만족시키는지 파악을 할수 있어야 protocol 간의 차이점을 알수 있습니다. 

2. Amdahl's law

   => Multiprocessing Programming으로 해당 작업을 얼마나 더 빠르게 처리할수 있을지는 나타내는 수식입니다. 

   ![Amdahl's laws](https://static.packt-cdn.com/products/9781788993913/graphics/6424ba43-feda-4833-9343-18aba4fb01e3.png)

   * p 는 전체 작업 중 병렬적으로 처리가 가능한 작업의 비율입니다. 
   * N은 사용가능한 총 프로세서의 개수입니다.

   예를 한 번 들어보겠습니다. 만약 제 컴퓨터에 2개의 프로세서가 있고 전체 작업중 1/3이 병렬로 처리가 가능하면 위 수식에 의해 Speedu = 1/ ((1-1/3) + (1/3)/2) = 6/5가 됩니다. 즉, 2개의 프로세서를 사용한다면 1개의 프로세서를 사용한것보다 1.2배 빠르게 작업을 처리할수 있습니다. 



## Exercises

여러 문제가 있는데 이 중 코딩 관련된 부분에 대해 제가 Rust로 작성한 답안입니다. 

### Exercise1

![dijkstra's dining problem](https://upload.wikimedia.org/wikipedia/commons/thumb/7/7b/An_illustration_of_the_dining_philosophers_problem.png/220px-An_illustration_of_the_dining_philosophers_problem.png)

문제: 5명의 철학자들이 위와 같은 식탁에서 식사를 하려고 합니다.  식탁에는 5개의 음식이 담긴 접시가 있으며 접시마다 젓가락 하나가 있습니다. 다음과 같은 원칙들을 지키면서 철학자들이 모두 식사를 할수 있는 알고리즘을 작성하시오

1. 철학자들의 자리는 지정되어 있습니다. 
2. 철학자들은 식사를 할때 젓가락 한쌍(두 개)가 필요하고 자기 자리 좌우에 있는 젓가락만 사용 가능합니다. 
3. 철학자들은 각가 생각을 하다가 배가 고플때 젓가락을 사용해서 음식을 먹습니다. 한 입을 먹고서는 젓가락을 다시 제자리에 두고 다시 생각을 합니다. 

```rust
use std::sync::{Arc, Mutex};

use rand::Rng;

//philosopher 0 can only use chopstick 0,1
//philosopher 1 can only use chopstick 1,2
//philosopher 2 can only use chopstick 2,3
//philosopher 3 can only use chopstick 3,4
//philosopher 4 can only use chopstick 4,0
fn main() {
    println!("question1 answer is starting");
    let count = 5; //number of philosophers = number of chopsticks
    let mut chopsticks = vec![];
    for i in 0..count {
        println!("created {}th chopstick", i);
        let chopstick = Arc::new(Mutex::new(true)); //true me
        chopsticks.push(chopstick);
    }
    for i in 0..count {
        let left = i;
        let mut right = i + 1;
        if i == count - 1 {
            right = 0;
        }
        let left_lock = chopsticks.get_mut(left as usize).unwrap().clone();
        let right_lock = chopsticks.get_mut(right as usize).unwrap().clone();
        let _ = std::thread::spawn(move || {
            println!("created philosopher_{:?}", i);
            loop {
                println!("philosopher_{:?} is currently thinking", i);
                let mut rng = rand::thread_rng();
                let ran = rng.gen_range(0..3);
                std::thread::sleep(std::time::Duration::from_secs(ran)); //think
                println!("philosopher_{:?} now wants to eat something", i);
                loop {
                    //first check if philosopher can get both locks
                    let _left = left_lock.lock().unwrap();
                    let _right = right_lock.lock().unwrap();
                    //got both locks
                    //eat
                    println!("philosopher_{:?} is currently eating", i);
                    std::thread::sleep(std::time::Duration::from_secs(1)); //eat
                    break;
                }
                println!("philosopher_{:?} is done eating", i);
            }
        });
    }

    //run a loop so that this program doesn't end
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1)); //eat for 1 second
    }
}
```



### Exercise4

문제: 감옥에 있는 죄수 P명에게 교도관은 다음과 같은 문제를 내고 성공하면 모두 석방, 실패하면 사형의 조건을 걸었습니다. 죄수들이 석방될수 있는 알고리즘을 만드시오.

1. 오늘 죄수들은 모두 모여 전략을 논할수 있습니다. 그러나 내일부터는 모두 각방에 격리되어 소통이 불가능합니다. 
2. 교도관은 "스위치 방"을 하나 마련했고 이 방에는 on/off 만 가능한 스위치가 하나 있습니다. 
3. 교도관은 내일부터 랜덤으로 한명의 죄수를 골라 이 스위치 방에 넣을 것입니다. 죄수는 스위치 방에서 스위치를 키거나(on to off or off to on) 아무것도 안할수 있습니다. 
4. 모든 죄수들은 언젠가는 스위치 방을 최소 1번씩은 방문하게 됩니다. 
5. 어떤 죄수든지 만약 모든 죄수들이 스위치 방을 최소 1번씩 방문했다고 확신이 들면 교도관한테 자신의 생각을 전달할수 있습니다. 만약 이 주장이 참이면 전원 석방, 거짓이면 전원 사형입니다.

```rust
use rand::Rng;

fn main() {
    //core of this program is to designate a prisoner("leader") that will count how many prisoners have visited 			the cell
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
```

## 마치면서 

이번 편에서는 간단히 Introduction 내용들을 살펴보았습니다. Exercises문제는 개수가 조금 많아 모두 풀지는 못하였는데 혹시 풀이를 원하시는 문제가 있으시다면 댓글로 알려주시면 최대한 빠르게 해답을 작성해보겠습니다. 감사합니다~

## Reference 

http://cs.ipm.ac.ir/asoc2016/Resources/Theartofmulticore.pdf
