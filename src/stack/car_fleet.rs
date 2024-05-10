// use std::{borrow::Borrow, collections::BTreeMap};
use std::{cmp::max, collections::HashMap};
#[allow(dead_code)]
pub fn car_fleet(target: i32, mut position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut time: Vec<i32>;
    if position.contains(&target) {
        result += 1;
        let idx = position.iter().position(|x| *x == target).unwrap();

        position.remove(idx);
    }
    let mut cars: Vec<(i32, i32)> = Vec::new();
    for idx in 0..position.len() {
        cars.push((*position.get(idx).unwrap(), *speed.get(idx).unwrap()));
    }
    cars.sort();
    cars.reverse();
    loop {
        time = cars
            .clone()
            .into_iter()
            .map(|(pos, spd)| max((target - pos) / spd, 0))
            .collect();
        cars = cars
            .clone()
            .into_iter()
            .map(|(k, v)| (k + v, v))
            .collect::<Vec<_>>();
        // check if any cars collide
        let mut collide : HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..time.len(){
            if collide.get(&time[i]).is_some(){
                collide.get_mut(&time[i]).unwrap().push(i as i32);
                // collide.insert(time[i],temp.clone() );
            }
            else  {
                collide.insert(time[i], vec![i as i32] );
            }
        }
        // collide.iter().filter(|k| k.1.len() > 1 ).collect::<Vec<_>>() ;
        // for (k,v) in collide{

        // }
         println!(" collide {:?}",  collide.iter().filter(|k| k.1.len() > 1 ).collect::<Vec<_>>() );

        if time.contains(&1) {
            break;
        }
    }
    // loop {
    //     if cars.is_empty(){ break;}
    //     println!(
    //         " cars {:?}",
    //         (
    //             cars.clone(),
    //             result,
    //             cars.iter().find_map(|(k, _)| Some(&target == k))
    //         )
    //     );
    //     //* if any keys match the target, pop them and increment result
    //     if cars.iter().any(|x|  x.0 == target){
    //         println!(" inc");
    //         result += 1;
    //         // let index = cars.iter().position(|(k, _)| k == &target);
    //         // if index.is_some() {
    //         cars = cars.into_iter().filter(|x| x.0 != target).collect::<Vec<_>>();
    //         // };
    //         // break;
    //     }
    //     //* update all values
    //     cars = cars
    //         .clone()
    //         .into_iter()
    //         .map(|(k, v)| (k + v, v))
    //         .collect::<Vec<_>>();
    // }

    println!(" cars {:?}", (time, cars, result));
    return result;
}
