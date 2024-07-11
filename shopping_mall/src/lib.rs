pub mod mall;
pub use guard::Guard;
pub use mall::floor::store::employee::Employee;
pub use mall::floor::store::*;
pub use mall::floor::*;
pub use mall::*;

pub fn biggest_store(mall: Mall) -> Store {
    mall.floors
        .iter()
        .flat_map(|floor| &floor.stores)
        .max_by_key(|store| store.square_meters)
        .cloned()
        .unwrap()
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let mall_iter = mall
        .floors
        .iter()
        .flat_map(|floor| floor.stores.clone())
        .flat_map(|store| store.employees);

    let highest_slary = mall_iter.clone().fold(f64::MIN, |acc, x| acc.max(x.salary));
    mall_iter.filter(|e| e.salary == highest_slary).collect()
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    mall.guards.len()
        + mall
            .floors
            .iter()
            .flat_map(|floor| &floor.stores)
            .fold(0_usize, |acc, x| acc + x.employees.len())
}

pub fn check_for_securities(mall: &mut Mall, guards: Vec<Guard>) {
    let mut g = guards.clone();
    let s = mall
        .floors
        .iter()
        .flat_map(|floor| &floor.stores)
        .fold(0_u64, |acc, x| acc + x.square_meters);
    println!("**********s = {} *************", s / 200);
    while mall.guards.len() * 200 < s as usize && g.len() > 0 {
        mall.guards.push(g[0].clone());
        g.remove(0);
    }
}
pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.iter_mut() {
        for store in floor.stores.iter_mut() {
            for employee in store.employees.iter_mut() {
                let w_h = employee.working_hours.1 - employee.working_hours.0;
                let percentage = employee.salary * 0.1;
                if w_h > 10 {
                    employee.salary += percentage
                } else {
                    employee.salary -= percentage
                }
            }
        }
    }
}
